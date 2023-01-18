use super::{CommonConfig, Draft};
use crate::{
    core::handle,
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
    let language = patch.language;
    let theme_mode = patch.theme_mode;
    let record_limit = patch.record_limit;
    let hotkeys = patch.hotkeys;

    match {
        if auto_launch.is_some() {
            // todo change auto launch
            // todo send msg to frontend
        }
        if hotkeys.is_some() {
            // send msg to frontend
            // system tary hotkey modify
        }

        if language.is_some() {
            handle::Handle::update_systray()?;
            handle::Handle::notice_to_window(handle::MsgTypeEnum::ChangeLanguage, language)?;
        }

        if theme_mode.is_some() {
            // todo send msg to frontend
        }

        if record_limit.is_some() {
            // todo send msg to frontend
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
    common.draft().enable_auto_launch = Some(false);
    common.draft().record_limit = Some(300);
    common.apply();
    let res = common.latest().save_file();
    match res {
        Ok(_) => println!("ok"),
        Err(e) => println!("err: {}", e),
    }
}
