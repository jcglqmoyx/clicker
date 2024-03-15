use std::error;

use rusqlite::{Connection, params};

use crate::config;
use crate::persistence::entity::record::Record;

pub(crate) fn add_record(record: Record) -> Result<(), Box<dyn error::Error>> {
    let events = serde_json::to_string(&record.events)?;
    let conn = Connection::open(config::SQLITE_FILE_PATH)?;

    conn.execute(
        "INSERT INTO record (id, title, description, events) VALUES (?1, ?2, ?3, ?4)",
        params![record.id, record.title, record.description, events],
    )?;
    Ok(())
}