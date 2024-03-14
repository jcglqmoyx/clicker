#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) id: i32,
    pub(crate) language: String,
    pub(crate) enable_notification: i32,
}

impl Config {
    pub(crate) fn new(id: i32, language: String, enable_notification: i32) -> Self {
        Config {
            id,
            language,
            enable_notification,
        }
    }
}
