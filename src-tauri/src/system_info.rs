use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub ram_total_bytes: u64,
    pub ram_used_bytes: u64,
    pub disks: Vec<DiskInfo>,
    pub os_name: String,
    pub os_version: String,
}

pub fn collect() -> Result<SystemInfo, AppError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_usage();
    let ram_total_bytes = sys.total_memory();
    let ram_used_bytes = sys.used_memory();

    let disks: Vec<DiskInfo> = Disks::new_with_refreshed_list()
        .list()
        .iter()
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().into_owned(),
            mount_point: d.mount_point().to_string_lossy().into_owned(),
            total_bytes: d.total_space(),
            available_bytes: d.available_space(),
        })
        .collect();

    Ok(SystemInfo {
        cpu_usage,
        ram_total_bytes,
        ram_used_bytes,
        disks,
        os_name: System::name().unwrap_or_else(|| "unknown".into()),
        os_version: System::os_version().unwrap_or_else(|| "unknown".into()),
    })
}
