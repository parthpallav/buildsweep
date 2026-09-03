use buildsweep_cleaner::validate_for_cleanup;
use buildsweep_core::SafetyClass;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn safe_path_under_root_is_allowed() {
    let dir = tempdir().unwrap();
    let artifact = dir.path().join("node_modules");
    fs::create_dir(&artifact).unwrap();
    let roots = vec![dir.path().to_path_buf()];
    let result = validate_for_cleanup(&artifact, &roots, SafetyClass::Safe);
    assert!(result.is_ok());
}

#[test]
fn path_outside_root_rejected() {
    let dir = tempdir().unwrap();
    let other = tempdir().unwrap();
    let artifact = other.path().join("node_modules");
    fs::create_dir(&artifact).unwrap();
    let roots = vec![dir.path().to_path_buf()];
    let result = validate_for_cleanup(&artifact, &roots, SafetyClass::Safe);
    assert!(result.is_err());
}

#[test]
fn unknown_safety_rejected() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("mystery");
    fs::create_dir(&path).unwrap();
    let roots = vec![dir.path().to_path_buf()];
    let result = validate_for_cleanup(&path, &roots, SafetyClass::Unknown);
    assert!(result.is_err());
}

#[test]
fn protected_git_rejected() {
    let dir = tempdir().unwrap();
    let git = dir.path().join(".git");
    fs::create_dir(&git).unwrap();
    let roots = vec![dir.path().to_path_buf()];
    let result = validate_for_cleanup(&git, &roots, SafetyClass::Safe);
    assert!(result.is_err());
}

#[cfg(unix)]
#[test]
fn symlink_escape_rejected() {
    use std::os::unix::fs::symlink;
    let dir = tempdir().unwrap();
    let outside = tempdir().unwrap();
    let outside_file = outside.path().join("secret");
    fs::write(&outside_file, "data").unwrap();
    let link = dir.path().join("node_modules");
    symlink(outside_file, &link).unwrap();
    let roots = vec![dir.path().to_path_buf()];
    let result = validate_for_cleanup(&link, &roots, SafetyClass::Safe);
    assert!(result.is_err());
}

#[test]
fn cleanup_plan_rejects_protected() {
    use buildsweep_cleaner::build_cleanup_plan;
    use buildsweep_core::{Artifact, ArtifactKind, CleanupSelection, Ecosystem, Project, WasteScoreBreakdown, ActivityStatus};
    use uuid::Uuid;

    let project = Project {
        id: Uuid::new_v4(),
        name: "test".into(),
        path: "/tmp/test".into(),
        ecosystem: Ecosystem::NodeJs,
        activity: ActivityStatus::Unknown,
        total_size_bytes: 100,
        reclaimable_size_bytes: 0,
        waste_score: WasteScoreBreakdown {
            total: 0,
            inactivity_score: 0,
            reclaimable_ratio_score: 0,
            reclaimable_size_score: 0,
            artifact_score: 0,
            reasons: vec![],
        },
        artifacts: vec![Artifact {
            id: Uuid::new_v4(),
            name: ".git".into(),
            path: "/tmp/test/.git".into(),
            size_bytes: 100,
            safety: SafetyClass::Protected,
            kind: ArtifactKind::Other,
            explanation: "protected".into(),
            shared: false,
        }],
        is_inactive: false,
    };

    let selection = CleanupSelection {
        project_id: project.id,
        artifact_ids: vec![project.artifacts[0].id],
    };

    let result = build_cleanup_plan(&[project], &[selection], &["/tmp".to_string()]);
    assert!(result.is_err());
}
