use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    if name == "target" {
        Some((
            ArtifactKind::RustTarget,
            SafetyClass::Safe,
            "Rust build artifacts; regenerated with cargo build",
        ))
    } else {
        None
    }
}
