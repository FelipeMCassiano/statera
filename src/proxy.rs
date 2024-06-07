use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc,
    },
};

use axum::{
    extract::{Request, State},
    http::{
        uri::{Authority, Scheme},
        StatusCode, Uri,
    },
    response::{IntoResponse, Response},
};
use reqwest::Client;
#[derive(Clone)]
pub struct AppState {
    pub addrs: Vec<String>,
    pub req_counter: Arc<AtomicUsize>,
    pub http_client: Client,
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
    let targets_addr = &addrs[count % addrs.len()];

    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(targets_addr).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };

    let req = http_client
        .request(req.method().clone(), req.uri().to_string())
        .build()
        .expect("VALID URL");

    match http_client.execute(req).await {
        Ok(res) => Ok({
            let axum_res: Response<reqwest::Body> = res.into();
            axum_res
        }),
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}
