use crate::Result;
use mongodb::{options::ClientOptions, Database};

#[derive(Clone, Debug)]
pub struct DB {
    pub database: Database,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut options =
            ClientOptions::parse("mongodb+srv://apphelley:kMSK0984kS9r@cluster0.nzoon.mongodb.net")
                .await?;
        options.app_name = Some("helley".to_string());

        let client = mongodb::Client::with_options(options)?;

        Ok(Self {
            database: client.database("helley"),
        })
    }
}