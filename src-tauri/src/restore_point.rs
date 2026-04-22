use crate::errors::AppError;

/// Creates a System Restore point through PowerShell's `Checkpoint-Computer`.
///
/// ⚠ Behavior may vary:
///   - System Restore must be enabled on the target drive.
///   - Windows enforces a throttle (1 restore point / 24h by default). Hitting
///     the throttle is treated as success, not failure — we still emit a log
///     line but continue cleanup.
pub async fn create(description: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let desc = description.to_string();
        tokio::task::spawn_blocking(move || create_blocking(&desc))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
    }

    #[cfg(not(windows))]
    {
        let _ = description;
        tracing::info!("skip restore point on non-windows target");
        Ok(())
    }
}

#[cfg(windows)]
fn create_blocking(description: &str) -> Result<(), AppError> {
    use std::process::Command;

    let script = format!(
        "try {{ Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop }} catch {{ if ($_.Exception.Message -match '24 hours') {{ exit 0 }} else {{ throw }} }}",
        description.replace('\'', "''")
    );

    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .output()
        .map_err(AppError::Io)?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AppError::RestorePointFailed(stderr.trim().into()))
    }
}
