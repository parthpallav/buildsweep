use crate::build_info::{get_build_info, BuildInfo};

#[tauri::command]
pub async fn get_build_info_cmd() -> Result<BuildInfo, String> {
    Ok(get_build_info())
}
