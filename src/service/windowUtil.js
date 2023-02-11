import { appWindow } from "@tauri-apps/api/window";

let closeWindowTimer = null;

export async function closeWindowLater(delay) {
  if (closeWindowTimer) {
    clearTimeout(closeWindowTimer);
    closeWindowTimer = null;
  }
  await appWindow.hide();
  // 延迟5秒如果未重新打开window则close
  closeWindowTimer = setTimeout(async () => {
    // 等待关闭window
    let visible = await appWindow.isVisible();
    if (!visible) {
      await appWindow.close();
    }
  }, delay);
}
