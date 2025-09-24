use actix_web::{get, middleware::NormalizePath, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use model::User;
use mongodb::{
    bson::doc,
    options::{ClientOptions, Credential, ServerAddress},
    Client,
};
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    time::{Duration, Instant},
};

mod model;

const DB_NAME: &str = "ESIEApp";
const COLL_NAME: &str = "users";
const LOG_FILE_PATH: &str = "/var/log/webapp/webapp.log";

struct AppState {
    hostname: String,
    start_instant: Instant,
    db_client: Client,
}

impl AppState {
    fn new(db_client: Client) -> Self {
        Self {
            start_instant: Instant::now(),
            db_client,
            hostname: hostname::get()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or("no hostname".into()),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct AddUserParams {
    first_name: String,
    last_name: String,
}

#[get("/adduser")]
async fn add_user(ctx: web::Data<AppState>, query: web::Query<AddUserParams>) -> impl Responder {
    log::debug!("Request: /adduser");

    // Add to log file **PANICs if directory does not exist**
    let mut log_file = std::fs::File::create(LOG_FILE_PATH).unwrap_or_else(|e| {
        log::error!("unable to create {LOG_FILE_PATH} ({e}), exiting");
        panic!();
    });
    log_file
        .write_all(&format!("{} {} added", query.first_name, query.last_name).into_bytes())
        .unwrap();

    // Add to db collection
    let collection = ctx
        .db_client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME);
    let user = User::new(query.first_name.clone(), query.last_name.clone());
    match collection.insert_one(user).await {
        Ok(_) => HttpResponse::Ok().body(format!(
            "User \"{} {}\" added to the database",
            query.first_name, query.last_name
        )),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Unable to add user to database: {e}"))
        }
    }
}

#[get("/")]
async fn stats(ctx: web::Data<AppState>) -> impl Responder {
    log::debug!("Request: /stats");
    let mut body = String::from("<!DOCTYPE html>\n<html>\n<body>");

    // Uptime
    let now = Instant::now();
    let diff_secs = now.duration_since(ctx.start_instant).as_secs();

    // User list
    body.push_str(&format!(
        "<p>Pod {} started {diff_secs}s ago\n\n</p>",
        ctx.hostname
    ));
    let mut user_list = String::from("<p>Currently registered users:\n</p>");
    let mut number_of_users = 0;

    let collection = ctx
        .db_client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME);

    log::debug!("Finding all registered users");
    match collection.find(doc! {}).await {
        Ok(mut user_docs) => {
            user_list.push_str("<ul>\n");
            while user_docs.advance().await.is_ok_and(|b| b) {
                let Ok(user) = user_docs.deserialize_current() else {
                    continue;
                };
                number_of_users += 1;
                user_list.push_str(&format!(
                    "<li>{} {} (added on {})</li>\n",
                    user.first_name, user.last_name, user.add_date
                ));
            }
            user_list.push_str("</ul>\n");
        }
        Err(_) => user_list = String::from("No users yet"),
    };

    if number_of_users == 0 {
        user_list = String::from("No users yet");
    }
    body.push_str(&user_list);

    // Add user form
    body.push_str(
        r#"
<body>
    <form id="form" method="GET" action="adduser">
        <input id="first_name" type="text" name="first_name", value = "first name">
        <input id="last_name" type="text" name="last_name", value = "last name">
        <input id="search-btn" type="submit" value="Add user">
    </form>
</body>
        "#,
    );

    body.push_str("\n</body>\n</html>");

    HttpResponse::Ok().body(body)
}

pub async fn run() -> anyhow::Result<()> {
    log::info!("Warming up!");
    let host = std::env::var("MONGODB_HOST").with_context(|| "missing MONGODB_HOST env var")?;
    let port: Option<u16> = std::env::var("MONGODB_PORT")
        .ok()
        .and_then(|p| p.parse().ok());
    let username = std::env::var("MONGODB_USER").with_context(|| "missing MONGODB_USER env var")?;
    let password =
        std::env::var("MONGODB_PASSWORD").with_context(|| "missing MONGODB_PASSWORD env var")?;
    tokio::time::sleep(Duration::from_secs(15)).await;

    log::info!("Initializing MongoDB connection");
    let creds = Credential::builder()
        .username(username)
        .password(password)
        .source(Some("admin".into()))
        .build();
    let client = Client::with_options(
        ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp { host, port }])
            .credential(creds)
            .build(),
    )
    .with_context(|| "failed to initialize MongoDB client")?;
    let session = client
        .start_session()
        .await
        .with_context(|| "unable to start session with MongoDB")?;
    let client = session.client();

    log::info!("Ready");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(client.clone())))
            .wrap(NormalizePath::trim())
            .service(stats)
            .service(add_user)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
