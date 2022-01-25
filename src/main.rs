mod error;
mod infra;
mod app;
use crate::infra::{routes::{routes, routes::Result}};

#[tokio::main]
async fn main() -> Result<()> {
    routes::init().await?;
    Ok(())
}

