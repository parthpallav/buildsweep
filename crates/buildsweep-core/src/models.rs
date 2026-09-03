use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::InactivityThreshold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SafetyClass {
    Safe,
    Review,
    Protected,
    Unknown,
}

impl SafetyClass {
    pub fn is_cleanup_eligible(self) -> bool {
        matches!(self, Self::Safe | Self::Review)
    }

    pub fn default_selected(self) -> bool {
        matches!(self, Self::Safe)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityStatus {
    Active { days_since: u32 },
    Inactive { days_since: u32 },
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ecosystem {
    NodeJs,
    Python,
    Rust,
    DotNet,
    Java,
    Flutter,
    Go,
    Swift,
    Xcode,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    NodeModules,
    NextBuild,
    NuxtBuild,
    Dist,
    Coverage,
    Turbo,
    Venv,
    Pycache,
    PytestCache,
    MypyCache,
    RustTarget,
    GradleCache,
    DotNetBin,
    DotNetObj,
    DartTool,
    FlutterBuild,
    DerivedData,
    Pods,
    GenericBuild,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Artifact {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub safety: SafetyClass,
    pub kind: ArtifactKind,
    pub explanation: String,
    pub shared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasteScoreBreakdown {
    pub total: u8,
    pub inactivity_score: u8,
    pub reclaimable_ratio_score: u8,
    pub reclaimable_size_score: u8,
    pub artifact_score: u8,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub ecosystem: Ecosystem,
    pub activity: ActivityStatus,
    pub total_size_bytes: u64,
    pub reclaimable_size_bytes: u64,
    pub waste_score: WasteScoreBreakdown,
    pub artifacts: Vec<Artifact>,
    pub is_inactive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanPhase {
    Starting,
    DiscoveringProjects,
    DetectingArtifacts,
    CalculatingSizes,
    AnalyzingActivity,
    Complete,
    Cancelled,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanProgress {
    pub scan_id: Uuid,
    pub phase: ScanPhase,
    pub projects_found: u32,
    pub artifacts_found: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanSummary {
    pub project_count: u32,
    pub total_reclaimable_bytes: u64,
    pub inactive_project_count: u32,
    pub largest_waste: Vec<LargestWasteEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LargestWasteEntry {
    pub name: String,
    pub reclaimable_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanResult {
    pub scan_id: Uuid,
    pub roots: Vec<String>,
    pub projects: Vec<Project>,
    pub summary: ScanSummary,
    pub scanned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupSelection {
    pub project_id: Uuid,
    pub artifact_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupPlanItem {
    pub artifact_id: Uuid,
    pub project_name: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub safety: SafetyClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupPlan {
    pub plan_id: Uuid,
    pub items: Vec<CleanupPlanItem>,
    pub total_bytes: u64,
    pub folder_count: u32,
    pub approved_roots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupItemResult {
    pub path: String,
    pub success: bool,
    pub error: Option<String>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupResult {
    pub plan_id: Uuid,
    pub moved_bytes: u64,
    pub moved_count: u32,
    pub failed_count: u32,
    pub items: Vec<CleanupItemResult>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub moved_bytes: u64,
    pub item_count: u32,
}

pub fn compute_scan_summary(projects: &[Project], threshold: InactivityThreshold) -> ScanSummary {
    let threshold_days = threshold.days();
    let mut total_reclaimable = 0u64;
    let mut inactive_count = 0u32;

    for project in projects {
        total_reclaimable += project.reclaimable_size_bytes;
        if project.is_inactive {
            inactive_count += 1;
        }
        let _ = threshold_days;
    }

    let mut largest: Vec<LargestWasteEntry> = projects
        .iter()
        .map(|p| LargestWasteEntry {
            name: p.name.clone(),
            reclaimable_bytes: p.reclaimable_size_bytes,
        })
        .filter(|e| e.reclaimable_bytes > 0)
        .collect();
    largest.sort_by(|a, b| b.reclaimable_bytes.cmp(&a.reclaimable_bytes));
    largest.truncate(3);

    ScanSummary {
        project_count: projects.len() as u32,
        total_reclaimable_bytes: total_reclaimable,
        inactive_project_count: inactive_count,
        largest_waste: largest,
    }
}
