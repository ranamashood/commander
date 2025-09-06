use std::{fs, path::PathBuf};

use anyhow::Result;
use rusqlite::Connection;

fn get_db_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(std::env::var("HOME").unwrap());
    path.push(".config/commander");

    fs::create_dir_all(&path)?;
    path.push("data.db");

    Ok(path)
}

pub fn get_db() -> Result<Connection> {
    let db_path = get_db_path()?;
    let db = Connection::open(db_path)?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY,
            command TEXT NOT NULL,
            description TEXT
        )",
        (),
    )?;

    Ok(db)
}
