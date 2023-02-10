pub enum WindowType {
    Config,
    Main,
}
pub struct WindowInfo {
    pub label: String,
    pub title: String,
    pub url: String,
    pub width: f64,
    pub height: f64,
    pub resizable: bool,
    // todo tauri no this API
    // minimizable: bool,
    pub fullscreenable: bool,
    pub always_on_top: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub skip_taskbar: bool,
}

impl WindowInfo {
    pub fn main() -> Self {
        WindowInfo {
            label: "main".into(),
            title: "Lanaya".into(),
            url: "/".into(),
            width: 800.0,
            height: 600.0,
            resizable: false,
            // minimizable: false,
            fullscreenable: false,
            always_on_top: false,
            transparent: true,
            decorations: false,
            skip_taskbar: true,
        }
    }
    pub fn config() -> Self {
        WindowInfo {
            label: "config".into(),
            title: "设置".into(),
            url: "/config".into(),
            width: 640.0,
            height: 480.0,
            resizable: false,
            // minimizable: false,
            fullscreenable: false,
            always_on_top: false,
            transparent: false,
            decorations: true,
            skip_taskbar: false,
        }
    }
}
