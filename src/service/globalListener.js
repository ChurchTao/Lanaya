import { listen } from "@tauri-apps/api/event";
// import { getCommonConfig } from "./cmds";

export const listenLanguageChange = async (consumer) => {
  const unListen = await listen("lanaya://change-language", async (event) => {
    consumer(event.payload);
  });
  return unListen;
};

export const listenRecordLimitChange = async (consumer) => {
  const unListen = await listen(
    "lanaya://change-record-limit",
    async (event) => {
      consumer(event.payload);
    }
  );
  return unListen;
};

export const listenHotkeysChange = async (consumer) => {
  const unListen = await listen("lanaya://change-hotkeys", async (event) => {
    consumer(event.payload);
  });
  return unListen;
};

export const listenClipboardChange = async (consumer) => {
  const unListen = await listen("lanaya://change-clipboard", async (event) => {
    consumer(event.payload);
  });
  return unListen;
};

export const listenAutoPasteChange = async (consumer) => {
  const unListen = await listen("lanaya://change-auto-paste", async (event) => {
    consumer(event.payload);
  });
  return unListen;
};

export const listenWindowBlur = async (consumer) => {
  const unlistenBlur = await listen("tauri://blur", async (event) => {
    consumer(event);
  });
  return unlistenBlur;
};
