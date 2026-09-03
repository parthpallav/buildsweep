use buildsweep_core::{BuildSweepError, CleanupItemResult, CleanupPlan, CleanupResult, Result};
use chrono::Utc;
use std::path::Path;

pub trait TrashAdapter {
    fn move_to_trash(&self, path: &Path) -> Result<()>;
}

pub struct NativeTrashAdapter;

impl TrashAdapter for NativeTrashAdapter {
    fn move_to_trash(&self, path: &Path) -> Result<()> {
        trash::delete(path).map_err(|e| {
            BuildSweepError::Io(format!("failed to move {} to trash: {}", path.display(), e))
        })
    }
}

pub fn execute_cleanup(
    plan: &CleanupPlan,
    adapter: &dyn TrashAdapter,
) -> Result<CleanupResult> {
    crate::plan::revalidate_plan(plan)?;

    let mut items = Vec::new();
    let mut moved_bytes = 0u64;
    let mut moved_count = 0u32;
    let mut failed_count = 0u32;

    for item in &plan.items {
        let path = Path::new(&item.path);
        match adapter.move_to_trash(path) {
            Ok(()) => {
                moved_bytes += item.size_bytes;
                moved_count += 1;
                items.push(CleanupItemResult {
                    path: item.path.clone(),
                    success: true,
                    error: None,
                    size_bytes: item.size_bytes,
                });
            }
            Err(e) => {
                failed_count += 1;
                items.push(CleanupItemResult {
                    path: item.path.clone(),
                    success: false,
                    error: Some(e.to_string()),
                    size_bytes: item.size_bytes,
                });
            }
        }
    }

    Ok(CleanupResult {
        plan_id: plan.plan_id,
        moved_bytes,
        moved_count,
        failed_count,
        items,
        completed_at: Utc::now(),
    })
}
