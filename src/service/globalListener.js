import { listen } from "@tauri-apps/api/event";
import { getCommonConfig } from "./cmds";

export const listenLanguageChange = async (consumer) => {
  await listen("lanaya://change-language", async (event) => {
    consumer(event.payload);
  });
};
