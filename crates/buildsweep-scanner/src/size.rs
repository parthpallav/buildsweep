use std::path::Path;

pub fn directory_size(path: &Path) -> u64 {
    let mut total = 0u64;
    let walker = jwalk::WalkDir::new(path).follow_links(false);
    for entry in walker.into_iter().flatten() {
        if entry.file_type().is_file() {
            total += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }
    total
}

pub fn project_total_size(root: &Path, artifact_paths: &[std::path::PathBuf]) -> u64 {
    let artifact_total: u64 = artifact_paths.iter().map(|p| directory_size(p)).sum();
    let source_estimate = estimate_source_size(root, artifact_paths);
    artifact_total + source_estimate
}

fn estimate_source_size(root: &Path, artifact_paths: &[std::path::PathBuf]) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if artifact_paths.iter().any(|a| a == &path) {
                continue;
            }
            if path.is_file() {
                total += entry.metadata().map(|m| m.len()).unwrap_or(0);
            } else if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !matches!(name, "node_modules" | "target" | ".git" | ".venv" | "venv" | ".next" | "dist" | "build" | "bin" | "obj") {
                    total += directory_size(&path).min(10_000_000);
                }
            }
        }
    }
    total.min(500_000_000)
}
