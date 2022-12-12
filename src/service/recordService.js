import { getConnect } from "./base/db";
import md5 from "md5";

async function selectPage(searchKey = "", limit = 100) {
  searchKey = `%${searchKey}%`;
  let db = await getConnect();
  return await db.select("SELECT * FROM record where content like $1 order by id desc limit $2", [
    searchKey,
    limit,
  ]);
}

async function insertRecord(content) {
  let newRecord = {
    content: content,
    md5: md5(content),
  };
  let db = await getConnect();
  let res = await db.execute("INSERT INTO record (content, md5, create_time) VALUES ($1,$2,$3)", [
    newRecord.content,
    newRecord.md5,
    new Date().getTime(),
  ]);
  console.log("insert success!", res);
}

async function updateRecord(record) {
  let db = await getConnect();
  return await db.execute(
    "UPDATE record SET content = $1, md5 = $2, create_time = $3 WHERE id = $4",
    [record.content, record.md5, new Date().getTime(), record.id]
  );
}

async function removeById(id) {
  let db = await getConnect();
  return await db.execute("DELETE FROM record WHERE id = $1", [id]);
}

export { selectPage, insertRecord, updateRecord, removeById };
