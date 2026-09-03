use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        "target" => Some((
            ArtifactKind::RustTarget,
            SafetyClass::Review,
            "Build output; verify if Maven/Gradle target directory",
        )),
        ".gradle" => Some((
            ArtifactKind::GradleCache,
            SafetyClass::Review,
            "Gradle cache; may slow rebuilds if removed",
        )),
        _ => None,
    }
}
