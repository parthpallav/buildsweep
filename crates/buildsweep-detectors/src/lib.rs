mod dotnet;
mod flutter;
mod java;
mod js;
mod macos;
mod protected;
mod python;
mod registry;
mod rust_eco;
mod windows_eco;

pub use registry::*;

use buildsweep_core::{ArtifactKind, Ecosystem, SafetyClass};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub root: std::path::PathBuf,
    pub ecosystem: Ecosystem,
    pub has_package_json: bool,
    pub has_cargo_toml: bool,
    pub has_pyproject: bool,
    pub has_pubspec: bool,
    pub has_csproj: bool,
    pub has_sln: bool,
    pub is_workspace_root: bool,
}

impl ProjectContext {
    pub fn from_root(root: &Path) -> Self {
        let has_package_json = root.join("package.json").is_file();
        let has_cargo_toml = root.join("Cargo.toml").is_file();
        let has_pyproject = root.join("pyproject.toml").is_file() || root.join("requirements.txt").is_file();
        let has_pubspec = root.join("pubspec.yaml").is_file();
        let has_csproj = root.read_dir().map_or(false, |mut rd| {
            rd.filter_map(|e| e.ok()).any(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "csproj")
                    .unwrap_or(false)
            })
        });
        let has_sln = root.read_dir().map_or(false, |mut rd| {
            rd.filter_map(|e| e.ok()).any(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "sln")
                    .unwrap_or(false)
            })
        });

        let ecosystem = if has_package_json {
            Ecosystem::NodeJs
        } else if has_cargo_toml {
            Ecosystem::Rust
        } else if has_pyproject {
            Ecosystem::Python
        } else if has_pubspec {
            Ecosystem::Flutter
        } else if has_csproj || has_sln {
            Ecosystem::DotNet
        } else if root.join("pom.xml").is_file() || root.join("build.gradle").is_file() {
            Ecosystem::Java
        } else if root.join("go.mod").is_file() {
            Ecosystem::Go
        } else if root.join("Package.swift").is_file() {
            Ecosystem::Swift
        } else if root.join("Podfile").is_file() {
            Ecosystem::Xcode
        } else {
            Ecosystem::Unknown
        };

        let is_workspace_root = root.join("pnpm-workspace.yaml").is_file()
            || root.join("lerna.json").is_file()
            || root.join("Cargo.toml").is_file() && is_cargo_workspace(root);

        Self {
            root: root.to_path_buf(),
            ecosystem,
            has_package_json,
            has_cargo_toml,
            has_pyproject,
            has_pubspec,
            has_csproj,
            has_sln,
            is_workspace_root,
        }
    }
}

fn is_cargo_workspace(root: &Path) -> bool {
    if let Ok(content) = std::fs::read_to_string(root.join("Cargo.toml")) {
        content.contains("[workspace]")
    } else {
        false
    }
}

#[derive(Debug, Clone)]
pub struct DetectedArtifact {
    pub name: String,
    pub relative_path: String,
    pub kind: ArtifactKind,
    pub safety: SafetyClass,
    pub explanation: String,
}

pub fn classify_entry_name(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    if let Some(result) = protected::classify_protected(name) {
        return Some(result);
    }
    js::classify(name)
        .or_else(|| python::classify(name))
        .or_else(|| rust_eco::classify(name))
        .or_else(|| java::classify(name))
        .or_else(|| dotnet::classify(name))
        .or_else(|| flutter::classify(name))
        .or_else(|| macos::classify(name))
        .or_else(|| windows_eco::classify(name))
}
