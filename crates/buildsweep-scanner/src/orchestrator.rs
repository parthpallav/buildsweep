use buildsweep_core::{
    compute_scan_summary, is_inactive, Artifact, Ecosystem, Project, ScanPhase, ScanProgress,
    ScanResult, Settings,
};
use buildsweep_core::{compute_waste_score, ActivityStatus};
use buildsweep_detectors::{detect_artifacts, ProjectContext};
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::activity::detect_activity;
use crate::discover::{discover_projects, DiscoveredProject};
use crate::size::{directory_size, project_total_size};

pub struct ScanOptions {
    pub roots: Vec<PathBuf>,
    pub settings: Settings,
}

pub fn run_scan_blocking(
    scan_id: Uuid,
    options: ScanOptions,
    progress_tx: mpsc::UnboundedSender<ScanProgress>,
    partial_snapshot: Option<std::sync::Arc<std::sync::Mutex<Option<ScanResult>>>>,
) -> Result<ScanResult, buildsweep_core::BuildSweepError> {
    let send_progress = |phase: ScanPhase, projects_found: u32, artifacts_found: u32, message: &str| {
        let _ = progress_tx.send(ScanProgress {
            scan_id,
            phase,
            projects_found,
            artifacts_found,
            message: message.to_string(),
        });
    };

    send_progress(ScanPhase::Starting, 0, 0, "Starting scan...");

    send_progress(ScanPhase::DiscoveringProjects, 0, 0, "Discovering projects...");
    let discovered = discover_projects(&options.roots, &options.settings.exclusions);

    send_progress(
        ScanPhase::DiscoveringProjects,
        discovered.len() as u32,
        0,
        &format!("Found {} projects. Calculating sizes...", discovered.len()),
    );

    let mut projects = Vec::new();
    let mut total_artifacts = 0u32;

    for (i, disc) in discovered.iter().enumerate() {
        send_progress(
            ScanPhase::DetectingArtifacts,
            (i + 1) as u32,
            total_artifacts,
            &format!("Scanning {} ({}/{})...", disc.name, i + 1, discovered.len()),
        );

        if let Some(project) = scan_project(disc, &options.settings) {
            total_artifacts += project.artifacts.len() as u32;
            projects.push(project);

            if let Some(snapshot) = &partial_snapshot {
                let partial = ScanResult {
                    scan_id,
                    roots: options
                        .roots
                        .iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect(),
                    projects: projects.clone(),
                    summary: compute_scan_summary(&projects, options.settings.inactivity_threshold),
                    scanned_at: Utc::now(),
                };
                if let Ok(mut guard) = snapshot.lock() {
                    *guard = Some(partial);
                }
            }
        }
    }

    send_progress(
        ScanPhase::Complete,
        projects.len() as u32,
        total_artifacts,
        "Scan complete",
    );

    let summary = compute_scan_summary(&projects, options.settings.inactivity_threshold);

    Ok(ScanResult {
        scan_id,
        roots: options
            .roots
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
        projects,
        summary,
        scanned_at: Utc::now(),
    })
}

pub async fn run_scan(
    scan_id: Uuid,
    options: ScanOptions,
    cancel: CancellationToken,
    progress_tx: mpsc::UnboundedSender<ScanProgress>,
) -> Result<ScanResult, buildsweep_core::BuildSweepError> {
    let send_progress = |phase: ScanPhase, projects_found: u32, artifacts_found: u32, message: &str| {
        let _ = progress_tx.send(ScanProgress {
            scan_id,
            phase,
            projects_found,
            artifacts_found,
            message: message.to_string(),
        });
    };

    send_progress(ScanPhase::Starting, 0, 0, "Starting scan...");

    if cancel.is_cancelled() {
        return Err(buildsweep_core::BuildSweepError::ScanCancelled);
    }

    send_progress(ScanPhase::DiscoveringProjects, 0, 0, "Discovering projects...");
    let discovered = discover_projects(&options.roots, &options.settings.exclusions);

    if cancel.is_cancelled() {
        return Err(buildsweep_core::BuildSweepError::ScanCancelled);
    }

    let mut projects = Vec::new();
    let mut total_artifacts = 0u32;

    for (i, disc) in discovered.iter().enumerate() {
        if cancel.is_cancelled() {
            return Err(buildsweep_core::BuildSweepError::ScanCancelled);
        }

        send_progress(
            ScanPhase::DetectingArtifacts,
            (i + 1) as u32,
            total_artifacts,
            &format!("Scanning {}...", disc.name),
        );

        if let Some(project) = scan_project(disc, &options.settings) {
            total_artifacts += project.artifacts.len() as u32;
            projects.push(project);
        }
    }

    send_progress(
        ScanPhase::Complete,
        projects.len() as u32,
        total_artifacts,
        "Scan complete",
    );

    let summary = compute_scan_summary(&projects, options.settings.inactivity_threshold);

    Ok(ScanResult {
        scan_id,
        roots: options.roots.iter().map(|p| p.to_string_lossy().to_string()).collect(),
        projects,
        summary,
        scanned_at: Utc::now(),
    })
}

fn scan_project(disc: &DiscoveredProject, settings: &Settings) -> Option<Project> {
    let ctx = ProjectContext::from_root(&disc.root);
    let detected = detect_artifacts(&ctx);

    let artifact_paths: Vec<PathBuf> = detected
        .iter()
        .map(|d| disc.root.join(&d.relative_path))
        .collect();

    let artifacts: Vec<Artifact> = detected
        .into_iter()
        .map(|d| {
            let full_path = disc.root.join(&d.relative_path);
            let size = directory_size(&full_path);
            let shared = ctx.is_workspace_root
                && !full_path.starts_with(&disc.root);
            Artifact {
                id: Uuid::new_v4(),
                name: d.name,
                path: full_path.to_string_lossy().to_string(),
                size_bytes: size,
                safety: d.safety,
                kind: d.kind,
                explanation: d.explanation,
                shared,
            }
        })
        .filter(|a| !a.shared)
        .collect();

    let reclaimable: u64 = artifacts
        .iter()
        .filter(|a| a.safety.is_cleanup_eligible())
        .map(|a| a.size_bytes)
        .sum();

    let total = project_total_size(&disc.root, &artifact_paths);
    let activity = detect_activity(&disc.root);
    let inactive = is_inactive(&activity, settings.inactivity_threshold);
    let waste_score = compute_waste_score(
        &activity,
        total,
        reclaimable,
        &artifacts,
        settings.inactivity_threshold,
    );

    Some(Project {
        id: Uuid::new_v4(),
        name: disc.name.clone(),
        path: disc.root.to_string_lossy().to_string(),
        ecosystem: ctx.ecosystem,
        activity,
        total_size_bytes: total,
        reclaimable_size_bytes: reclaimable,
        waste_score,
        artifacts,
        is_inactive: inactive,
    })
}

pub struct ScanHandle {
    pub cancel: CancellationToken,
    pub result: tokio::task::JoinHandle<Result<ScanResult, buildsweep_core::BuildSweepError>>,
}

pub fn spawn_scan(
    options: ScanOptions,
    progress_tx: mpsc::UnboundedSender<ScanProgress>,
) -> (Uuid, ScanHandle) {
    let scan_id = Uuid::new_v4();
    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let handle = tokio::spawn(async move {
        run_scan(scan_id, options, cancel_clone, progress_tx).await
    });

    (scan_id, ScanHandle { cancel, result: handle })
}

pub type SharedScanResult = Arc<ScanResult>;
