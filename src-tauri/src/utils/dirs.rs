use anyhow::Result;
use std::path::PathBuf;
use tauri::api::path::home_dir;

static APP_DIR: &str = "lanaya";
static CONFIG_FILE: &str = "config.json";

/// get the app home dir
pub fn app_home_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        use tauri::utils::platform::current_exe;

        let app_exe = current_exe()?;
        let app_exe = dunce::canonicalize(app_exe)?;
        let app_dir = app_exe
            .parent()
            .ok_or(anyhow::anyhow!("failed to get the portable app dir"))?;
        Ok(PathBuf::from(app_dir).join(".config").join(APP_DIR))

    }

    #[cfg(not(target_os = "windows"))]
    Ok(home_dir()
        .ok_or(anyhow::anyhow!("failed to get the app home dir"))?
        .join(".config")
        .join(APP_DIR))
}

/// logs dir
#[allow(unused)]
pub fn app_logs_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("logs"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(CONFIG_FILE))
}

#[allow(unused)]
pub fn app_data_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("data"))
}

pub fn app_data_img_dir() -> Result<PathBuf> {
    Ok(app_data_dir()?.join("img"))
}

#[test]
fn test() {
    println!("app_home_dir: {:?}", app_home_dir());
    println!("app_logs_dir: {:?}", app_logs_dir());
    println!("config_path: {:?}", config_path());
    println!("app_data_dir: {:?}", app_data_dir());
}
