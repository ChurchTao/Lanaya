import Database from "tauri-plugin-sql-api";
// 数据库文件存放位置
// import { appLocalDataDir } from "@tauri-apps/api/path";
// const appLocalDataDirPath = await appLocalDataDir();
// console.log(appLocalDataDirPath);
let db = null;

async function connect() {
  try {
    db = await Database.load("sqlite:lanaya_data.db");
    return db;
  } catch (e) {
    console.log(e);
  }
}

async function getConnect() {
  if (db) {
    return db;
  } else {
    return await connect();
  }
}

export { getConnect };
