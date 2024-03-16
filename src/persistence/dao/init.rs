use std::error;

use rusqlite::Connection;

use crate::config;

pub(crate) fn init() -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(config::SQLITE_FILE_PATH)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS record (
                id                 INTEGER PRIMARY KEY AUTOINCREMENT,
                title              TEXT NOT NULL,
                events             TEXT NOT NULL
            )",
        [],
    )?;
    Ok(())
}