use latte::server;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> latte::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}