use super::tray::Tray;
use crate::log_err;
use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Window};

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
}

pub enum MsgTypeEnum {
    ChangeLanguage,
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
        }
        Ok(())
    }
}
