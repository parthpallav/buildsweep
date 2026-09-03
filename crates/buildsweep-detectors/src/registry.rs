use crate::{classify_entry_name, DetectedArtifact, ProjectContext};
use buildsweep_core::SafetyClass;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

const MONOREPO_SUBDIRS: &[&str] = &["apps", "packages", "crates", "services", "libs"];

pub fn detect_artifacts(ctx: &ProjectContext) -> Vec<DetectedArtifact> {
    let mut artifacts = Vec::new();
    let mut seen_paths = HashSet::new();

    collect_artifacts_in_dir(&ctx.root, &ctx.root, ctx, &mut artifacts, &mut seen_paths);

    for sub in MONOREPO_SUBDIRS {
        let subpath = ctx.root.join(sub);
        if !subpath.is_dir() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(&subpath) {
            for entry in entries.flatten() {
                let package_root = entry.path();
                if package_root.is_dir() {
                    collect_artifacts_in_dir(
                        &package_root,
                        &ctx.root,
                        ctx,
                        &mut artifacts,
                        &mut seen_paths,
                    );
                }
            }
        }
    }

    artifacts
}

fn collect_artifacts_in_dir(
    scan_dir: &Path,
    project_root: &Path,
    ctx: &ProjectContext,
    artifacts: &mut Vec<DetectedArtifact>,
    seen_paths: &mut HashSet<PathBuf>,
) {
    let entries = match std::fs::read_dir(scan_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        if !path.is_dir() {
            continue;
        }

        let classified = if ctx.has_package_json && scan_dir == project_root {
            crate::js::detect_in_context(&name, true)
        } else {
            classify_entry_name(&name)
        };

        let Some((kind, safety, explanation)) = classified else {
            continue;
        };

        if safety == SafetyClass::Protected || safety == SafetyClass::Unknown {
            continue;
        }

        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        if !seen_paths.insert(canonical) {
            continue;
        }

        let relative = path
            .strip_prefix(project_root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| name.clone());

        artifacts.push(DetectedArtifact {
            name: name.clone(),
            relative_path: relative,
            kind,
            safety,
            explanation: explanation.to_string(),
        });
    }
}

pub fn is_project_marker(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    matches!(
        name,
        "package.json"
            | "Cargo.toml"
            | "pyproject.toml"
            | "requirements.txt"
            | "go.mod"
            | "pom.xml"
            | "build.gradle"
            | "build.gradle.kts"
            | "Podfile"
            | "Package.swift"
            | "pubspec.yaml"
            | "pnpm-workspace.yaml"
            | "yarn.lock"
            | "package-lock.json"
            | "lerna.json"
    ) || name.ends_with(".csproj")
        || name.ends_with(".sln")
}

pub fn is_git_dir(path: &Path) -> bool {
    path.is_dir() && path.file_name().map(|n| n == ".git").unwrap_or(false)
}
