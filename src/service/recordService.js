import {
  clearData,
  insertIfNotExist,
  findAllRecord,
  markFavorite,
  saveTags as saveTagsCmd,
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

pub struct QueryReq {
    pub key: Option<String>,
    pub limit: Option<usize>,
    pub is_favorite: Option<bool>,
}
 */

async function selectPage(searchKey = "", isFavorite = undefined, limit = 300) {
  // 如果 searchKey 以f:开头，那么就是查询收藏的记录
  if (searchKey === "") {
    return await findAllRecord(limit);
  }
  if (searchKey.startsWith("f:")) {
    isFavorite = true;
    searchKey = searchKey.substring(2);
  }
  let query = {
    limit,
  };
  if (searchKey.startsWith("t:")) {
    query.tags = searchKey.substring(2).split(",").filter(Boolean);
  } else if (searchKey !== "") {
    query.key = searchKey;
  }
  if (isFavorite !== undefined) {
    query.is_favorite = isFavorite;
  }
  return await findByKey(query);
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

async function markFav(id) {
  await markFavorite(id);
}

async function saveTags(id, tags) {
  return await saveTagsCmd(id, tags);
}

export { selectPage, insertRecord, updateRecord, clearAll, markFav, saveTags };
