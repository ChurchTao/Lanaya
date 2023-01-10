use super::{CommonConfig, Draft};
use crate::{
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

#[test]
fn test_config() {
    log_err!(Config::init_config());
    let common = Config::common();
    println!("common: {:?}", common.data().enable_auto_launch);
    common.draft().enable_auto_launch = Some(false);
    common.apply();
    let res = common.latest().save_file();
    match res {
        Ok(_) => println!("ok"),
        Err(e) => println!("err: {}", e),
    }
}
