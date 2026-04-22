use std::process::Command;

use serde::{Deserialize, Serialize};
use windows_service::service::{ServiceAccess, ServiceState as WsServiceState};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

use crate::errors::AppError;

const ERROR_ACCESS_DENIED: i32 = 5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceState {
    pub name: String,
    pub start_type: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub start_type: String,
    pub status: String,
}

/// Toggles a service: `enable=false` → Disabled + Stopped; `enable=true` →
/// Automatic + Running (no-op if already matching).
///
/// We use `sc.exe config` to change the start type because `windows-service`'s
/// `change_config` requires a full `ServiceInfo` and would clobber fields like
/// the service account or load-order group. `sc.exe` passes SERVICE_NO_CHANGE
/// for every field we don't mention, which is exactly what we want.
pub fn toggle(name: &str, enable: bool) -> Result<ServiceState, AppError> {
    let new_start_keyword = if enable { "auto" } else { "disabled" };

    // ---- change start type via sc.exe ----
    let output = Command::new("sc.exe")
        .args(["config", name, "start=", new_start_keyword])
        .output()
        .map_err(AppError::Io)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let combined = format!("{stderr} {stdout}");
        if combined.contains("Access is denied") {
            return Err(AppError::AccessDenied(
                "service operation requires Administrator privileges".into(),
            ));
        }
        return Err(AppError::Service(combined));
    }

    // ---- transition state via windows-service (has nicer error types) ----
    let manager = ServiceManager::local_computer(
        None::<&str>,
        ServiceManagerAccess::CONNECT,
    )
    .map_err(translate)?;

    let access = ServiceAccess::QUERY_STATUS | ServiceAccess::START | ServiceAccess::STOP;
    let service = manager.open_service(name, access).map_err(translate)?;

    let status = service.query_status().map_err(translate)?;
    let desired_state = if enable {
        WsServiceState::Running
    } else {
        WsServiceState::Stopped
    };

    if status.current_state != desired_state {
        let r = if enable {
            service.start::<&str>(&[]).map(|_| ())
        } else {
            service.stop().map(|_| ())
        };
        if let Err(err) = r {
            tracing::warn!(?err, service = name, "transition failed");
        }
    }

    let final_status = service.query_status().map_err(translate)?;

    Ok(ServiceState {
        name: name.to_string(),
        start_type: new_start_keyword.to_string(),
        status: format!("{:?}", final_status.current_state),
    })
}

/// Describes each requested service without mutating state.
pub fn describe_many(names: &[String]) -> Result<Vec<ServiceInfo>, AppError> {
    let manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .map_err(translate)?;

    let mut out = Vec::with_capacity(names.len());

    for name in names {
        let access = ServiceAccess::QUERY_CONFIG | ServiceAccess::QUERY_STATUS;
        match manager.open_service(name, access) {
            Ok(svc) => {
                let cfg = svc.query_config().ok();
                let status = svc.query_status().ok();
                out.push(ServiceInfo {
                    name: name.clone(),
                    display_name: cfg
                        .as_ref()
                        .map(|c| c.display_name.to_string_lossy().into_owned())
                        .unwrap_or_default(),
                    start_type: cfg
                        .as_ref()
                        .map(|c| format!("{:?}", c.start_type))
                        .unwrap_or_else(|| "Unknown".into()),
                    status: status
                        .map(|s| format!("{:?}", s.current_state))
                        .unwrap_or_else(|| "Unknown".into()),
                });
            }
            Err(_) => {
                out.push(ServiceInfo {
                    name: name.clone(),
                    display_name: String::new(),
                    start_type: "NotPresent".into(),
                    status: "NotPresent".into(),
                });
            }
        }
    }

    Ok(out)
}

fn translate(err: windows_service::Error) -> AppError {
    if let windows_service::Error::Winapi(ref io) = err {
        if io.raw_os_error() == Some(ERROR_ACCESS_DENIED) {
            return AppError::AccessDenied(
                "service operation requires Administrator privileges".into(),
            );
        }
    }
    AppError::Service(err.to_string())
}
