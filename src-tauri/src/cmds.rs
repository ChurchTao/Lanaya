use crate::{
    config,
    config::{CommonConfig, Config},
    core::handle::Handle,
    log_err,
};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn get_common_config() -> CmdResult<CommonConfig> {
    Ok(Config::common().data().clone())
}

#[tauri::command]
pub fn set_common_config(config: CommonConfig) -> CmdResult {
    Config::common().draft().patch_config(config);
    Config::common().apply();
    log_err!(Config::common().data().save_file());

    // todo enable_auto_launch
    // todo hotkeys
    Handle::refresh_common_config();
    log_err!(Handle::update_systray());
    Ok(())
}

#[tauri::command]
pub async fn change_language(language: String) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        language: Some(language),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}
