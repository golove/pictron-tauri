use chrono::format::format;
// database.rs
use rusqlite::{Connection, Result, Transaction};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgDetail {
    pub src: String,
    pub aspect_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub srcs: Vec<ImgDetail>,
    pub star: u8,
    pub collect: bool,
    pub download: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeName {
    name: String,
    value: bool,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_file: &str) -> Result<Self> {
        let conn = Connection::open(db_file)?;
        // 创建 picture 表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pictures (
              id              INTEGER PRIMARY KEY,
              title             TEXT NOT NULL,
              url              TEXT NOT NULL,
              srcs             TEXT NOT NULL,
              star             INTEGER NOT NULL,
              collect          BOOLEAN NOT NULL,
              download         BOOLEAN NOT NULL,
              deleted           BOOLEAN NOT NULL
              )",
            [],
        )?;
        Ok(Database { conn })
    }
    pub fn open(db_file: &str) -> Result<Self> {
        let conn = Connection::open(db_file)?;
        Ok(Database { conn })
    }

    pub fn insert_picture(&self, picture: Picture) -> Result<()> {
        let serialized_srcs = to_string(&picture.srcs).unwrap_or_default();
        self.conn.execute(
            "INSERT INTO pictures (id, title, url, srcs, star, collect, download, deleted) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&picture.id as &dyn rusqlite::ToSql, &picture.title, &picture.url, &serialized_srcs, &picture.star, &picture.collect, &picture.download, &picture.deleted],
        )?;
        Ok(())
    }


    // update picture
    pub fn update_picture(self: &Self,sql: &str,) -> Result<()> {
        let mut stmt = self.conn.prepare(
            sql,
        )?;
        let _ = stmt.execute([])?;


        Ok(())
    }





    pub fn select_picture(&self, sql: &str) -> Result<Vec<Picture>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let srcs_str: String = row.get(3)?;
            let srcs: Vec<ImgDetail> = from_str(&srcs_str).unwrap_or_default();
            Ok(Picture {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                srcs,
                star: row.get(4)?,
                collect: row.get(5)?,
                download: row.get(6)?,
                deleted: row.get(7)?,
            })
        })?;

        let mut collected_pictures = Vec::new();
        for row in rows {
            collected_pictures.push(row?);
        }

        Ok(collected_pictures)
    }

    

}
