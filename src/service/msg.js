import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";

export async function sendNotice(title, body) {
  let permissionGranted = await isPermissionGranted();
  console.log("permissionGranted", permissionGranted);
  if (!permissionGranted) {
    const permission = await requestPermission();
    console.log("permission", permission);
    permissionGranted = permission === "granted";
  }
  if (permissionGranted) {
    sendNotification({ title, body });
  }
}
