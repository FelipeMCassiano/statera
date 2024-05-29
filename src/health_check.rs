use core::str;

use axum::body::Body;
use futures::future::join_all;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use tokio::time::{sleep, Duration};

use crate::configs::{HealthCheck, Server};

async fn health_check(
    server: &Server,
    endpoint: &str,
    http_client: &Client<HttpConnector, Body>,
) -> Result<(), String> {
    let url = format!("http://{}:{}{}", server.host, server.port, endpoint);
    let uri = url.parse().expect("Invalid URL");

    match http_client.get(uri).await {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Server: {} is unhealthy", server.name)),
    }
}

pub async fn run_health_check(
    HealthCheck { interval, endpoint }: HealthCheck,
    servers: Vec<Server>,
    http_client: Client<HttpConnector, Body>,
) -> Result<(), String> {
    let interval = Duration::from_secs(interval);
    loop {
        let mut health_checks = vec![];
        for server in &servers {
            health_checks.push(health_check(server, &endpoint, &http_client));
        }

        let results: Vec<_> = join_all(health_checks).await;

        for result in results {
            result?
        }

        sleep(interval).await;
    }
}
