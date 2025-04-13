use thiserror::Error;
use std::io::Read;

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

    let response = ureq::get(&url).call()
        .map_err(|e| TransportError::DownloadFailed(format!("{}: {}", url, e)))?;

    if response.status() != 200 {
        Err(TransportError::DownloadFailed(format!(
            "Failed to fetch URL: {} (status: {})",
            url, response.status()
        )))?;
    }

    let bytes = response.into_reader().bytes()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TransportError::DownloadFailed(format!("Failed to read response: {}", e)))?;

    Ok(bytes)
}