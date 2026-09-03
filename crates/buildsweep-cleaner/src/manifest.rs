use buildsweep_core::CleanupResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub timestamp: DateTime<Utc>,
    pub original_path: String,
    pub artifact_type: String,
    pub size_bytes: u64,
    pub success: bool,
    pub error: Option<String>,
    pub platform: String,
    pub app_version: String,
}

pub fn append_manifest(manifest_path: &Path, result: &CleanupResult, app_version: &str) -> std::io::Result<()> {
    if let Some(parent) = manifest_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(manifest_path)?;

    let platform = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "unknown"
    };

    for item in &result.items {
        let entry = ManifestEntry {
            timestamp: result.completed_at,
            original_path: item.path.clone(),
            artifact_type: "artifact".to_string(),
            size_bytes: item.size_bytes,
            success: item.success,
            error: item.error.clone(),
            platform: platform.to_string(),
            app_version: app_version.to_string(),
        };
        let line = serde_json::to_string(&entry)?;
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
