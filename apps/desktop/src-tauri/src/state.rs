use std::sync::Arc;
use buildsweep_core::{CleanupPlan, ScanResult, Settings};
use buildsweep_license::LicenseStatus;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct ActiveScan {
    pub scan_id: Uuid,
    pub cancel: CancellationToken,
    pub result: Option<ScanResult>,
}

pub struct AppState {
    pub data_dir: PathBuf,
    pub settings: Mutex<Settings>,
    pub active_scan: Mutex<Option<ActiveScan>>,
    pub last_scan: Mutex<Option<ScanResult>>,
    pub plans: Mutex<HashMap<Uuid, CleanupPlan>>,
    pub stored_license: Mutex<Option<String>>,
    pub license_status: Mutex<LicenseStatus>,
}

// Re-export for sharing across async tasks
pub type SharedAppState = Arc<AppState>;

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let settings_path = data_dir.join("settings.json");
        let settings = if settings_path.exists() {
            std::fs::read_to_string(&settings_path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Settings::default()
        };

        let license_path = data_dir.join("pro.lic");
        let stored_license = license_path
            .exists()
            .then(|| std::fs::read_to_string(&license_path).ok())
            .flatten();

        let license_status = buildsweep_license::resolve_status(stored_license.as_deref());

        Self {
            data_dir,
            settings: Mutex::new(settings),
            active_scan: Mutex::new(None),
            last_scan: Mutex::new(None),
            plans: Mutex::new(HashMap::new()),
            stored_license: Mutex::new(stored_license),
            license_status: Mutex::new(license_status),
        }
    }

    pub fn settings_path(&self) -> PathBuf {
        self.data_dir.join("settings.json")
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.data_dir.join("cleanup-manifest.jsonl")
    }

    pub fn history_path(&self) -> PathBuf {
        self.data_dir.join("cleanup-history.jsonl")
    }

    pub fn license_path(&self) -> PathBuf {
        self.data_dir.join("pro.lic")
    }
}

pub type ProgressSender = mpsc::UnboundedSender<buildsweep_core::ScanProgress>;
