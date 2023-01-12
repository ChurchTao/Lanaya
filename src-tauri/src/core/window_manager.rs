use tauri::{AppHandle, Manager};

pub enum WindowType {
    Config,
    // About,
}
struct WindowInfo {
    label: String,
    title: String,
    url: String,
    width: f64,
    height: f64,
    resizable: bool,
    // todo tauri no this API
    // minimizable: bool,
    fullscreenable: bool,
    always_on_top: bool,
}

impl WindowInfo {
    fn config() -> Self {
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
        }
    }
}

pub fn open_window(app_handle: &AppHandle, window_type: WindowType) {
    let window_info = match window_type {
        WindowType::Config => WindowInfo::config(),
        // WindowType::About => WindowInfo::about(),
    };

    let label = window_info.label.as_str();
    let title = window_info.title.as_str();
    let url = window_info.url.as_str();

    if let Some(window) = app_handle.get_window(label) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let new_window = tauri::window::WindowBuilder::new(
        app_handle,
        label.to_string(),
        tauri::WindowUrl::App(url.into()),
    )
    .title(title)
    .center()
    .visible(false)
    .resizable(window_info.resizable)
    .fullscreen(window_info.fullscreenable)
    .always_on_top(window_info.always_on_top)
    .inner_size(window_info.width, window_info.height)
    .build();
    match new_window {
        Ok(window) => {
            let _ = window.show();
            let _ = window.set_focus();
        }
        Err(e) => {
            println!("create_window error: {}", e);
        }
    }
}
