use std::error;
use std::path::Path;

use rusqlite::Connection;

pub(crate) fn init(db_path: &Path) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(db_path)?;
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
