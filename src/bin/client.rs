use my_redis::client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    client::run().await
}
