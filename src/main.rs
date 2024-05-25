use std::str::FromStr;

use axum::{
    body::Body,
    extract::Request,
    handler::HandlerWithoutStateExt,
    http::{
        uri::{Authority, Scheme},
        StatusCode, Uri,
    },
    response::IntoResponse,
};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2207").await.unwrap();

    let app = proxy.into_make_service();

    axum::serve(listener, app).await.unwrap();
}

async fn proxy(mut req: Request) -> impl IntoResponse {
    let client = Client::builder(TokioExecutor::new()).build_http::<Body>();
    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str("0.0.0.0:8080").ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };
    match client.request(req).await {
        Ok(res) => Ok(res),
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}
