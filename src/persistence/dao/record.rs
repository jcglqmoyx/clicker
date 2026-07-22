use std::error;
use std::path::Path;

use rusqlite::{Connection, params};

use crate::persistence::entity::record::Record;

pub(crate) fn add_record(record: Record, db_path: &Path) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(db_path)?;
    let events = serde_json::to_string(&record.events)?;
    conn.execute(
        "INSERT INTO record (title, events) VALUES (?1, ?2)",
        params![record.title, events],
    )?;
    Ok(())
}

pub(crate) fn list_records(db_path: &Path) -> Result<Vec<Record>, Box<dyn error::Error>> {
    let conn = Connection::open(db_path)?;
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

pub(crate) fn delete_record(id: i32, db_path: &Path) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(db_path)?;
    conn.execute("DELETE FROM record WHERE id = ?1", params![id])?;
    Ok(())
}

#[cfg(test)]
fn temp_db_path() -> std::path::PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut p = std::env::temp_dir();
    p.push(format!("clicker_test_{}_{}.db", std::process::id(), n));
    p
}

#[test]
fn test_add_record() {
    use crate::persistence::dao::init;
    use crate::persistence::entity::event::Event;
    let path = temp_db_path();
    let _ = std::fs::remove_file(&path);
    init::init(&path).unwrap();
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
    let result = add_record(record, &path);
    let _ = std::fs::remove_file(&path);
    assert!(result.is_ok());
}

#[test]
fn test_list_records() {
    use crate::persistence::dao::init;
    use crate::persistence::entity::event::Event;
    let path = temp_db_path();
    let _ = std::fs::remove_file(&path);
    init::init(&path).unwrap();
    let mut record = Record {
        id: None,
        title: "r".to_string(),
        events: vec![Event::new(1, 2, enigo::Button::Left, 1, 10)],
    };
    assert!(add_record(record, &path).is_ok());
    let records = list_records(&path).unwrap();
    let _ = std::fs::remove_file(&path);
    assert_eq!(records.len(), 1);
}
