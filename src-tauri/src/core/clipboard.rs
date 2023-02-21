use super::database;
use super::handle::{self, MsgTypeEnum};
use crate::core::database::Record;
use crate::utils::{img_util, json_util, string_util};
use anyhow::Result;
use arboard::Clipboard;
use chrono::Duration;
use serde::{Deserialize, Serialize};

use std::thread;
const CHANGE_DEFAULT_MSG: &str = "ok";

pub struct ClipboardWatcher;

pub struct ClipBoardOprator;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ImageDataDB {
    pub width: usize,
    pub height: usize,
    pub base64: String,
}

impl ClipBoardOprator {
    pub fn set_text(text: String) -> Result<()> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text)?;
        Ok(())
    }

    pub fn set_image(data: ImageDataDB) -> Result<()> {
        let mut clipboard = Clipboard::new()?;
        let img_data = img_util::base64_to_rgba8(&data.base64).unwrap();
        clipboard.set_image(img_data)?;
        Ok(())
    }
}

impl ClipboardWatcher {
    pub fn start() {
        tauri::async_runtime::spawn(async {
            // 500毫秒检测一次剪切板变化
            let wait_millis = 500i64;
            let mut last_content = String::new();
            let mut last_img_md5 = String::new();
            let mut clipboard = Clipboard::new().unwrap();
            println!("start clipboard watcher");
            loop {
                let text = clipboard.get_text();
                let res = text.map(|text| {
                    let content_origin = text.clone();
                    let content = text.trim();
                    if !content.is_empty() && content != last_content {
                        let res = database::SqliteDB::new().insert_if_not_exist(Record {
                            content: content_origin,
                            data_type: "text".to_string(),
                            is_favorite: false,
                            ..Default::default()
                        });
                        match res {
                            Ok(_) => {
                                handle::Handle::notice_to_window(
                                    MsgTypeEnum::ChangeClipBoard,
                                    CHANGE_DEFAULT_MSG,
                                )
                                .unwrap();
                            }
                            Err(e) => {
                                println!("insert record error: {}", e);
                            }
                        }
                        last_content = content.to_string();
                    }
                });
                if let Err(e) = res {
                    println!("get text error: {}", e);
                }

                let img = clipboard.get_image();
                let res = img.map(|img| {
                    let img_md5 = string_util::md5_by_bytes(&img.bytes);
                    if img_md5 != last_img_md5 {
                        let base64 = img_util::rgba8_to_base64(&img);
                        let content_db = ImageDataDB {
                            width: img.width,
                            height: img.height,
                            base64,
                        };
                        let content = json_util::stringfy(&content_db).unwrap();
                        let res = database::SqliteDB::new().insert_if_not_exist(Record {
                            content,
                            data_type: "image".to_string(),
                            is_favorite: false,
                            ..Default::default()
                        });
                        match res {
                            Ok(_) => {
                                drop(img);
                                handle::Handle::notice_to_window(
                                    MsgTypeEnum::ChangeClipBoard,
                                    CHANGE_DEFAULT_MSG,
                                )
                                .unwrap();
                            }
                            Err(e) => {
                                println!("insert record error: {}", e);
                            }
                        }
                        last_img_md5 = img_md5;
                    }
                });
                if let Err(e) = res {
                    println!("get image error: {}", e);
                }
                thread::sleep(Duration::milliseconds(wait_millis).to_std().unwrap());
            }
        });
    }
}
