use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        "bin" => Some((
            ArtifactKind::DotNetBin,
            SafetyClass::Safe,
            ".NET build output; regenerated on build",
        )),
        "obj" => Some((
            ArtifactKind::DotNetObj,
            SafetyClass::Safe,
            ".NET intermediate build files; regenerated on build",
        )),
        _ => None,
    }
}
