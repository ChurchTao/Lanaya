create table if not exists record
(
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    content     TEXT,
    md5         VARCHAR(200) DEFAULT '',
    create_time INTEGER,
    is_favorite INTEGER DEFAULT 0
);