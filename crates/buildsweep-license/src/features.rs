use crate::{current_status, LicenseStatus, LicenseTier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProFeature {
    BatchCleanup,
    CleanupPresets,
    Exclusions,
    CleanupHistory,
}

pub fn is_pro_enabled(status: &LicenseStatus) -> bool {
    status.valid && status.tier == LicenseTier::Pro
}

pub fn check_feature(status: &LicenseStatus, feature: ProFeature) -> Result<(), String> {
    if is_pro_enabled(status) {
        return Ok(());
    }
    let name = match feature {
        ProFeature::BatchCleanup => "batch cleanup",
        ProFeature::CleanupPresets => "cleanup presets",
        ProFeature::Exclusions => "exclusions",
        ProFeature::CleanupHistory => "cleanup history",
    };
    Err(format!("Pro license required for {}", name))
}

pub fn can_batch_cleanup(status: &LicenseStatus, project_count: usize) -> Result<(), String> {
    if project_count <= 1 {
        return Ok(());
    }
    check_feature(status, ProFeature::BatchCleanup)
}

pub fn resolve_status(stored_license: Option<&str>) -> LicenseStatus {
    current_status(stored_license)
}
