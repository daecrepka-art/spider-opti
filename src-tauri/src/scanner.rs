use std::path::{Path, PathBuf};
use std::time::SystemTime;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JunkCategory {
    Temp,
    Prefetch,
    ThumbCache,
    InetCache,
    WindowsTemp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JunkItem {
    pub id: String,
    pub path: PathBuf,
    pub size: u64,
    pub category: JunkCategory,
}

/// Scan every allowed root in parallel and return a flat list of junk items.
///
/// Must be invoked inside `tokio::task::spawn_blocking` — `rayon` is a
/// blocking API.
pub fn scan_all() -> Result<Vec<JunkItem>, AppError> {
    let roots = allowed_roots();

    let items: Vec<JunkItem> = roots
        .par_iter()
        .flat_map_iter(|(root, category)| scan_root(root, *category))
        .collect();

    Ok(items)
}

fn scan_root(root: &Path, category: JunkCategory) -> Vec<JunkItem> {
    if !root.exists() {
        return Vec::new();
    }
    WalkDir::new(root)
        .follow_links(false)
        .max_depth(32)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| passes_filter(e.path(), category))
        .filter_map(|e| {
            let size = e.metadata().ok()?.len();
            Some(JunkItem {
                id: format!("{:x}", blake_like_hash(e.path())),
                path: e.path().to_path_buf(),
                size,
                category,
            })
        })
        .collect()
}

fn passes_filter(path: &Path, category: JunkCategory) -> bool {
    match category {
        JunkCategory::Prefetch => path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("pf"))
            .unwrap_or(false)
            && older_than_days(path, 7),
        _ => true,
    }
}

fn older_than_days(path: &Path, days: u64) -> bool {
    let Ok(meta) = path.metadata() else {
        return false;
    };
    let Ok(modified) = meta.modified() else {
        return false;
    };
    let Ok(elapsed) = SystemTime::now().duration_since(modified) else {
        return false;
    };
    elapsed.as_secs() > days * 86_400
}

fn allowed_roots() -> Vec<(PathBuf, JunkCategory)> {
    let mut roots: Vec<(PathBuf, JunkCategory)> = Vec::new();

    if let Some(temp) = std::env::var_os("TEMP") {
        roots.push((PathBuf::from(temp), JunkCategory::Temp));
    }
    if let Some(localappdata) = std::env::var_os("LOCALAPPDATA") {
        let base = PathBuf::from(localappdata);
        roots.push((
            base.join("Microsoft/Windows/INetCache"),
            JunkCategory::InetCache,
        ));
        roots.push((
            base.join("Microsoft/Windows/Explorer"),
            JunkCategory::ThumbCache,
        ));
    }
    if let Some(windir) = std::env::var_os("SystemRoot") {
        let base = PathBuf::from(windir);
        roots.push((base.join("Temp"), JunkCategory::WindowsTemp));
        roots.push((base.join("Prefetch"), JunkCategory::Prefetch));
    }

    roots
}

/// Tiny deterministic hash used as a stable item id; no crypto dependency.
fn blake_like_hash(path: &Path) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h = DefaultHasher::new();
    path.hash(&mut h);
    h.finish()
}
