use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    // i18n
    pub language: Option<String>,

    /// `light` or `dark` or `system`
    pub theme_mode: Option<String>,
    /// can the app auto startup
    pub enable_auto_launch: Option<bool>,
    /// hotkey map
    /// format: {func},{key}
    pub hotkeys: Option<Vec<String>>,
    /// theme setting
    pub font_family: Option<String>,
    pub font_size: Option<String>,
}

impl Config {}

#[test]
fn test() {}
