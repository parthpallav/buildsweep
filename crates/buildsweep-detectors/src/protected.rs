use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify_protected(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        ".git" | "src" | "app" | "lib" | "include" | "assets" | "public" | "static" => Some((
            ArtifactKind::Other,
            SafetyClass::Protected,
            "Protected project directory",
        )),
        ".env" | ".env.local" | ".env.production" => Some((
            ArtifactKind::Other,
            SafetyClass::Protected,
            "Environment configuration file",
        )),
        "config" | "configs" | "settings" => Some((
            ArtifactKind::Other,
            SafetyClass::Protected,
            "Configuration directory",
        )),
        _ if name.ends_with(".db") || name.ends_with(".sqlite") => Some((
            ArtifactKind::Other,
            SafetyClass::Protected,
            "Database file",
        )),
        _ => None,
    }
}
