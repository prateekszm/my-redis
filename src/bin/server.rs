use my_redis::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run().await
}
