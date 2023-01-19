import { invoke } from "@tauri-apps/api/tauri";

export async function getCommonConfig() {
  return invoke("get_common_config");
}

export async function setCommonConfig(config) {
  return invoke("set_common_config", { config });
}

export async function setLanguage(language) {
  return invoke("change_language", { language });
}

export async function setRecordLimit(limit) {
  return invoke("change_record_limit", { limit });
}

export async function setAutoLaunch(enable) {
  return invoke("change_auto_launch", { enable });
}

export async function setThemeMode(themeMode) {
  return invoke("change_theme_mode", { themeMode });
}

export async function setHotkeys(hotkeys) {
  return invoke("change_hotkeys", { hotkeys });
}
