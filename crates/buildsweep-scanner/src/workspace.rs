use std::path::{Path, PathBuf};

/// Collect workspace member paths relative to a repository root.
pub fn workspace_member_paths(root: &Path) -> Vec<PathBuf> {
    let mut members = Vec::new();
    members.extend(parse_cargo_workspace_members(root));
    members.extend(parse_npm_workspaces(root));
    members.extend(parse_pnpm_workspaces(root));

    members
        .into_iter()
        .map(|m| root.join(m))
        .filter(|p| p.is_dir())
        .collect()
}

fn parse_cargo_workspace_members(root: &Path) -> Vec<String> {
    let content = match std::fs::read_to_string(root.join("Cargo.toml")) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    if !content.contains("[workspace]") {
        return Vec::new();
    }

    let mut members = Vec::new();
    let mut in_members = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_members = trimmed == "[workspace]" || trimmed.starts_with("[workspace.");
            continue;
        }
        if !in_members {
            continue;
        }
        if trimmed.starts_with("members") {
            in_members = true;
            if let Some(start) = trimmed.find('[') {
                if let Some(end) = trimmed.rfind(']') {
                    let inner = &trimmed[start + 1..end];
                    for part in inner.split(',') {
                        if let Some(m) = parse_quoted(part.trim()) {
                            members.push(m);
                        }
                    }
                    in_members = false;
                }
            }
            continue;
        }
        if trimmed.starts_with('"') {
            if let Some(m) = parse_quoted(trimmed.trim_end_matches(',')) {
                members.push(m);
            }
        }
    }

    members
}

fn parse_npm_workspaces(root: &Path) -> Vec<String> {
    let content = match std::fs::read_to_string(root.join("package.json")) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut members = Vec::new();
    if let Some(workspaces) = value.get("workspaces") {
        match workspaces {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        members.extend(expand_workspace_glob(root, s));
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                if let Some(arr) = obj.get("packages").and_then(|v| v.as_array()) {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            members.extend(expand_workspace_glob(root, s));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    members
}

fn parse_pnpm_workspaces(root: &Path) -> Vec<String> {
    let content = match std::fs::read_to_string(root.join("pnpm-workspace.yaml")) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut members = Vec::new();
    let mut in_packages = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("packages:") {
            in_packages = true;
            continue;
        }
        if in_packages {
            if trimmed.starts_with('-') {
                let pattern = trimmed.trim_start_matches('-').trim();
                let pattern = pattern.trim_matches('\'').trim_matches('"');
                members.extend(expand_workspace_glob(root, pattern));
            } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
                break;
            }
        }
    }
    members
}

fn expand_workspace_glob(root: &Path, pattern: &str) -> Vec<String> {
    if pattern.contains('*') {
        let prefix = pattern.split('*').next().unwrap_or("");
        let base = root.join(prefix);
        if !base.is_dir() {
            return Vec::new();
        }
        return std::fs::read_dir(&base)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| e.path().is_dir())
                    .filter_map(|e| {
                        e.path()
                            .strip_prefix(root)
                            .ok()
                            .map(|p| p.to_string_lossy().to_string())
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    vec![pattern.to_string()]
}

fn parse_quoted(s: &str) -> Option<String> {
    let s = s.trim().trim_matches(',').trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Some(s[1..s.len() - 1].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_buildsweep_cargo_workspace() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let members = workspace_member_paths(&root);
        assert!(members.iter().any(|p| p.ends_with("buildsweep-core")));
    }
}
