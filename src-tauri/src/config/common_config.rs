use crate::utils::{dirs, json_util};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct CommonConfig {
    // i18n
    pub language: Option<String>,
    /// `light` or `dark` or  todo `system`
    pub theme_mode: Option<String>,
    /// can the app auto startup
    pub enable_auto_launch: Option<bool>,
    /// hotkey map
    /// format: {func},{key}
    pub hotkeys: Option<Vec<String>>,
    // pub font_family: Option<String>,
    // pub font_size: Option<String>,
}

impl CommonConfig {
    pub fn new() -> Self {
        match dirs::config_path().and_then(|path| json_util::read::<CommonConfig>(&path)) {
            Ok(config) => config,
            Err(_) => Self::template(),
        }
    }

    pub fn template() -> Self {
        Self {
            language: match cfg!(feature = "default-meta") {
                false => Some("en".into()),
                true => Some("zh".into()),
            },
            theme_mode: Some("light".into()),
            enable_auto_launch: Some(false),
            ..Self::default()
        }
    }

    /// Save IVerge App Config
    pub fn save_file(&self) -> Result<()> {
        json_util::save(&dirs::config_path()?, &self)
    }

    /// patch config
    /// only save to file
    pub fn patch_config(&mut self, patch: CommonConfig) {
        macro_rules! patch {
            ($key: tt) => {
                if patch.$key.is_some() {
                    self.$key = patch.$key;
                }
            };
        }
        patch!(language);
        patch!(theme_mode);
        patch!(enable_auto_launch);
        patch!(hotkeys);
    }
}
