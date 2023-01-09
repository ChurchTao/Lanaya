#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::SystemTray;
use tauri::{App, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};
use window_shadows::set_shadow;
mod config;
mod utils;

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let show = CustomMenuItem::new("show".to_string(), "唤起主界面")
        .accelerator("CommandOrControl+Shift+C");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口").accelerator("Esc");
    let quit = CustomMenuItem::new("quit".to_string(), "退出").accelerator("CommandOrControl+Q");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            set_up(app);
            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![greet])
        .plugin(TauriSql::default().add_migrations("sqlite:lanaya_data.db", get_migrations()))
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn set_up(app: &mut App) {
    // Make the docker NOT to have an active app when started
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let window = app.get_window("main").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");
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
