use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tracing::warn;

use crate::errors::AppError;
use crate::emit_log;
use crate::CleanTarget;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FailedItem {
    pub path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CleanReport {
    pub freed_bytes: u64,
    pub failed: Vec<FailedItem>,
}

/// Deletes every target file that is inside an allowed root.
///
/// Invoked from `spawn_blocking`; must not panic.
pub fn clean(items: Vec<CleanTarget>, app: AppHandle) -> Result<CleanReport, AppError> {
    let allowed = safe_roots();

    let mut report = CleanReport::default();

    for target in items {
        let path = target.path.clone();

        if !allowed.iter().any(|root| path.starts_with(root)) {
            warn!(?path, "refused to delete path outside allowed roots");
            report.failed.push(FailedItem {
                path,
                reason: "outside safe roots".into(),
            });
            continue;
        }

        let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        match fs::remove_file(&path) {
            Ok(_) => {
                report.freed_bytes += size;
                emit_log(
                    &app,
                    "info",
                    &format!("removed {}", path.display()),
                );
            }
            Err(err) => {
                emit_log(
                    &app,
                    "warn",
                    &format!("failed to remove {}: {err}", path.display()),
                );
                report.failed.push(FailedItem {
                    path,
                    reason: err.to_string(),
                });
            }
        }
    }

    Ok(report)
}

fn safe_roots() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::new();
    for var in ["TEMP", "TMP", "LOCALAPPDATA"] {
        if let Some(v) = std::env::var_os(var) {
            roots.push(PathBuf::from(v));
        }
    }
    if let Some(windir) = std::env::var_os("SystemRoot") {
        let base = PathBuf::from(windir);
        roots.push(base.join("Temp"));
        roots.push(base.join("Prefetch"));
    }
    roots
}
