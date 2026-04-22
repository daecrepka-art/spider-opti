mod errors;
mod system_info;
mod scanner;
mod cleaner;
mod restore_point;

#[cfg(windows)]
mod services;
#[cfg(windows)]
mod registry;

#[cfg(not(windows))]
mod services {
    use crate::errors::AppError;
    use serde::{Deserialize, Serialize};

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

    pub fn toggle(_name: &str, _enable: bool) -> Result<ServiceState, AppError> {
        Err(AppError::Unsupported("services API only available on Windows"))
    }

    pub fn describe_many(_names: &[String]) -> Result<Vec<ServiceInfo>, AppError> {
        Ok(Vec::new())
    }
}

#[cfg(not(windows))]
mod registry {
    use crate::errors::AppError;
    use serde::{Deserialize, Serialize};
    use tauri::AppHandle;

    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct RegistryReport {
        pub removed: Vec<String>,
        pub skipped: Vec<String>,
    }

    pub fn clean_whitelisted(_app: AppHandle) -> Result<RegistryReport, AppError> {
        Err(AppError::Unsupported("registry cleaning only available on Windows"))
    }
}

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tracing::{info, warn};

use crate::errors::AppError;

/// Real list of services safe to disable on Windows 10 / 11 (Home / Pro).
///
/// // ⚠ Behavior may vary on LTSC / Server SKUs — test required.
pub const DESTRUCTIVE_SERVICES: &[&str] = &[
    "DiagTrack",
    "WSearch",
    "SysMain",
    "MapsBroker",
    "RetailDemo",
    "WMPNetworkSvc",
    "XblAuthManager",
    "XblGameSave",
    "XboxNetApiSvc",
    "DiagnosticHub.StandardCollector.Service",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleanTarget {
    pub id: String,
    pub path: PathBuf,
}

// =================== Tauri commands ===================

#[tauri::command]
async fn get_system_info() -> Result<system_info::SystemInfo, AppError> {
    tokio::task::spawn_blocking(system_info::collect)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
async fn scan_junk_files(app: AppHandle) -> Result<Vec<scanner::JunkItem>, AppError> {
    emit_log(&app, "info", "🕷  Паук начал обход файловой системы...");
    let items = tokio::task::spawn_blocking(scanner::scan_all)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;
    emit_log(
        &app,
        "info",
        &format!("Найдено {} объектов-мусора", items.len()),
    );
    Ok(items)
}

#[tauri::command]
async fn clean_selected(
    app: AppHandle,
    items: Vec<CleanTarget>,
) -> Result<cleaner::CleanReport, AppError> {
    emit_log(&app, "info", "Создаю точку восстановления перед очисткой...");
    restore_point::create("Spider-Opti: before file cleanup").await?;
    emit_log(&app, "info", "Точка восстановления создана ✓");

    let app_for_log = app.clone();
    let report = tokio::task::spawn_blocking(move || cleaner::clean(items, app_for_log))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;

    emit_log(
        &app,
        "info",
        &format!(
            "Очистка завершена: освобождено {} байт, ошибок {}",
            report.freed_bytes,
            report.failed.len()
        ),
    );
    Ok(report)
}

#[tauri::command]
async fn toggle_service(
    app: AppHandle,
    name: String,
    enable: bool,
) -> Result<services::ServiceState, AppError> {
    emit_log(
        &app,
        "info",
        &format!(
            "Переключение службы {name}: {}",
            if enable { "enable" } else { "disable" }
        ),
    );
    let name_for_task = name.clone();
    let state = tokio::task::spawn_blocking(move || services::toggle(&name_for_task, enable))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;
    Ok(state)
}

#[tauri::command]
async fn clean_registry(app: AppHandle) -> Result<registry::RegistryReport, AppError> {
    emit_log(&app, "info", "Создаю точку восстановления перед чисткой реестра...");
    restore_point::create("Spider-Opti: before registry cleanup").await?;

    let app_for_log = app.clone();
    let report = tokio::task::spawn_blocking(move || registry::clean_whitelisted(app_for_log))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;
    emit_log(
        &app,
        "info",
        &format!(
            "Реестр: удалено {}, пропущено {}",
            report.removed.len(),
            report.skipped.len()
        ),
    );
    Ok(report)
}

#[tauri::command]
async fn list_destructive_services() -> Result<Vec<services::ServiceInfo>, AppError> {
    let names: Vec<String> = DESTRUCTIVE_SERVICES.iter().map(|s| (*s).to_string()).collect();
    tokio::task::spawn_blocking(move || services::describe_many(&names))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
async fn create_restore_point(description: String) -> Result<(), AppError> {
    restore_point::create(&description).await
}

// =================== Events / helpers ===================

pub(crate) fn emit_log(app: &AppHandle, level: &str, msg: &str) {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let payload = serde_json::json!({
        "level": level,
        "msg": msg,
        "ts": ts,
    });
    if let Err(e) = app.emit("spider://log", payload) {
        warn!("failed to emit spider://log: {e}");
    }
}

// =================== Entry ===================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    info!("Spider-Opti starting");

    if !is_user_admin() {
        tracing::error!(
            "Spider-Opti was not launched as Administrator — privileged commands will fail."
        );
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_logger())
        .setup(|app| {
            let handle = app.handle().clone();
            emit_log(&handle, "info", "Spider-Opti активирован 🕸");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            scan_junk_files,
            clean_selected,
            toggle_service,
            clean_registry,
            list_destructive_services,
            create_restore_point,
        ])
        .run(tauri::generate_context!())
        .expect("Tauri runtime failed");
}

/// Tiny no-op plugin placeholder kept so we have a single place to attach a
/// real logger plugin later (e.g. `tauri-plugin-log`) without touching `run()`.
fn tauri_plugin_logger() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri::plugin::Builder::new("spider-logger").build()
}

#[cfg(windows)]
fn is_user_admin() -> bool {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }
        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        )
        .is_ok();
        ok && elevation.TokenIsElevated != 0
    }
}

#[cfg(not(windows))]
fn is_user_admin() -> bool {
    true
}
