pub(crate) mod configs;
mod health_check;
mod proxy;

use axum::handler::Handler;
use axum_server::tls_rustls::RustlsConfig;
use configs::{find_files_paths, load_config};
use core::sync::atomic::AtomicUsize;
use proxy::{balancer, AppState};
use reqwest::Client;
use std::{path::PathBuf, str::FromStr, sync::Arc};

#[tokio::main]
async fn main() {
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
        Some(ssl) => {
            let Some(cert_path) = find_files_paths(
                &PathBuf::from_str(".").expect("invalid path"),
                ssl.certificate,
            )else{
                todo!()
            };
            let Some(key_path) = find_files_paths(
                &PathBuf::from_str(".").expect("invalid path"),
                ssl.key
            )else{
                todo!()
            };

            Some(
                RustlsConfig::from_pem_file(cert_path, key_path)
                    .await
                    .unwrap(),
            )
        }

        None => None,
    };

    match ssl_config {
        Some(ssl) => {
            axum_server::from_tcp_rustls(listener.into_std().expect("INVALID TCP ADDRESS"), ssl)
                .serve(app.into_make_service())
                .await
                .unwrap()
        }
        None => axum_server::from_tcp(listener.into_std().expect("INVALID TCP ADDRESS"))
            .serve(app.into_make_service())
            .await
            .unwrap(),
    };
}
