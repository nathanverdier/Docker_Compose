use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub add_date: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
            add_date: chrono::Utc::now(),
        }
    }
}
