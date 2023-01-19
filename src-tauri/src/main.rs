#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{App, Manager, SystemTray};
use tauri_plugin_sql::{Migration, MigrationKind};
use window_shadows::set_shadow;

use crate::config::Config;
use crate::core::tray;
mod cmds;
mod config;
mod core;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_up(app);
            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:lanaya_data.db", get_migrations())
                .build(),
        )
        .system_tray(SystemTray::new())
        .on_system_tray_event(core::tray::Tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            cmds::get_common_config,
            cmds::set_common_config,
            cmds::change_language
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn set_up(app: &mut App) {
    // Make the docker NOT to have an active app when started
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let window = app.get_window("main").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");
    core::handle::Handle::global().init(app.app_handle());
    log_err!(Config::init_config());
    log_err!(tray::Tray::update_systray(&app.app_handle()));
}

fn get_migrations() -> Vec<Migration> {
    let mut migrations = Vec::new();
    migrations.push(Migration {
        version: 1,
        description: "create record table",
        sql: include_str!("../migrations/record.sql"),
        kind: MigrationKind::Up,
    });
    migrations
}
