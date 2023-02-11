#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{App, Manager, SystemTray};
use window_shadows::set_shadow;

use crate::config::Config;
use crate::core::clipboard;
use crate::core::database::SqliteDB;
use crate::core::sysopt;
use crate::core::tray;
mod cmds;
mod config;
mod core;
mod utils;

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            set_up(app);
            Ok(())
        })
        .system_tray(SystemTray::new())
        .on_system_tray_event(core::tray::Tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            cmds::get_common_config,
            cmds::set_common_config,
            cmds::change_language,
            cmds::change_record_limit,
            cmds::change_auto_launch,
            cmds::change_theme_mode,
            cmds::change_hotkeys,
            cmds::clear_data,
            cmds::insert_record,
            cmds::insert_if_not_exist,
            cmds::find_all_record,
            cmds::mark_favorite,
            cmds::find_by_key,
            cmds::delete_over_limit,
            cmds::write_to_clip,
        ])
        .build(tauri::generate_context!())
        .expect("error while build tauri application");

    app.run(|app_handle, e| match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit => {
            app_handle.exit(0);
        }
        _ => {}
    });
}

fn set_up(app: &mut App) {
    // Make the docker NOT to have an active app when started
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let window = app.get_window("main").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");
    core::handle::Handle::global().init(app.app_handle());
    log_err!(Config::init_config());
    log_err!(tray::Tray::update_systray(&app.app_handle()));
    log_err!(sysopt::Sysopt::global().init_launch());
    let _ = core::handle::Handle::refresh_global_shortcut();
    SqliteDB::init();
    clipboard::ClipboardWatcher::start();
}
