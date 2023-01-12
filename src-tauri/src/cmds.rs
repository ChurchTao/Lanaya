use crate::{
    config::{CommonConfig, Config},
    log_err,
};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn get_common_config() -> CmdResult<CommonConfig> {
    Ok(Config::common().data().clone())
}

#[tauri::command]
pub fn set_common_config(common_config: CommonConfig) -> CmdResult {
    Config::common().draft().patch_config(common_config);
    Config::common().apply();
    log_err!(Config::common().data().save_file());
    Config::common().data().notify();
    Ok(())
}
