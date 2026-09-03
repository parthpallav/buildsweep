use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        "bin" | "obj" => Some((
            ArtifactKind::GenericBuild,
            SafetyClass::Review,
            "MSBuild output directory; verify project context",
        )),
        _ => None,
    }
}
