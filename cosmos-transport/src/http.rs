use crate::TransportError;
use std::io::Read;

pub fn pull(url: &str) -> Result<Vec<u8>, TransportError> {
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