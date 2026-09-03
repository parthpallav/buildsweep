use buildsweep_core::ActivityStatus;
use chrono::{DateTime, Duration, Utc};
use git2::Repository;
use std::path::Path;

const SOURCE_EXTENSIONS: &[&str] = &["rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "cs", "swift", "kt"];

const MANIFEST_FILES: &[&str] = &[
    "package.json",
    "Cargo.toml",
    "pyproject.toml",
    "requirements.txt",
    "go.mod",
    "pom.xml",
    "build.gradle",
    "pubspec.yaml",
    "Package.swift",
];

pub fn detect_activity(project_root: &Path) -> ActivityStatus {
    if let Some(status) = git_activity(project_root) {
        return status;
    }
    if let Some(status) = manifest_activity(project_root) {
        return status;
    }
    if let Some(status) = source_activity(project_root) {
        return status;
    }
    ActivityStatus::Unknown
}

fn git_activity(root: &Path) -> Option<ActivityStatus> {
    let repo = Repository::open(root).ok()?;
    let head = repo.head().ok()?;
    let commit = head.peel_to_commit().ok()?;
    let commit_time = DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)?;
    classify_timestamp(commit_time)
}

fn manifest_activity(root: &Path) -> Option<ActivityStatus> {
    let mut newest: Option<DateTime<Utc>> = None;
    for name in MANIFEST_FILES {
        let path = root.join(name);
        if let Ok(meta) = std::fs::metadata(&path) {
            if let Ok(modified) = meta.modified() {
                let dt: DateTime<Utc> = modified.into();
                newest = Some(match newest {
                    Some(n) if dt > n => dt,
                    Some(n) => n,
                    None => dt,
                });
            }
        }
    }
    newest.and_then(classify_timestamp)
}

fn source_activity(root: &Path) -> Option<ActivityStatus> {
    let mut examined = 0u32;
    let mut newest: Option<DateTime<Utc>> = None;
    let walker = jwalk::WalkDir::new(root).max_depth(4).follow_links(false);
    for entry in walker.into_iter().flatten() {
        if examined >= 200 {
            break;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if SOURCE_EXTENSIONS.contains(&ext) {
                examined += 1;
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        let dt: DateTime<Utc> = modified.into();
                        newest = Some(match newest {
                            Some(n) if dt > n => dt,
                            Some(n) => n,
                            None => dt,
                        });
                    }
                }
            }
        }
    }
    newest.and_then(classify_timestamp)
}

fn classify_timestamp(ts: DateTime<Utc>) -> Option<ActivityStatus> {
    let now = Utc::now();
    let days = now.signed_duration_since(ts).num_days().max(0) as u32;
    if days <= 30 {
        Some(ActivityStatus::Active { days_since: days })
    } else {
        Some(ActivityStatus::Inactive { days_since: days })
    }
}

pub fn days_since_activity(activity: &ActivityStatus) -> Option<u32> {
    match activity {
        ActivityStatus::Active { days_since } | ActivityStatus::Inactive { days_since } => {
            Some(*days_since)
        }
        ActivityStatus::Unknown => None,
    }
}

#[allow(dead_code)]
fn is_stale(ts: DateTime<Utc>, threshold_days: u32) -> bool {
    Utc::now().signed_duration_since(ts) > Duration::days(threshold_days as i64)
}
