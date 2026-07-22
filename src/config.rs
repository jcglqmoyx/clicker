use std::path::PathBuf;

use dirs::home_dir;

pub(crate) fn get_database_path() -> PathBuf {
    let mut db_path = home_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push(".clicker.db");
    db_path
}

#[test]
fn test_get_database_path() {
    let db_path = get_database_path();
    assert_eq!(
        db_path.file_name().and_then(|s| s.to_str()),
        Some(".clicker.db")
    );
    assert!(db_path.is_absolute() || db_path.starts_with("."));
}