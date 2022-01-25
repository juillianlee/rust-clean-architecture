use std::env;

use crate::infra::routes::routes::Result;
use mongodb::{options::ClientOptions, Database};
#[derive(Clone, Debug)]
pub struct DB {
    pub database: Database,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let db_uri =
            env::var("DB_URI").unwrap_or_else(|e| panic!("could not find {}: {}", "DB_URI", e));

        let db_name =
            env::var("DB_NAME").unwrap_or_else(|e| panic!("could not find {}: {}", "DB_NAME", e));

        let mut options = ClientOptions::parse(db_uri).await?;
        options.app_name = Some(db_name.clone());

        let client = mongodb::Client::with_options(options)?;
        let database = client.database(&db_name.as_str());
        Ok(Self { database: database })
    }
}
