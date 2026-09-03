use buildsweep_core::{
    Artifact, BuildSweepError, CleanupPlan, CleanupPlanItem, CleanupSelection, Project, Result,
};
use std::path::PathBuf;
use uuid::Uuid;

pub fn build_cleanup_plan(
    projects: &[Project],
    selections: &[CleanupSelection],
    approved_roots: &[String],
) -> Result<CleanupPlan> {
    let roots: Vec<PathBuf> = approved_roots.iter().map(PathBuf::from).collect();
    let mut items = Vec::new();

    for selection in selections {
        let project = projects
            .iter()
            .find(|p| p.id == selection.project_id)
            .ok_or_else(|| BuildSweepError::Internal("project not found".to_string()))?;

        for artifact_id in &selection.artifact_ids {
            let artifact = project
                .artifacts
                .iter()
                .find(|a| a.id == *artifact_id)
                .ok_or_else(|| BuildSweepError::Internal("artifact not found".to_string()))?;

            validate_artifact(artifact, &roots)?;
            items.push(CleanupPlanItem {
                artifact_id: artifact.id,
                project_name: project.name.clone(),
                name: artifact.name.clone(),
                path: artifact.path.clone(),
                size_bytes: artifact.size_bytes,
                safety: artifact.safety,
            });
        }
    }

    let total_bytes: u64 = items.iter().map(|i| i.size_bytes).sum();
    let folder_count = items.len() as u32;

    Ok(CleanupPlan {
        plan_id: Uuid::new_v4(),
        items,
        total_bytes,
        folder_count,
        approved_roots: approved_roots.to_vec(),
    })
}

fn validate_artifact(artifact: &Artifact, roots: &[PathBuf]) -> Result<()> {
    if !artifact.safety.is_cleanup_eligible() {
        return Err(BuildSweepError::UnknownPath(artifact.path.clone()));
    }
    crate::path_safety::validate_for_cleanup(
        PathBuf::from(&artifact.path).as_path(),
        roots,
        artifact.safety,
    )?;
    Ok(())
}

pub fn revalidate_plan(plan: &CleanupPlan) -> Result<()> {
    let roots: Vec<PathBuf> = plan.approved_roots.iter().map(PathBuf::from).collect();
    for item in &plan.items {
        crate::path_safety::validate_for_cleanup(
            PathBuf::from(&item.path).as_path(),
            &roots,
            item.safety,
        )?;
    }
    Ok(())
}
