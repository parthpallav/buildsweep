use crate::state::SharedAppState;
use buildsweep_core::{ScanProgress, ScanResult};
use buildsweep_scanner::ScanOptions;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

#[tauri::command]
pub async fn pick_scan_folders(app: AppHandle) -> Result<Vec<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let folders = app
        .dialog()
        .file()
        .set_title("Select project folders")
        .blocking_pick_folders();

    let paths: Vec<String> = folders
        .unwrap_or_default()
        .into_iter()
        .filter_map(|p| {
            let fallback = p.to_string();
            p.simplified()
                .into_path()
                .ok()
                .map(|path| path.to_string_lossy().to_string())
                .or_else(|| if fallback.is_empty() { None } else { Some(fallback) })
        })
        .collect();

    if paths.is_empty() {
        return Ok(vec![]);
    }

    Ok(paths)
}

#[tauri::command]
pub async fn start_scan(
    app: AppHandle,
    state: State<'_, SharedAppState>,
    roots: Vec<String>,
) -> Result<Uuid, String> {
    if roots.is_empty() {
        return Err("Select at least one folder to scan.".to_string());
    }

    let root_paths: Vec<PathBuf> = roots.iter().map(PathBuf::from).collect();
    for path in &root_paths {
        if !path.exists() {
            return Err(format!("Folder does not exist: {}", path.display()));
        }
        if !path.is_dir() {
            return Err(format!("Not a folder: {}", path.display()));
        }
    }

    let settings = state.settings.lock().map_err(|e| e.to_string())?.clone();
    let scan_id = Uuid::new_v4();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ScanProgress>();
    let app_progress = app.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_progress.emit("scan://progress", &progress);
        }
    });

    let options = ScanOptions {
        roots: root_paths,
        settings,
    };

    let app_handle = app.clone();
    let state_inner: SharedAppState = Arc::clone(state.inner());
    let partial_snapshot = Arc::new(std::sync::Mutex::new(None::<ScanResult>));
    let partial_for_scan = Arc::clone(&partial_snapshot);

    let partial_for_poll = Arc::clone(&partial_snapshot);

    tokio::spawn(async move {
        let state_for_poll = Arc::clone(&state_inner);
        let poll = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                if let Ok(guard) = partial_for_poll.lock() {
                    if let Some(partial) = guard.clone() {
                        if let Ok(mut last) = state_for_poll.last_scan.lock() {
                            *last = Some(partial);
                        }
                    }
                }
            }
        });

        let result = tokio::task::spawn_blocking(move || {
            buildsweep_scanner::run_scan_blocking(scan_id, options, tx, Some(partial_for_scan))
        })
        .await;

        poll.abort();

        match result {
            Ok(Ok(scan_result)) => {
                if let Ok(mut last) = state_inner.last_scan.lock() {
                    *last = Some(scan_result.clone());
                }
                let _ = app_handle.emit("scan://complete", &scan_result);
            }
            Ok(Err(e)) => {
                let _ = app_handle.emit("scan://error", e.to_string());
            }
            Err(e) => {
                let _ = app_handle.emit("scan://error", e.to_string());
            }
        }
        if let Ok(mut active) = state_inner.active_scan.lock() {
            *active = None;
        }
    });

    {
        let mut active = state.active_scan.lock().map_err(|e| e.to_string())?;
        *active = Some(crate::state::ActiveScan {
            scan_id,
            cancel: tokio_util::sync::CancellationToken::new(),
            result: None,
        });
    }

    Ok(scan_id)
}

#[tauri::command]
pub async fn cancel_scan(state: State<'_, SharedAppState>, scan_id: Uuid) -> Result<(), String> {
    let mut active = state.active_scan.lock().map_err(|e| e.to_string())?;
    if let Some(scan) = active.as_ref() {
        if scan.scan_id == scan_id {
            scan.cancel.cancel();
            *active = None;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_scan_snapshot(state: State<'_, SharedAppState>) -> Result<Option<ScanResult>, String> {
    let last = state.last_scan.lock().map_err(|e| e.to_string())?;
    Ok(last.clone())
}
