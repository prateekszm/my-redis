pub mod command;
pub mod store;
pub mod net;

use anyhow::Result;
pub async fn run() -> Result<()> {
    net::run().await
}