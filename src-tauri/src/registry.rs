use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use winreg::enums::*;
use winreg::RegKey;

use crate::emit_log;
use crate::errors::AppError;

/// Safe WHITELIST. Only HKCU branches are ever touched — no HKLM, no
/// CurrentControlSet, no Services.
enum Action {
    /// Remove every value under the key (keep the key itself).
    ClearValues,
    /// Recursively delete the subkey if it exists.
    DeleteSubkeyAll,
}

struct WhitelistEntry {
    subkey: &'static str,
    action: Action,
    label: &'static str,
}

// Every entry here is under HKCU; we never touch HKLM from this whitelist.
static WHITELIST: Lazy<Vec<WhitelistEntry>> = Lazy::new(|| {
    vec![
        WhitelistEntry {
            subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\RunMRU",
            action: Action::ClearValues,
            label: "Explorer RunMRU",
        },
        WhitelistEntry {
            subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\TypedPaths",
            action: Action::ClearValues,
            label: "Explorer TypedPaths",
        },
        WhitelistEntry {
            subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\RecentDocs",
            action: Action::DeleteSubkeyAll,
            label: "Explorer RecentDocs",
        },
    ]
});

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RegistryReport {
    pub removed: Vec<String>,
    pub skipped: Vec<String>,
}

pub fn clean_whitelisted(app: AppHandle) -> Result<RegistryReport, AppError> {
    let mut report = RegistryReport::default();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for entry in WHITELIST.iter() {
        let result = match entry.action {
            Action::ClearValues => clear_values(&hkcu, entry.subkey),
            Action::DeleteSubkeyAll => delete_subkey_all(&hkcu, entry.subkey),
        };
        match result {
            Ok(_) => {
                emit_log(&app, "info", &format!("registry: cleaned {}", entry.label));
                report.removed.push(entry.label.to_string());
            }
            Err(err) => {
                emit_log(
                    &app,
                    "warn",
                    &format!("registry: skipped {} ({err})", entry.label),
                );
                report.skipped.push(format!("{}: {err}", entry.label));
            }
        }
    }

    Ok(report)
}

fn clear_values(root: &RegKey, subkey: &str) -> std::io::Result<()> {
    let key = match root.open_subkey_with_flags(subkey, KEY_READ | KEY_SET_VALUE) {
        Ok(k) => k,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(err),
    };
    let values: Vec<String> = key
        .enum_values()
        .filter_map(|v| v.ok().map(|(n, _)| n))
        .collect();
    for name in values {
        key.delete_value(&name)?;
    }
    Ok(())
}

fn delete_subkey_all(root: &RegKey, subkey: &str) -> std::io::Result<()> {
    match root.delete_subkey_all(subkey) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}
