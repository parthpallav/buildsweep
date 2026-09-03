use buildsweep_detectors::{is_git_dir, is_project_marker};
use crate::workspace::workspace_member_paths;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DiscoveredProject {
    pub root: PathBuf,
    pub name: String,
}

pub fn discover_projects(roots: &[PathBuf], exclusions: &[String]) -> Vec<DiscoveredProject> {
    let mut found = Vec::new();
    let mut seen = HashSet::new();

    for root in roots {
        if exclusions.iter().any(|e| root.starts_with(e)) {
            continue;
        }
        discover_in_root(root, &mut found, &mut seen, exclusions, 0);
    }

    found.sort_by(|a, b| a.root.cmp(&b.root));
    dedupe_nested(&mut found);
    found
}

fn discover_in_root(
    root: &Path,
    found: &mut Vec<DiscoveredProject>,
    seen: &mut HashSet<PathBuf>,
    exclusions: &[String],
    depth: usize,
) {
    if depth > 24 {
        return;
    }
    if exclusions.iter().any(|e| root.starts_with(e)) {
        return;
    }

    let canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    if !seen.insert(canonical.clone()) {
        return;
    }

    if is_git_dir(&root.join(".git")) {
        register_git_workspace(root, found, seen);
    } else {
        let mut has_marker = false;
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if is_project_marker(&path) {
                    has_marker = true;
                    break;
                }
            }
        }

        if has_marker {
            push_project(root, found);
        }
    }

    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let skip = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| {
                        matches!(
                            n,
                            "node_modules" | "target" | ".git" | ".venv" | "venv" | ".next" | "dist"
                                | "build" | "bin" | "obj" | "__pycache__"
                        )
                    })
                    .unwrap_or(false);
                if !skip {
                    discover_in_root(&path, found, seen, exclusions, depth + 1);
                }
            }
        }
    }
}

fn register_git_workspace(root: &Path, found: &mut Vec<DiscoveredProject>, seen: &mut HashSet<PathBuf>) {
    let members = workspace_member_paths(root);
    if members.is_empty() {
        push_project(root, found);
    } else {
        let mut registered = false;
        for member in members {
            let canonical = member.canonicalize().unwrap_or_else(|_| member.clone());
            if !seen.insert(canonical) {
                continue;
            }
            if member.join("package.json").is_file()
                || member.join("Cargo.toml").is_file()
                || member.join("pyproject.toml").is_file()
                || is_project_marker_dir(&member)
            {
                push_project(&member, found);
                registered = true;
            }
        }

        if !registered {
            push_project(root, found);
        }
    }
}

fn is_project_marker_dir(path: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if is_project_marker(&entry.path()) {
                return true;
            }
        }
    }
    false
}

fn push_project(root: &Path, found: &mut Vec<DiscoveredProject>) {
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();
    found.push(DiscoveredProject {
        root: root.to_path_buf(),
        name,
    });
}

fn dedupe_nested(projects: &mut Vec<DiscoveredProject>) {
    let mut canonical_seen = HashSet::new();
    projects.retain(|p| {
        let canonical = p.root.canonicalize().unwrap_or_else(|_| p.root.clone());
        canonical_seen.insert(canonical)
    });

    let roots: Vec<PathBuf> = projects.iter().map(|p| p.root.clone()).collect();
    projects.retain(|p| {
        !roots.iter().any(|other| {
            other != &p.root
                && p.root.starts_with(other)
                && is_workspace_monorepo_root(other)
        })
    });
}

fn is_workspace_monorepo_root(root: &Path) -> bool {
    root.join("pnpm-workspace.yaml").is_file()
        || root.join("lerna.json").is_file()
        || (root.join("Cargo.toml").is_file() && cargo_has_workspace(root))
}

fn cargo_has_workspace(root: &Path) -> bool {
    std::fs::read_to_string(root.join("Cargo.toml"))
        .map(|c| c.contains("[workspace]"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn discovers_node_project() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("node-project");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("package.json"), r#"{"name":"test"}"#).unwrap();

        let projects = discover_projects(&[dir.path().to_path_buf()], &[]);
        assert_eq!(projects.len(), 1);
        assert!(projects[0].root.ends_with("node-project"));
    }

    #[test]
    fn discovers_cargo_workspace_members() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let projects = discover_projects(&[root], &[]);
        assert!(projects.len() > 1, "expected workspace members, got {}", projects.len());
    }
}
