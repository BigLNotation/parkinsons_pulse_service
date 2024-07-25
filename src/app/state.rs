use dotenvy::dotenv;
use mongodb::{Client, Database};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")?;
        let client = Client::with_uri_str(database_url).await?;
        let db = client.database("capstone");
        Ok(AppState { db: Arc::new(db) })
    }
}
