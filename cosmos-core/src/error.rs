use std::io;
use thiserror::Error;
use nova::NovaError;

#[derive(Debug, Error)]
pub enum CosmosError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Install script failed: {0}")]
    ScriptFailed(String),

    #[error("Dependency resolution error: {0}")]
    DependencyError(String),

    #[error("Semver parse error: {0}")]
    SemverError(String),

    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Failed to copy files: {0}")]
    CopyFailed(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Checksum validation failed: {0}")]
    ChecksumFailed(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Invalid checksum: {0}")]
    InvalidChecksum(String),
}

impl From<NovaError> for CosmosError {
    fn from(e: NovaError) -> Self {
        CosmosError::ScriptFailed(format!("nova: {:?}", e))
    }
}

