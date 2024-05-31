pub(crate) mod configs;
mod health_check;
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

    let port = format!("0.0.0.0:{}", configs.port);

    let mut servers_ports: Vec<String> = Vec::new();
    for servers in &configs.servers {
        servers_ports.push(format!("{}:{}", servers.host, servers.port));
    }

    let listener = tokio::net::TcpListener::bind(&port).await.unwrap();

    let client = Client::builder(TokioExecutor::new()).build_http::<Body>();

    let app_state = AppState {
        addrs: servers_ports,
        req_counter: Arc::new(AtomicUsize::new(0)),
        http_client: client.clone(),
    };

    let app = balancer.with_state(app_state);

    if let Some(health_check) = configs.health_check {
        health_check::run_health_check(health_check, configs.servers, client)
            .await
            .unwrap();
    }

    axum::serve(listener, app).await.unwrap();
}
