use std::{path::PathBuf, str::FromStr};

use axum_server::tls_rustls::RustlsConfig;
use configs::{find_files_paths, Ssl};

use crate::configs;

pub async fn configure_ssl(ssl: Ssl) -> Option<RustlsConfig> {
    let cert_path = find_file_path(".", ssl.certificate)?;
    let key_path = find_file_path(".", ssl.key)?;

    Some(
        RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .expect("Failed to load SSL certificates"),
    )
}
fn find_file_path(base: &str, filename: String) -> Option<PathBuf> {
    find_files_paths(&PathBuf::from_str(base).expect("invalid path"), filename)
}
