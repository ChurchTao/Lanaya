import Database from "tauri-plugin-sql-api";

let db = null;

async function connect() {
  try {
    db = await Database.load("sqlite:lanaya_data.db");
    init(db);
    return db;
  } catch (e) {
    console.log(e);
  }
}

async function init(db) {
  await db.execute(
    "create table if not exists record(id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,content TEXT,md5 VARCHAR(200) DEFAULT '',create_time INTEGER);"
  );
}

async function getConnect() {
  if (db) {
    return db;
  } else {
    return await connect();
  }
}

export { getConnect };
