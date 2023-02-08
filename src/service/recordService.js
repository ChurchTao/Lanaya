import {
  clearData,
  insertIfNotExist,
  findAllRecord,
  markFavorite,
  findByKey,
  deleteOverLimit,
} from "./cmds";

/**
struct Record {
    pub id: u64,
    pub content: String,
    pub md5: String,
    pub create_time: u64,
    pub is_favorite: bool,
}
 */

async function selectPage(searchKey = "", limit = 300) {
  if (searchKey === "") {
    return await findAllRecord(limit);
  }
  return await findByKey(searchKey);
}

async function insertRecord(content, limit = 300) {
  let newRecord = {
    id: 0,
    content: content,
    md5: "",
    is_favorite: false,
    create_time: 0,
  };
  await insertIfNotExist(newRecord);
  removeOverLimit(limit);
}

/**
 * 删除超过limit的记录
 * @param {*} limit
 * @returns
 */
async function removeOverLimit(limit = 300) {
  await deleteOverLimit(limit);
}

async function updateRecord(record) {
  await insertIfNotExist(record);
}

async function clearAll() {
  return await clearData();
}

export { selectPage, insertRecord, updateRecord, clearAll };
