use buildsweep_core::{ArtifactKind, SafetyClass};

pub fn classify(name: &str) -> Option<(ArtifactKind, SafetyClass, &'static str)> {
    match name {
        ".venv" | "venv" => Some((
            ArtifactKind::Venv,
            SafetyClass::Safe,
            "Python virtual environment; recreate with venv/poetry",
        )),
        "__pycache__" => Some((
            ArtifactKind::Pycache,
            SafetyClass::Safe,
            "Python bytecode cache; regenerated automatically",
        )),
        ".pytest_cache" => Some((
            ArtifactKind::PytestCache,
            SafetyClass::Safe,
            "pytest cache; regenerated on test run",
        )),
        ".mypy_cache" => Some((
            ArtifactKind::MypyCache,
            SafetyClass::Safe,
            "mypy cache; regenerated on type check",
        )),
        _ => None,
    }
}
