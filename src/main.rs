extern crate dotenv;

mod error;
mod infra;
mod app;
use crate::infra::{routes::{routes, routes::Result}};

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    routes::init().await?;
    Ok(())
}

