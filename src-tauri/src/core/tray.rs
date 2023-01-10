use crate::config::{CommonConfig, Config};
use anyhow::Result;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

use super::handle;

pub struct Tray {}

impl Tray {
    pub fn tray_menu(app_handle: &AppHandle) -> SystemTrayMenu {
        let zh = { Config::common().latest().language == Some("zh".into()) };
        let version = app_handle.package_info().version.to_string();
        if zh {
            SystemTrayMenu::new()
                .add_item(
                    CustomMenuItem::new("open_window", "显示界面")
                        .accelerator("CommandOrControl+Shift+C"),
                )
                .add_item(CustomMenuItem::new("hide_window", "隐藏界面").accelerator("Esc"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_submenu(SystemTraySubmenu::new(
                    "语言",
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("language_zh", "简体中文"))
                        .add_item(CustomMenuItem::new("language_en", "English")),
                ))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("app_version", format!("版本 {version}")).disabled())
                .add_item(CustomMenuItem::new("quit", "退出").accelerator("CmdOrControl+Q"))
        } else {
            SystemTrayMenu::new()
                .add_item(
                    CustomMenuItem::new("open_window", "Show Window")
                        .accelerator("CommandOrControl+Shift+C"),
                )
                .add_item(CustomMenuItem::new("hide_window", "Hide Window").accelerator("Esc"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_submenu(SystemTraySubmenu::new(
                    "Language",
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("language_zh", "简体中文"))
                        .add_item(CustomMenuItem::new("language_en", "English")),
                ))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(
                    CustomMenuItem::new("app_version", format!("Version {version}")).disabled(),
                )
                .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q"))
        }
    }

    pub fn update_systray(app_handle: &AppHandle) -> Result<()> {
        app_handle
            .tray_handle()
            .set_menu(Tray::tray_menu(app_handle))?;
        Tray::update_select_item(app_handle)?;
        Ok(())
    }

    pub fn update_select_item(app_handle: &AppHandle) -> Result<()> {
        let language = { Config::common().latest().language.clone() };

        let tray = app_handle.tray_handle();

        if let Some(language) = language {
            if language == "zh" {
                let _ = tray.get_item("language_zh").set_selected(true);
                let _ = tray.get_item("language_en").set_selected(false);
            } else {
                let _ = tray.get_item("language_en").set_selected(true);
                let _ = tray.get_item("language_zh").set_selected(false);
            }
        }
        Ok(())
    }

    pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
        match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open_window" => {
                    let window = app_handle.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "hide_window" => {
                    let window = app_handle.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "language_zh" => change_language("zh".into()),
                "language_en" => change_language("en".into()),
                "quit" => {
                    app_handle.exit(0);
                    std::process::exit(0);
                }
                _ => {}
            },
            #[cfg(target_os = "windows")]
            SystemTrayEvent::LeftClick { .. } => {
                let window = app_handle.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        }
    }
}

// 切换模式语言
fn change_language(mode: String) {
    tauri::async_runtime::spawn(async move {
        match patch(CommonConfig {
            language: Some(mode),
            ..CommonConfig::default()
        })
        .await
        {
            Ok(_) => handle::Handle::refresh_common_config(),
            Err(err) => println!("change_language: {}", err),
        }
    });
}

async fn patch(config: CommonConfig) -> Result<()> {
    Config::common().draft().patch_config(config.clone());
    let language = config.language;
    if language.is_some() {
        handle::Handle::update_systray()?;
        Config::common().apply();
        Config::common().data().save_file()?;
    }
    Ok(())
}
