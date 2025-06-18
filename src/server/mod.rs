pub mod command;
pub mod net;
pub mod store;

use anyhow::Result;
pub async fn run() -> Result<()> {
    net::run().await
}
