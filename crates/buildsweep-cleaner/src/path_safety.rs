use buildsweep_core::{BuildSweepError, Result, SafetyClass};
use std::path::{Component, Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

const PROTECTED_NAMES: &[&str] = &[
    ".git", "src", "app", "lib", ".env", "config", "assets", "public",
];

pub fn normalize_path(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    let nfc: String = s.nfc().collect();
    PathBuf::from(nfc.trim_end_matches(['/', '\\']))
}

pub fn validate_for_cleanup(
    path: &Path,
    approved_roots: &[PathBuf],
    expected_safety: SafetyClass,
) -> Result<PathBuf> {
    if !expected_safety.is_cleanup_eligible() {
        return Err(BuildSweepError::UnknownPath(path.display().to_string()));
    }

    let normalized = normalize_path(path);

    if is_filesystem_root(&normalized) {
        return Err(BuildSweepError::ProtectedPath(
            "filesystem root".to_string(),
        ));
    }

    if is_home_root(&normalized) {
        return Err(BuildSweepError::ProtectedPath("home directory".to_string()));
    }

    if let Some(name) = normalized.file_name().and_then(|n| n.to_str()) {
        if PROTECTED_NAMES.contains(&name) || name.starts_with(".env") {
            return Err(BuildSweepError::ProtectedPath(name.to_string()));
        }
    }

    for component in normalized.components() {
        if matches!(component, Component::ParentDir) {
            return Err(BuildSweepError::PathEscape(
                "path traversal detected".to_string(),
            ));
        }
    }

    let meta = std::fs::symlink_metadata(&normalized).map_err(|e| {
        BuildSweepError::InvalidPath(format!("{}: {}", normalized.display(), e))
    })?;

    if meta.file_type().is_symlink() {
        return Err(BuildSweepError::SymlinkRejected(
            normalized.display().to_string(),
        ));
    }

    #[cfg(windows)]
    if is_reparse_point(&normalized)? {
        return Err(BuildSweepError::SymlinkRejected(
            normalized.display().to_string(),
        ));
    }

    let canonical = normalized.canonicalize().map_err(|e| {
        BuildSweepError::InvalidPath(format!("{}: {}", normalized.display(), e))
    })?;

    let under_root = approved_roots.iter().any(|root| {
        let root_canon = root.canonicalize().unwrap_or_else(|_| root.clone());
        canonical.starts_with(&root_canon)
    });

    if !under_root {
        return Err(BuildSweepError::PathEscape(
            canonical.display().to_string(),
        ));
    }

    Ok(canonical)
}

fn is_filesystem_root(path: &Path) -> bool {
    path.parent().is_none() || path == Path::new("/") || path == Path::new("\\\\?\\")
}

fn is_home_root(path: &Path) -> bool {
    if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        let home_path = PathBuf::from(home);
        path == home_path
    } else {
        false
    }
}

#[cfg(windows)]
fn is_reparse_point(path: &Path) -> Result<bool> {
    use std::os::windows::fs::MetadataExt;
    let meta = std::fs::metadata(path)?;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    Ok(meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0)
}

#[cfg(not(windows))]
fn is_reparse_point(_path: &Path) -> Result<bool> {
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn rejects_protected_git() {
        let dir = tempdir().unwrap();
        let git = dir.path().join(".git");
        fs::create_dir(&git).unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let result = validate_for_cleanup(&git, &roots, SafetyClass::Safe);
        assert!(matches!(result, Err(BuildSweepError::ProtectedPath(_))));
    }

    #[test]
    fn rejects_unknown_safety() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("unknown");
        fs::create_dir(&path).unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let result = validate_for_cleanup(&path, &roots, SafetyClass::Unknown);
        assert!(matches!(result, Err(BuildSweepError::UnknownPath(_))));
    }
}
