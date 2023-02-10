use super::{
    tray::Tray,
    window_manager::{WindowInfo, WindowType},
};
use crate::{config::Config, log_err, utils::hotkey_util};
use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
}

pub enum MsgTypeEnum {
    ChangeLanguage,
    ChangeRecordLimit,
    ChangeHotKeys,
    ChangeClipBoard,
}

impl Handle {
    pub fn global() -> &'static Handle {
        static HANDLE: OnceCell<Handle> = OnceCell::new();

        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init(&self, app_handle: AppHandle) {
        *self.app_handle.lock() = Some(app_handle);
    }

    pub fn get_window(&self) -> Option<Window> {
        self.app_handle
            .lock()
            .as_ref()
            .map_or(None, |a| a.get_window("main"))
    }

    fn get_manager(&self) -> Result<impl GlobalShortcutManager> {
        let app_handle = self.app_handle.lock();
        if app_handle.is_none() {
            bail!("failed to get the hotkey manager");
        }
        Ok(app_handle.as_ref().unwrap().global_shortcut_manager())
    }

    pub fn refresh_common_config() {
        if let Some(window) = Self::global().get_window() {
            log_err!(window.emit("lanaya://refresh-common-config", "yes"));
        }
    }

    pub fn update_systray() -> Result<()> {
        let app_handle = Self::global().app_handle.lock();
        if app_handle.is_none() {
            bail!("update_systray unhandled error");
        }
        Tray::update_systray(app_handle.as_ref().unwrap())?;
        Ok(())
    }

    #[allow(unused)]
    pub fn update_systray_select_item() -> Result<()> {
        let app_handle = Self::global().app_handle.lock();
        if app_handle.is_none() {
            bail!("update_systray_select_item unhandled error");
        }
        Tray::update_select_item(app_handle.as_ref().unwrap())?;
        Ok(())
    }

    pub fn notice_to_window<S: Serialize + Clone>(msg_type: MsgTypeEnum, msg: S) -> Result<()> {
        let app_handle = Self::global().app_handle.lock();
        if app_handle.is_none() {
            bail!("notice_all_window unhandled error");
        }
        match msg_type {
            MsgTypeEnum::ChangeLanguage => {
                let window = app_handle.as_ref().unwrap().get_window("main");
                if window.is_some() {
                    log_err!(window.unwrap().emit("lanaya://change-language", msg));
                }
            }
            MsgTypeEnum::ChangeRecordLimit => {
                let window = app_handle.as_ref().unwrap().get_window("main");
                if window.is_some() {
                    log_err!(window.unwrap().emit("lanaya://change-record-limit", msg));
                }
            }
            MsgTypeEnum::ChangeHotKeys => {
                let window = app_handle.as_ref().unwrap().get_window("main");
                if window.is_some() {
                    log_err!(window.unwrap().emit("lanaya://change-hotkeys", msg));
                }
            }
            MsgTypeEnum::ChangeClipBoard => {
                let window = app_handle.as_ref().unwrap().get_window("main");
                if window.is_some() {
                    log_err!(window.unwrap().emit("lanaya://change-clipboard", msg));
                }
            }
        }
        Ok(())
    }

    pub fn refresh_global_shortcut() -> Result<()> {
        let hotkeys_new = Config::common().latest().hotkeys.clone();
        if let Some(hotkeys) = hotkeys_new {
            // hotkeys 中找到 global-shortcut 开头的第一个
            let mut global_shortcut = None;
            for hotkey in hotkeys {
                if hotkey.starts_with("global-shortcut") {
                    global_shortcut = Some(hotkey);
                    break;
                }
            }
            if let Some(global_shortcut) = global_shortcut {
                let hot_key_arr: Vec<&str> = global_shortcut.split(":").collect();
                let hot_key_arr: Vec<&str> = hot_key_arr[1].split("+").collect();
                let hot_key_arr: Vec<u32> = hot_key_arr
                    .iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
                // 目前只有一个 global-shortcut，此处可以改为循环
                let short_cut_name = hotkey_util::get_short_cut_name(hot_key_arr, true);
                let mut shortcut_manager = Self::global().get_manager()?;
                let _ = shortcut_manager.unregister_all();
                let _ = shortcut_manager.register(short_cut_name.as_str(), || {
                    Self::open_window(WindowType::Main)
                });
            }
        }
        Ok(())
    }

    pub fn open_window(window_type: WindowType) {
        tauri::async_runtime::spawn(async move {
            let binding = Self::global().app_handle.lock();
            let app_handle = binding.as_ref().unwrap();

            let window_info = match window_type {
                WindowType::Config => WindowInfo::config(),
                WindowType::Main => WindowInfo::main(),
            };

            let label = window_info.label.as_str();
            let title = window_info.title.as_str();
            let url = window_info.url.as_str();

            if let Some(window) = app_handle.get_window(label) {
                if window.is_visible().unwrap() {
                    let _ = window.close();
                    return;
                }
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
                return;
            }

            let new_window = tauri::window::WindowBuilder::new(
                app_handle,
                label.to_string(),
                tauri::WindowUrl::App(url.into()),
            )
            .title(title)
            .center()
            .visible(false)
            .resizable(window_info.resizable)
            .fullscreen(window_info.fullscreenable)
            .always_on_top(window_info.always_on_top)
            .inner_size(window_info.width, window_info.height)
            .transparent(window_info.transparent)
            .decorations(window_info.decorations)
            .skip_taskbar(window_info.skip_taskbar)
            .center()
            .build();
            match new_window {
                Ok(window) => {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                Err(e) => {
                    println!("create_window error: {}", e);
                }
            }
        });
    }
}
