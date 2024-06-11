use crate::server::start_server;

pub(crate) mod configs;
mod health_check;
mod proxy;
mod server;
mod ssl;

#[tokio::main]
async fn main() {
    start_server().await
}
