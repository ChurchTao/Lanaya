import { invoke } from "@tauri-apps/api/tauri";

export async function getCommonConfig() {
  return invoke("get_common_config");
}

export async function setCommonConfig(config) {
  return invoke("set_common_config", config);
}
