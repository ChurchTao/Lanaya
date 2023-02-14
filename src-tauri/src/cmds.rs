use crate::{
    config,
    config::{CommonConfig, Config},
    core::{
        clipboard::{ClipBoardOprator, ImageDataDB},
        database::{QueryReq, Record, SqliteDB},
        handle::Handle,
    },
    log_err,
    utils::json_util,
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

#[tauri::command]
pub async fn change_record_limit(limit: u32) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        record_limit: Some(limit),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_auto_launch(enable: bool) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        enable_auto_launch: Some(enable),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_theme_mode(theme_mode: String) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        theme_mode: Some(theme_mode),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub async fn change_hotkeys(hotkeys: Vec<String>) -> CmdResult {
    let _ = config::modify_common_config(CommonConfig {
        hotkeys: Some(hotkeys),
        ..CommonConfig::default()
    })
    .await;
    Ok(())
}

#[tauri::command]
pub fn clear_data() -> bool {
    match SqliteDB::new().clear_data() {
        Ok(()) => true,
        Err(e) => {
            println!("{}", format!("清除失败:{}", e));
            false
        }
    }
}

#[tauri::command]
pub fn insert_record(r: Record) -> bool {
    match SqliteDB::new().insert_record(r) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn insert_if_not_exist(r: Record) -> bool {
    match SqliteDB::new().insert_if_not_exist(r) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn find_all_record() -> Vec<Record> {
    SqliteDB::new().find_all().unwrap()
}

#[tauri::command]
pub fn mark_favorite(id: u64) -> bool {
    match SqliteDB::new().mark_favorite(id) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn delete_by_id(id: u64) -> bool {
    match SqliteDB::new().delete_by_id(id) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn find_by_key(query: QueryReq) -> Vec<Record> {
    SqliteDB::new().find_by_key(query).unwrap()
}

#[tauri::command]
pub fn delete_over_limit(limit: usize) -> bool {
    match SqliteDB::new().delete_over_limit(limit) {
        Ok(_i) => true,
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}

#[tauri::command]
pub fn write_to_clip(id: u64) -> bool {
    let record = SqliteDB::new().find_by_id(id);
    match record {
        Ok(r) => {
            if r.data_type == "text" {
                let _ = ClipBoardOprator::set_text(r.content);
            } else if r.data_type == "image" {
                let image_data: ImageDataDB = json_util::parse(&r.content).unwrap();
                let _ = ClipBoardOprator::set_image(image_data);
            }
            true
        }
        Err(e) => {
            println!("err:{}", e);
            false
        }
    }
}
