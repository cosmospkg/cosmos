mod http;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Transport error: {0}")]
    TransportError(String),
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Unsupported URL scheme: {0}")]
    UnsupportedUrlScheme(String),
}

/// Returns true if the URL is supported by the enabled transport features.
pub fn supports_url(url: &str) -> bool {
    if url.starts_with("http://") {
        cfg!(feature = "http")
    } else if url.starts_with("https://") {
        cfg!(feature = "https")
    } else {
        false
    }
}


pub fn fetch_bytes(url: &str) -> Result<Vec<u8>, TransportError> {
    if !supports_url(url) {
        return Err(TransportError::UnsupportedUrlScheme(url.to_string()));
    }

    let protocol = url.split("://").next().unwrap_or("");
    let bytes = match protocol {
        "http" => http::pull(url),
        "https" => http::pull(url),
        _ => return Err(TransportError::UnsupportedUrlScheme(url.to_string())),
    }?;

    Ok(bytes)
}