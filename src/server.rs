use std::sync::{atomic::AtomicUsize, Arc};

use axum::handler::Handler;
use reqwest::Client;

use crate::{
    configs::load_config,
    health_check,
    proxy::{balancer, AppState},
    ssl::configure_ssl,
};

pub async fn start_server() {
    let configs = load_config().await;

    let port = format!("0.0.0.0:{}", configs.port);

    let servers_ports = configs
        .servers
        .iter()
        .map(|s| format!("{}:{}", s.host, s.port))
        .collect();

    let listener = tokio::net::TcpListener::bind(&port).await.unwrap();

    let client = Client::new();

    let app_state = AppState {
        addrs: servers_ports,
        req_counter: Arc::new(AtomicUsize::new(0)),
        http_client: client.clone(),
    };

    let app = balancer.with_state(app_state);

    if let Some(health_check) = configs.health_check {
        if let Err(e) = health_check::run_health_check(health_check, configs.servers, client).await
        {
            eprintln!("{}", e);
            return;
        }
    }

    let ssl_config = match configs.ssl {
        Some(ssl) => configure_ssl(ssl).await,
        None => None,
    };

    match ssl_config {
        Some(ssl) => {
            axum_server::from_tcp_rustls(listener.into_std().expect("Invalid TCP address"), ssl)
                .serve(app.into_make_service())
                .await
                .expect("Server failed");
        }
        None => {
            axum_server::from_tcp(listener.into_std().expect("Invalid TCP address"))
                .serve(app.into_make_service())
                .await
                .expect("Server failed");
        }
    }
}
