use crate::state::SharedAppState;
use buildsweep_core::Settings;
use buildsweep_license::{check_feature, ProFeature};
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, SharedAppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn save_settings(state: State<'_, SharedAppState>, settings: Settings) -> Result<(), String> {
    if !settings.exclusions.is_empty() {
        let license = state.license_status.lock().map_err(|e| e.to_string())?;
        check_feature(&license, ProFeature::Exclusions)?;
    }

    {
        let mut s = state.settings.lock().map_err(|e| e.to_string())?;
        *s = settings.clone();
    }
    std::fs::create_dir_all(&state.data_dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(state.settings_path(), json).map_err(|e| e.to_string())?;
    Ok(())
}
