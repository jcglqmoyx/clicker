use std::error;

use rusqlite::Connection;

use crate::config;
use crate::persistence::entity::config::Config;

pub(crate) fn init() -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(config::SQLITE_FILE_PATH)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
                  id                  INTEGER PRIMARY KEY,
                  language            TEXT,
                  enable_notification INTEGER
                  )",
        [],
    )?;
    let mut stmt = conn.prepare("SELECT * FROM config")?;
    let configs = stmt.query_map([], |row| {
        Ok(Config::new(row.get(0)?, row.get(1)?))
    })?;

    if configs.count() == 0 {
        let user = Config::new(String::from("en"), 1);
        conn.execute(
            "INSERT INTO config (language, enable_notification) VALUES (?1, ?2)",
            &[&user.language, &user.enable_notification.to_string()],
        )?;
    }

    Ok(())
}