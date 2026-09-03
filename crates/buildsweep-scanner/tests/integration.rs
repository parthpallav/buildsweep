use buildsweep_scanner::discover_projects;
use std::path::PathBuf;

#[test]
fn discovers_fixture_projects() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures");
    if !fixtures.exists() {
        return;
    }
    let projects = discover_projects(&[fixtures], &[]);
    assert!(!projects.is_empty());
}

#[test]
fn discovers_node_project_by_marker() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/node-project");
    if !root.exists() {
        return;
    }
    let projects = discover_projects(&[root.parent().unwrap().to_path_buf()], &[]);
    assert!(projects.iter().any(|p| p.name == "node-project"));
}

#[test]
fn discovers_in_downloads_smoke() {
    let downloads = std::path::PathBuf::from(std::env::var("HOME").unwrap()).join("Downloads");
    if !downloads.is_dir() {
        return;
    }
    let projects = buildsweep_scanner::discover_projects(&[downloads.clone()], &[]);
    eprintln!("Downloads scan found {} projects", projects.len());
    assert!(
        projects.len() >= 10,
        "expected many projects in Downloads, got {}",
        projects.len()
    );
}
