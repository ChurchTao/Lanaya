use crate::{config::Config, log_err};
use anyhow::{anyhow, Result};
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::utils::platform::current_exe;

pub struct Sysopt {
    auto_launch: Arc<Mutex<Option<AutoLaunch>>>,
}

impl Sysopt {
    pub fn global() -> &'static Sysopt {
        static SYSOPT: OnceCell<Sysopt> = OnceCell::new();

        SYSOPT.get_or_init(|| Sysopt {
            auto_launch: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init_launch(&self) -> Result<()> {
        let enable = { Config::common().latest().enable_auto_launch.clone() };
        let enable = enable.unwrap_or(false);

        println!("enable auto launch: {}", enable);

        let app_exe = current_exe()?;
        let app_exe = dunce::canonicalize(app_exe)?;
        let app_name = app_exe
            .file_stem()
            .and_then(|f| f.to_str())
            .ok_or(anyhow!("failed to get file stem"))?;
        let app_path = app_exe
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("failed to get app_path"))?
            .to_string();
        #[cfg(target_os = "windows")]
        let app_path = format!("\"{app_path}\"");

        // use the /Applications/Lanaya.app path
        #[cfg(target_os = "macos")]
        let app_path = (|| -> Option<String> {
            let path = std::path::PathBuf::from(&app_path);
            let path = path.parent()?.parent()?.parent()?;
            let extension = path.extension()?.to_str()?;
            match extension == "app" {
                true => Some(path.as_os_str().to_str()?.to_string()),
                false => None,
            }
        })()
        .unwrap_or(app_path);
        println!("app_path: {}", app_path);

        let auto = AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(&app_path)
            .build()?;

        match auto.is_enabled() {
            Ok(true) => {
                if enable {
                    return Ok(());
                }
            }
            _ => {}
        }

        // macos每次启动都更新登录项，避免重复设置登录项
        #[cfg(target_os = "macos")]
        let _ = auto.disable();

        if enable {
            auto.enable()?;
        }
        *self.auto_launch.lock() = Some(auto);

        Ok(())
    }

    pub fn update_launch(&self) -> Result<()> {
        let auto_launch = self.auto_launch.lock();

        if auto_launch.is_none() {
            drop(auto_launch);
            return self.init_launch();
        }
        let enable = { Config::common().latest().enable_auto_launch.clone() };
        let enable = enable.unwrap_or(false);
        let auto_launch = auto_launch.as_ref().unwrap();

        match enable {
            true => auto_launch.enable()?,
            false => log_err!(auto_launch.disable()), // 忽略关闭的错误
        };

        Ok(())
    }

    /// todo listen clipboard loop
    #[allow(unused)]
    pub fn init_clipboard_listener(&self) {}
}
