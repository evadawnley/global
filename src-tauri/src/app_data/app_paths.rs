use std::path::PathBuf;
use tauri::api::path::app_data_dir;


static APP_NAME: &str = "Global";
static APP_CACHE_DIR: &str = "Cache";

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    let app_data_root = dirs::home_dir()
        .map(|path| {
            path.join(
                app_data_dir(&tauri::generate_context!().config())
                    .unwrap_or_else(|| "app_data".into()),
            )
        })
        .ok_or("Failed to get home directory")?;

    // Ensure the main app data directory exists
    std::fs::create_dir_all(&app_data_root).map_err(|e| e.to_string())?;

    // Create the subfolder within app_data (e.g., logs)
    let app_data_path = app_data_root.join(APP_NAME);
    std::fs::create_dir_all(&app_data_path).map_err(|e| e.to_string())?;

    Ok(app_data_path)
}
pub fn get_app_cache_dir() -> Result<PathBuf, String> {
    let app_data_path=get_app_data_dir()?;
    let app_cache_path = app_data_path.join(APP_CACHE_DIR);
    std::fs::create_dir_all(&app_cache_path).map_err(|e| e.to_string())?;

    Ok(app_cache_path)
}