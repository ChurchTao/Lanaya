use super::{CommonConfig, Draft};
use crate::{
    core::handle,
    core::sysopt,
    log_err,
    utils::{dirs, json_util},
};
use anyhow::Result;
use once_cell::sync::OnceCell;
use std::fs;

pub struct Config {
    common_config: Draft<CommonConfig>,
}

impl Config {
    pub fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();

        CONFIG.get_or_init(|| Config {
            common_config: Draft::from(CommonConfig::new()),
        })
    }

    pub fn common() -> Draft<CommonConfig> {
        Self::global().common_config.clone()
    }

    /// 初始化配置
    pub fn init_config() -> Result<()> {
        log_err!(dirs::app_home_dir().map(|app_dir| {
            if !app_dir.exists() {
                let _ = fs::create_dir_all(&app_dir);
            }
        }));
        log_err!(dirs::app_data_dir().map(|app_dir| {
            if !app_dir.exists() {
                let _ = fs::create_dir_all(&app_dir);
            }
        }));
        log_err!(dirs::app_data_img_dir().map(|app_dir| {
            if !app_dir.exists() {
                let _ = fs::create_dir_all(&app_dir);
            }
        }));
        log_err!(dirs::config_path().map(|path| {
            if !path.exists() {
                log_err!(json_util::save(&path, &CommonConfig::template()));
            }
        }));
        Ok(())
    }
}

/// 修改通用配置文件的入口
pub async fn modify_common_config(patch: CommonConfig) -> Result<()> {
    Config::common().draft().patch_config(patch.clone());

    let auto_launch = patch.enable_auto_launch;
    let auto_paste = patch.enable_auto_paste;
    let delete_confirm = patch.enable_delete_confirm;
    let language = patch.language;
    let theme_mode = patch.theme_mode;
    let record_limit = patch.record_limit;
    let hotkeys = patch.hotkeys;

    match {
        if auto_launch.is_some() {
            sysopt::Sysopt::global().update_launch()?;
        }

        if hotkeys.is_some() {
            let _ = handle::Handle::refresh_global_shortcut();
            handle::Handle::notice_to_window(handle::MsgTypeEnum::ChangeHotKeys, hotkeys)?;
        }

        if language.is_some() {
            handle::Handle::update_systray()?;
            handle::Handle::notice_to_window(handle::MsgTypeEnum::ChangeLanguage, language)?;
        }

        if theme_mode.is_some() {
            // todo send msg to frontend
        }

        if record_limit.is_some() {
            handle::Handle::notice_to_window(handle::MsgTypeEnum::ChangeRecordLimit, record_limit)?;
        }

        if auto_paste.is_some() {
            handle::Handle::notice_to_window(handle::MsgTypeEnum::ChangeAutoPaste, auto_paste)?;
        }

        if delete_confirm.is_some() {
            handle::Handle::notice_to_window(handle::MsgTypeEnum::ChangeDeleteConfirm, delete_confirm)?;
        }

        <Result<()>>::Ok(())
    } {
        Ok(()) => {
            Config::common().apply();
            Config::common().data().save_file()?;
            Ok(())
        }
        Err(err) => {
            Config::common().discard();
            Err(err)
        }
    }
}

#[test]
fn test_config() {
    log_err!(Config::init_config());
    let common = Config::common();
    println!("common: {:?}", common.data());
}
