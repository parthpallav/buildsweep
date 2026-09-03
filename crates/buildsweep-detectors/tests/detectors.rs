use buildsweep_detectors::{classify_entry_name, detect_artifacts, ProjectContext};
use buildsweep_core::SafetyClass;
use std::path::PathBuf;

#[test]
fn detects_node_artifacts() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/node-project");
    if !root.exists() {
        return;
    }
    let ctx = ProjectContext::from_root(&root);
    let artifacts = detect_artifacts(&ctx);
    let names: Vec<_> = artifacts.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"node_modules"));
    assert!(names.contains(&".next"));
}

#[test]
fn protected_git_not_detected_as_artifact() {
    let result = classify_entry_name(".git");
    assert!(result.is_some());
    let (_, safety, _) = result.unwrap();
    assert_eq!(safety, SafetyClass::Protected);
}

#[test]
fn unknown_not_cleanup_eligible() {
    assert!(!SafetyClass::Unknown.is_cleanup_eligible());
    assert!(!SafetyClass::Protected.is_cleanup_eligible());
}
