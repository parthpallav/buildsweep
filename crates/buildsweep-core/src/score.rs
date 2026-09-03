use crate::config::InactivityThreshold;
use crate::models::{ActivityStatus, Artifact, WasteScoreBreakdown};

pub fn compute_waste_score(
    activity: &ActivityStatus,
    total_size_bytes: u64,
    reclaimable_size_bytes: u64,
    artifacts: &[Artifact],
    threshold: InactivityThreshold,
) -> WasteScoreBreakdown {
    let threshold_days = threshold.days() as f64;

    let inactivity_score = match activity {
        ActivityStatus::Inactive { days_since } => {
            let ratio = (*days_since as f64 / threshold_days).min(1.0);
            (ratio * 40.0).round() as u8
        }
        ActivityStatus::Active { days_since } => {
            let ratio = (1.0 - (*days_since as f64 / threshold_days)).max(0.0);
            ((1.0 - ratio) * 20.0).round() as u8
        }
        ActivityStatus::Unknown => 0,
    };

    let reclaimable_ratio_score = if total_size_bytes > 0 {
        let ratio = reclaimable_size_bytes as f64 / total_size_bytes as f64;
        (ratio * 30.0).min(30.0).round() as u8
    } else {
        0
    };

    let reclaimable_gb = reclaimable_size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let reclaimable_size_score = if reclaimable_gb >= 10.0 {
        20
    } else if reclaimable_gb >= 5.0 {
        16
    } else if reclaimable_gb >= 1.0 {
        12
    } else if reclaimable_gb >= 0.5 {
        8
    } else if reclaimable_gb > 0.0 {
        4
    } else {
        0
    };

    let safe_count = artifacts
        .iter()
        .filter(|a| a.safety == crate::models::SafetyClass::Safe)
        .count();
    let artifact_score = (safe_count * 2).min(10) as u8;

    let total = (inactivity_score as u16
        + reclaimable_ratio_score as u16
        + reclaimable_size_score as u16
        + artifact_score as u16)
        .min(100) as u8;

    let mut reasons = Vec::new();
    match activity {
        ActivityStatus::Inactive { days_since } => {
            reasons.push(format!("inactive for {} days", days_since));
        }
        ActivityStatus::Active { days_since } => {
            reasons.push(format!("active within {} days", days_since));
        }
        ActivityStatus::Unknown => {
            reasons.push("activity could not be determined".to_string());
        }
    }
    if total_size_bytes > 0 && reclaimable_size_bytes > 0 {
        let pct = (reclaimable_size_bytes as f64 / total_size_bytes as f64 * 100.0).round() as u64;
        reasons.push(format!("{}% of project storage is regeneratable", pct));
    }
    if reclaimable_size_bytes > 0 {
        reasons.push(format!(
            "{} reclaimable",
            format_bytes(reclaimable_size_bytes)
        ));
    }
    if safe_count > 0 {
        reasons.push(format!("{} known safe artifacts", safe_count));
    }

    WasteScoreBreakdown {
        total,
        inactivity_score,
        reclaimable_ratio_score,
        reclaimable_size_score,
        artifact_score,
        reasons,
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const KB: f64 = 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

pub fn is_inactive(activity: &ActivityStatus, threshold: InactivityThreshold) -> bool {
    match activity {
        ActivityStatus::Inactive { days_since } => *days_since >= threshold.days(),
        ActivityStatus::Active { .. } => false,
        ActivityStatus::Unknown => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ActivityStatus, Artifact, ArtifactKind, SafetyClass};
    use uuid::Uuid;

    fn artifact(safety: SafetyClass) -> Artifact {
        Artifact {
            id: Uuid::new_v4(),
            name: "node_modules".to_string(),
            path: "/tmp/node_modules".to_string(),
            size_bytes: 1_000_000,
            safety,
            kind: ArtifactKind::NodeModules,
            explanation: "test".to_string(),
            shared: false,
        }
    }

    #[test]
    fn inactive_project_scores_higher() {
        let inactive = compute_waste_score(
            &ActivityStatus::Inactive { days_since: 312 },
            20_000_000_000,
            15_000_000_000,
            &vec![artifact(SafetyClass::Safe), artifact(SafetyClass::Safe), artifact(SafetyClass::Safe)],
            InactivityThreshold::Days90,
        );
        let active = compute_waste_score(
            &ActivityStatus::Active { days_since: 5 },
            20_000_000_000,
            15_000_000_000,
            &vec![artifact(SafetyClass::Safe), artifact(SafetyClass::Safe), artifact(SafetyClass::Safe)],
            InactivityThreshold::Days90,
        );
        assert!(inactive.total > active.total);
        assert!(inactive.total <= 100);
    }

    #[test]
    fn unknown_activity_zero_inactivity_score() {
        let score = compute_waste_score(
            &ActivityStatus::Unknown,
            1_000_000,
            500_000,
            &[],
            InactivityThreshold::Days90,
        );
        assert_eq!(score.inactivity_score, 0);
    }
}
