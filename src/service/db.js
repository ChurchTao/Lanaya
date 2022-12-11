import Database from "tauri-plugin-sql-api";
import { QueryResult } from "tauri-plugin-sql-api";
import { useStore } from "../stores/todos";
import md5 from "md5";

let db = null;

async function connect() {
  const s = useStore();
  try {
    db = await Database.load("sqlite:lanaya_data.db");
    s.setDbConnectionString(db.path);
    return db;
  } catch (e) {
    console.log(e);
    s.setErrorState(e);
  }
}

async function all() {
  const db = await connect();

  return await db.select("SELECT * FROM record");
}

async function create(content) {
  const newRecord = {
    md5: md5(content),
    content,
  };
  if (db) {
    await db.execute("INSERT INTO record (md5, content, completed) VALUES ($1,$2,$3)", [
      newRecord.md5,
      content,
    ]);
  } else {
    console.warn(`There is not a valid DB connection, adding TODO to local storage only`);
  }
  return newTodo;
}

async function update(todo: Todo): Promise<Todo> {
  await db.execute("UPDATE todos SET title = $1, completed = $2 WHERE id = $3", [
    todo.title,
    todo.completed,
    todo.id,
  ]);
  return todo;
}

async function remove(id: uuid): Promise<QueryResult> {
  return await db.execute("DELETE FROM todos WHERE id = $1", [id]);
}

export default {
  connect,
  all,
  create,
  update,
  remove,
};
