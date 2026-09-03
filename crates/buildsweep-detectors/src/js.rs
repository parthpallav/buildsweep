use super::classify_entry_name;
use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        "node_modules" => Some((
            ArtifactKind::NodeModules,
            SafetyClass::Safe,
            "npm/yarn/pnpm dependencies; reinstall with package manager",
        )),
        ".next" => Some((
            ArtifactKind::NextBuild,
            SafetyClass::Safe,
            "Next.js build output; regenerated on build",
        )),
        ".nuxt" => Some((
            ArtifactKind::NuxtBuild,
            SafetyClass::Safe,
            "Nuxt build output; regenerated on build",
        )),
        "coverage" => Some((
            ArtifactKind::Coverage,
            SafetyClass::Safe,
            "Test coverage output; regenerated on test run",
        )),
        ".turbo" => Some((
            ArtifactKind::Turbo,
            SafetyClass::Safe,
            "Turborepo cache; regenerated automatically",
        )),
        "dist" => Some((
            ArtifactKind::Dist,
            SafetyClass::Review,
            "Build output directory; verify before removing",
        )),
        _ => None,
    }
}

pub fn detect_in_context(name: &str, has_package_json: bool) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    if let Some(result) = classify(name) {
        if name == "dist" && !has_package_json {
            return None;
        }
        return Some(result);
    }
    classify_entry_name(name)
}
