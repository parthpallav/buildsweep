use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        "Pods" => Some((
            ArtifactKind::Pods,
            SafetyClass::Safe,
            "CocoaPods dependencies; reinstall with pod install",
        )),
        "DerivedData" => Some((
            ArtifactKind::DerivedData,
            SafetyClass::Safe,
            "Xcode derived data; regenerated on build",
        )),
        _ => None,
    }
}
