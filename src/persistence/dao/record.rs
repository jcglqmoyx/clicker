use std::error;

use rusqlite::{Connection, params};

use crate::config;
use crate::persistence::entity::record::Record;

pub(crate) fn add_record(record: Record) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(config::SQLITE_FILE_PATH)?;
    let events = serde_json::to_string(&record.events)?;
    let x = conn.execute(
        "INSERT INTO record (title, events) VALUES (?1, ?2)",
        params![record.title, events],
    )?;
    println!("{}", x);
    println!("Record added successfully");
    Ok(())
}

pub(crate) fn count_records() -> Result<i64, Box<dyn error::Error>> {
    let conn = Connection::open(config::SQLITE_FILE_PATH)?;
    let count = conn.query_row("SELECT COUNT(*) FROM record", [], |row| row.get(0))?;
    Ok(count)
}

pub(crate) fn list_records() -> Result<Vec<Record>, Box<dyn error::Error>> {
    let conn = Connection::open(config::SQLITE_FILE_PATH)?;
    let mut stmt = conn.prepare("SELECT id, title, description, events FROM record")?;
    let records_iter = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(0)?,
            title: row.get(1)?,
            events: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or(Vec::new()),
        })
    })?;

    let mut records = Vec::new();
    for record in records_iter {
        records.push(record?);
    }
    Ok(records)
}

#[test]
fn test_add_record() {
    let record = Record {
        id: None,
        title: "title".to_string(),
        events: Vec::new(),
    };
    assert!(add_record(record).is_ok());
}

#[test]
fn test_count_records() {
    let count = count_records().unwrap();
    println!("{}", count);
    assert_ne!(count, 0);
}

#[test]
fn test_list_records() {
    let records = list_records().unwrap();
    println!("{:#?}", records);
    assert_ne!(records.len(), 0);
}