use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc,
    },
};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{
        uri::{Authority, Scheme},
        StatusCode, Uri,
    },
    response::IntoResponse,
};
use hyper_util::client::legacy::{connect::HttpConnector, Client};

#[derive(Clone)]
pub struct AppState {
    pub addrs: Vec<String>,
    pub req_counter: Arc<AtomicUsize>,
    pub http_client: Client<HttpConnector, Body>,
}

pub async fn balancer(
    State(AppState {
        addrs,
        req_counter,
        http_client,
    }): State<AppState>,
    mut req: Request,
) -> impl IntoResponse {
    let count = req_counter.fetch_add(1, Relaxed);
    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(&addrs[count % addrs.len()]).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };
    match http_client.request(req).await {
        Ok(res) => Ok(res),
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}
