use anyhow::Result;
use std::path::PathBuf;
use tauri::{
    api::path::{home_dir, resource_dir},
    Env, PackageInfo,
};

static APP_DIR: &str = "lanaya";
static CONFIG_FILE: &str = "config.json";
static mut RESOURCE_DIR: Option<PathBuf> = None;

/// portable flag
#[allow(unused)]
static mut PORTABLE_FLAG: bool = false;
#[allow(unused)]
pub static mut APP_VERSION: &str = "v0.0.1";

/// initialize portable flag
#[cfg(target_os = "windows")]
pub unsafe fn init_portable_flag() -> Result<()> {
    use tauri::utils::platform::current_exe;

    let exe = current_exe()?;

    if let Some(dir) = exe.parent() {
        let dir = PathBuf::from(dir).join(".config/PORTABLE");

        if dir.exists() {
            PORTABLE_FLAG = true;
        }
    }

    Ok(())
}

/// get the verge app home dir
pub fn app_home_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    unsafe {
        use tauri::utils::platform::current_exe;

        if !PORTABLE_FLAG {
            Ok(home_dir()
                .ok_or(anyhow::anyhow!("failed to get app home dir"))?
                .join(".config")
                .join(APP_DIR))
        } else {
            let app_exe = current_exe()?;
            let app_exe = dunce::canonicalize(app_exe)?;
            let app_dir = app_exe
                .parent()
                .ok_or(anyhow::anyhow!("failed to get the portable app dir"))?;
            Ok(PathBuf::from(app_dir).join(".config").join(APP_DIR))
        }
    }

    #[cfg(not(target_os = "windows"))]
    Ok(home_dir()
        .ok_or(anyhow::anyhow!("failed to get the app home dir"))?
        .join(".config")
        .join(APP_DIR))
}

/// get the resources dir
#[allow(unused)]
pub fn app_resources_dir(package_info: &PackageInfo) -> Result<PathBuf> {
    let res_dir = resource_dir(package_info, &Env::default())
        .ok_or(anyhow::anyhow!("failed to get the resource dir"))?
        .join("resources");

    unsafe {
        RESOURCE_DIR = Some(res_dir.clone());

        let ver = package_info.version.to_string();
        let ver_str = format!("v{ver}");
        APP_VERSION = Box::leak(Box::new(ver_str));
    }

    Ok(res_dir)
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
pub fn app_res_dir() -> Result<PathBuf> {
    unsafe {
        Ok(RESOURCE_DIR
            .clone()
            .ok_or(anyhow::anyhow!("failed to get the resource dir"))?)
    }
}

#[allow(unused)]
pub fn path_to_str(path: &PathBuf) -> Result<&str> {
    let path_str = path
        .as_os_str()
        .to_str()
        .ok_or(anyhow::anyhow!("failed to get path from {:?}", path))?;
    Ok(path_str)
}

#[test]
fn test() {
    println!("app_home_dir: {:?}", app_home_dir());
    println!("app_logs_dir: {:?}", app_logs_dir());
    println!("config_path: {:?}", config_path());
}
