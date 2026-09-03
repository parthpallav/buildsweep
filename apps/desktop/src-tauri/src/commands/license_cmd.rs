use crate::build_info::allow_local_license;
use crate::state::SharedAppState;
use buildsweep_cleaner::load_history;
use buildsweep_core::HistoryEntry;
use buildsweep_license::{
    generate_local_pro_license, resolve_status, LicenseStatus, LicenseTier, ProFeature,
    EMBEDDED_PUBLIC_KEY_B64,
};
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

fn local_private_key_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        let path = PathBuf::from(home).join(".buildsweep/dev-private.key");
        if path.is_file() {
            return path;
        }
    }

    if let Ok(from_env) = std::env::var("BUILDSWEEP_LICENSE_PRIVATE_KEY_PATH") {
        return PathBuf::from(from_env);
    }

    PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".buildsweep/dev-private.key")
}

fn read_local_private_key() -> Result<String, String> {
    if let Ok(inline) = std::env::var("BUILDSWEEP_LICENSE_PRIVATE_KEY") {
        if !inline.trim().is_empty() {
            return Ok(inline);
        }
    }

    let path = local_private_key_path();
    std::fs::read_to_string(&path).map_err(|_| {
        format!(
            "Local signing key not found at {}. Run: cargo run -p license-signer -- install-dev",
            path.display()
        )
    })
}

async fn persist_license(
    state: &SharedAppState,
    license_key: String,
    status: LicenseStatus,
) -> Result<(), String> {
    std::fs::create_dir_all(&state.data_dir).map_err(|e| e.to_string())?;
    std::fs::write(state.license_path(), &license_key).map_err(|e| e.to_string())?;

    {
        let mut stored = state.stored_license.lock().map_err(|e| e.to_string())?;
        *stored = Some(license_key);
    }
    {
        let mut lic = state.license_status.lock().map_err(|e| e.to_string())?;
        *lic = status;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_license_status(state: State<'_, SharedAppState>) -> Result<LicenseStatus, String> {
    let stored = state.stored_license.lock().map_err(|e| e.to_string())?;
    let status = resolve_status(stored.as_deref());
    Ok(status)
}

#[tauri::command]
pub async fn activate_license(
    state: State<'_, SharedAppState>,
    license_key: String,
) -> Result<LicenseStatus, String> {
    let status = buildsweep_license::verify_license(&license_key, EMBEDDED_PUBLIC_KEY_B64)
        .map_err(|e| e.to_string())?;

    if status.tier != LicenseTier::Pro {
        return Err("License is not a Pro license".to_string());
    }

    persist_license(state.inner(), license_key, status.clone()).await?;
    Ok(status)
}

#[tauri::command]
pub async fn generate_local_license(
    state: State<'_, SharedAppState>,
) -> Result<LicenseStatus, String> {
    if !allow_local_license() {
        return Err(
            "Local license generation is only available in the Personal build.".to_string(),
        );
    }

    let private_key = read_local_private_key()?;
    let license_id = format!("LOCAL-{}", Uuid::new_v4());
    let license_key =
        generate_local_pro_license(&private_key, &license_id).map_err(|e| e.to_string())?;

    let status = buildsweep_license::verify_license(&license_key, EMBEDDED_PUBLIC_KEY_B64)
        .map_err(|e| e.to_string())?;

    persist_license(state.inner(), license_key, status.clone()).await?;
    Ok(status)
}

#[tauri::command]
pub async fn get_cleanup_history(state: State<'_, SharedAppState>) -> Result<Vec<HistoryEntry>, String> {
    let license = state.license_status.lock().map_err(|e| e.to_string())?;
    buildsweep_license::check_feature(&license, ProFeature::CleanupHistory)
        .map_err(|e| e)?;
    Ok(load_history(&state.history_path()))
}
