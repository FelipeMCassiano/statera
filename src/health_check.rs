use core::str;

use axum::body::Body;
use futures::{stream::FuturesUnordered, StreamExt};
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use tokio::time::{sleep, Duration};

use crate::configs::{HealthCheck, Server};

async fn health_check(
    server: &Server,
    endpoint: &str,
    http_client: &Client<HttpConnector, Body>,
) -> Result<(), String> {
    let url = format!("http://{}:{}{}", server.host, server.port, endpoint);
    let uri = url
        .parse()
        .map_err(|_| format!("Invalid URL for server: {}", server.name))?;

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
        let mut health_checks = FuturesUnordered::new();
        for server in &servers {
            health_checks.push(health_check(server, &endpoint, &http_client));
        }

        while let Some(result) = health_checks.next().await {
            result?
        }

        sleep(interval).await;
    }
}
