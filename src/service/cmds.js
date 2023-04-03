import { invoke } from "@tauri-apps/api/tauri";
import { sendNotice } from "@/service/msg";

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

export async function clearData() {
  return invoke("clear_data");
}

export async function insertRecord(record) {
  return invoke("insert_record", { record });
}

export async function insertIfNotExist(r) {
  return invoke("insert_if_not_exist", { r });
}

export async function findAllRecord() {
  return invoke("find_all_record");
}

export async function markFavorite(id) {
  return invoke("mark_favorite", { id });
}

export async function saveTags(id, tags) {
  return invoke("save_tags", { id, tags: tags.join(",") });
}

export async function findByKey(query) {
  return invoke("find_by_key", { query });
}

export async function deleteOverLimit(limit) {
  return invoke("delete_over_limit", { limit });
}

export async function writeToClip(id) {
  invoke("write_to_clip", { id });
  sendNotice("", "Copy!");
}

export async function deleteById(id) {
  return invoke("delete_by_id", { id });
}
