use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};

#[tauri::command]
pub async fn open_file_dialog(app_handle: AppHandle) -> Result<String, String> {
    let application_dialog = app_handle.dialog().clone();
    let file_dialog = FileDialogBuilder::new(application_dialog).add_filter("CSV", &["csv"]);
    // .blocking_pick_file()
    // .unwrap()
    // .to_string();

    /*
    TODO: Add another file filters for cross platforms
    #[cfg(target_os = "macos")] {
        file_dialog = file_dialog.add_filter("CSV", &["csv"]);
    }

    #[cfg(not(target_os = "macos"))] {
        file_dialog = file_dialog.set_title("Choose your sender file").add_filter("CSV", &["csv"]);
    }
    */

    match file_dialog.blocking_pick_file() {
        Some(file_path) => Ok(file_path.to_string()),
        None => Ok("".to_string()),
    }
}
