use serde::{Serialize, Serializer};
use thiserror::Error;

/// Unified error type for every Tauri command.
///
/// The custom `Serialize` impl turns any variant into a plain string on the
/// wire so the React side can just `catch (e) { ... }` without parsing a
/// discriminated union.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Failed to create a system restore point: {0}")]
    RestorePointFailed(String),

    #[error("Registry error: {0}")]
    Registry(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Scan error: {0}")]
    Scan(String),

    #[error("Unsupported on this platform: {0}")]
    Unsupported(&'static str),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}
