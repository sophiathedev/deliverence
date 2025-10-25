// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod command;

use command::data::{read_csv, read_receiver_csv_list};
use command::dialog::open_file_dialog;
use command::window::{open_sending_mail_window, open_template_manager_window};
use command::mailer::send_email_by_thread;
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri_plugin_cache::CacheExt;

fn initialize_main_menu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let handle = app.handle().clone();

    let application_name = &app.package_info().name;

    let mut main_menu = MenuBuilder::new(&handle);

    #[cfg(target_os = "macos")]
    {
        let macos_application_menu = SubmenuBuilder::new(&handle, application_name)
            .about(Default::default())
            .separator()
            .services()
            .separator()
            .hide()
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;

        main_menu = main_menu.item(&macos_application_menu);
    }

    let file_menu = SubmenuBuilder::new(&handle, "File").build()?;

    let edit_menu = SubmenuBuilder::new(&handle, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .separator()
        .build()?;

    main_menu = main_menu.items(&[&file_menu, &edit_menu]);

    app.set_menu(main_menu.build()?)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let application = tauri::Builder::default()
        .plugin(tauri_plugin_cache::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(initialize_main_menu)
        .invoke_handler(tauri::generate_handler![
            open_file_dialog,
            read_csv,
            open_sending_mail_window,
            open_template_manager_window,
            read_receiver_csv_list,
            send_email_by_thread,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    application.run(|app_handle, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            let cache = app_handle.cache();
            let _ = cache.clear();
        }
    })
}
