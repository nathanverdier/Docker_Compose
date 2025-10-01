#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    )
    .expect("unable to init logger");

    return webapp::run().await;
}
