use std::error;

use rusqlite::{Connection, params};

use crate::config;
use crate::persistence::entity::record::Record;

pub(crate) fn add_record(record: Record) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(config::get_database_path())?;
    let events = serde_json::to_string(&record.events)?;
    conn.execute(
        "INSERT INTO record (title, events) VALUES (?1, ?2)",
        params![record.title, events],
    )?;
    Ok(())
}

pub(crate) fn list_records() -> Result<Vec<Record>, Box<dyn error::Error>> {
    let conn = Connection::open(config::get_database_path())?;
    let mut stmt = conn.prepare("SELECT id, title, events FROM record")?;
    let records_iter = stmt.query_map([], |row| {
        Ok(Record {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            events: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or(Vec::new()),
        })
    })?;

    let mut records = Vec::new();
    for record in records_iter {
        records.push(record?);
    }
    Ok(records)
}

pub(crate) fn delete_record(id: i32) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(config::get_database_path())?;
    conn.execute("DELETE FROM record WHERE id = ?1", params![id])?;
    Ok(())
}

#[test]
fn test_add_record() {
    use crate::persistence::entity::event::Event;
    let mut record = Record {
        id: None,
        title: "delete files".to_string(),
        events: Vec::new(),
    };
    record.events.push(Event::new(
        500,
        200,
        enigo::Button::Left,
        1,
        2000,
    ));
    assert!(add_record(record).is_ok());
}

#[test]
fn test_list_records() {
    let records = list_records().unwrap();
    assert_ne!(records.len(), 0);
}