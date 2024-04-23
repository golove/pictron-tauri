use rusqlite::Connection;

use serde_json::{from_str, to_string};
use tauri::AppHandle;
use std::fs;

const CURRENT_DB_VERSION: u32 = 1;

use crate::Picture;
use crate::ImgDetail;


/// Initializes the database connection, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.
pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("pictron.sqlite");
    print!("sqlite_path: {}", sqlite_path.to_str().unwrap());
    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
  if existing_version < CURRENT_DB_VERSION {
    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

    tx.execute_batch(
      "CREATE TABLE IF NOT EXISTS pictures (
        id              INTEGER PRIMARY KEY,
        title             TEXT NOT NULL,
        url              TEXT NOT NULL,
        srcs             TEXT NOT NULL,
        star             INTEGER NOT NULL,
        collect          BOOLEAN NOT NULL,
        download         BOOLEAN NOT NULL,
        deleted           BOOLEAN NOT NULL
        );"
    )?;

    tx.commit()?;
  }

  

  Ok(())
}




pub fn insert_picture( db: &Connection,picture: Picture) ->Result<(), rusqlite::Error> {
  let serialized_srcs = to_string(&picture.srcs).unwrap_or_default();
  db.execute(
      "INSERT INTO pictures (id, title, url, srcs, star, collect, download, deleted) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
      &[&picture.id as &dyn rusqlite::ToSql, &picture.title, &picture.url, &serialized_srcs, &picture.star, &picture.collect, &picture.download, &picture.deleted],
  )?;
  Ok(())
}



pub fn select_picture(db: &Connection, sql: &str) -> Result<Vec<Picture>, rusqlite::Error> {
  let mut stmt = db.prepare(sql)?;
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



pub fn update_picture(db: &Connection, sql: &str) -> Result<(), rusqlite::Error> {
  let mut stmt = db.prepare(sql)?;
  let _ = stmt.execute([])?;

  Ok(())
}

