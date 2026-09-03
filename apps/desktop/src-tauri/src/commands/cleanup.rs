use crate::state::SharedAppState;
use buildsweep_cleaner::{
    append_history, append_manifest, NativeTrashAdapter,
};
use buildsweep_cleaner::{
    build_cleanup_plan as make_cleanup_plan, execute_cleanup as run_cleanup,
};
use buildsweep_core::{CleanupPlan, CleanupResult, CleanupSelection};
use buildsweep_license::can_batch_cleanup;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

#[tauri::command(rename_all = "snake_case")]
pub async fn build_cleanup_plan(
    state: State<'_, SharedAppState>,
    selections: Vec<CleanupSelection>,
    approved_roots: Vec<String>,
) -> Result<CleanupPlan, String> {
    let last = state.last_scan.lock().map_err(|e| e.to_string())?;
    let scan = last
        .as_ref()
        .ok_or_else(|| "No scan results available".to_string())?;

    let project_count = selections.len();
    let license = state.license_status.lock().map_err(|e| e.to_string())?;
    can_batch_cleanup(&license, project_count).map_err(|e| e)?;

    let plan = make_cleanup_plan(&scan.projects, &selections, &approved_roots)
        .map_err(|e| e.to_string())?;

    let plan_id = plan.plan_id;
    {
        let mut plans = state.plans.lock().map_err(|e| e.to_string())?;
        plans.insert(plan_id, plan.clone());
    }

    Ok(plan)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn execute_cleanup(
    app: AppHandle,
    state: State<'_, SharedAppState>,
    plan_id: Uuid,
) -> Result<CleanupResult, String> {
    let plan = {
        let plans = state.plans.lock().map_err(|e| e.to_string())?;
        plans
            .get(&plan_id)
            .cloned()
            .ok_or_else(|| "Cleanup plan not found".to_string())?
    };

    let adapter = NativeTrashAdapter;
    let result = run_cleanup(&plan, &adapter).map_err(|e| e.to_string())?;

    let _ = append_manifest(
        &state.manifest_path(),
        &result,
        env!("CARGO_PKG_VERSION"),
    );

    let license = state.license_status.lock().map_err(|e| e.to_string())?;
    if buildsweep_license::is_pro_enabled(&license) {
        let _ = append_history(
            &state.history_path(),
            result.moved_bytes,
            result.moved_count,
        );
    }

    {
        let mut plans = state.plans.lock().map_err(|e| e.to_string())?;
        plans.remove(&plan_id);
    }

    let _ = app.emit("cleanup://complete", &result);
    Ok(result)
}
