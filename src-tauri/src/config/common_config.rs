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
    pub record_limit: Option<u32>,
}

impl CommonConfig {
    pub fn new() -> Self {
        match dirs::config_path().and_then(|path| json_util::read::<CommonConfig>(&path)) {
            Ok(config) => {
                // 先拿 template
                // 再拿 config 去覆盖
                let mut template = Self::template();
                template.merge(config);
                template
            }
            Err(_) => Self::template(),
        }
    }

    pub fn template() -> Self {
        Self {
            language: match cfg!(feature = "default-meta") {
                false => Some("zh".into()),
                true => Some("en".into()),
            },
            theme_mode: Some("light".into()),
            enable_auto_launch: Some(false),
            record_limit: Some(100),
            ..Self::default()
        }
    }

    pub fn save_file(&self) -> Result<()> {
        json_util::save(&dirs::config_path()?, &self)
    }

    pub fn merge(&mut self, other: Self) {
        if let Some(language) = other.language {
            self.language = Some(language);
        }
        if let Some(theme_mode) = other.theme_mode {
            self.theme_mode = Some(theme_mode);
        }
        if let Some(enable_auto_launch) = other.enable_auto_launch {
            self.enable_auto_launch = Some(enable_auto_launch);
        }
        if let Some(record_limit) = other.record_limit {
            self.record_limit = Some(record_limit);
        }
        if let Some(hotkeys) = other.hotkeys {
            self.hotkeys = Some(hotkeys);
        }
    }

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
        patch!(record_limit);
    }
}
