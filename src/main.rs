pub(crate) mod configs;
mod proxy;

use axum::body::Body;
use axum::handler::Handler;
use configs::load_config;
use core::sync::atomic::AtomicUsize;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use proxy::{balancer, AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let configs = load_config().await;

    let port = format!("0.0.0.0:{}", configs.statera.port);

    let server_ports: Vec<String> = {
        configs
            .servers
            .ports
            .to_vec()
            .iter()
            .map(|p| format!("0.0.0.0:{}", p))
            .collect()
    };
    let listener = tokio::net::TcpListener::bind(&port).await.unwrap();

    let client = Client::builder(TokioExecutor::new()).build_http::<Body>();
    let app_state = AppState {
        addrs: server_ports.clone(),
        req_counter: Arc::new(AtomicUsize::new(0)),
        http_client: client,
    };

    let app = balancer.with_state(app_state);

    axum::serve(listener, app).await.unwrap();
}
