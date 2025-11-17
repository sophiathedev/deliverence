use tauri::{Manager, WebviewWindowBuilder};

#[tauri::command]
pub async fn open_sending_mail_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(main) = app_handle.get_webview_window("main") {
        let _ = main.hide();
    }

    let new_sending_mail_window = WebviewWindowBuilder::new(
        &app_handle,
        "sending_mail",
        tauri::WebviewUrl::App("sending_mail.html".into()),
    )
    .title("Deliverence - Sending mail")
    .inner_size(1000.0, 650.0)
    .center()
    .always_on_top(false)
    .resizable(false)
    .maximizable(false)
    .fullscreen(false)
    .build()
    .map_err(|e| e.to_string())?;

    let app_handle_clone = app_handle.clone();
    new_sending_mail_window.on_window_event(move |event| match event {
        tauri::WindowEvent::CloseRequested { .. } => {
            if let Some(main) = app_handle_clone.get_webview_window("main") {
                let _ = main.close();
            }
        }
        _ => {}
    });

    Ok(())
}

#[tauri::command]
pub async fn open_template_manager_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(main) = app_handle.get_webview_window("main") {
        let _ = main.show();
        let _ = main.set_focus();
    }

    let new_template_manager_window = WebviewWindowBuilder::new(
        &app_handle,
        "template_manager",
        tauri::WebviewUrl::App("template_manager.html".into()),
    )
    .title("Deliverence - Template manager")
    .inner_size(1000.0, 650.0)
    .center()
    .always_on_top(false)
    .build()
    .map_err(|e| e.to_string())?;

    let app_handle_clone = app_handle.clone();
    new_template_manager_window.on_window_event(move |event| match event {
        tauri::WindowEvent::CloseRequested { .. } => {
            if let Some(main) = app_handle_clone.get_webview_window("main") {
                let _ = main.show();
                let _ = main.set_focus();
            }
        }
        _ => {}
    });

    Ok(())
}
