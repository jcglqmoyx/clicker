#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) language: String,
    pub(crate) enable_notification: i32,
}

impl Config {
    pub(crate) fn new(language: String, enable_notification: i32) -> Self {
        Config {
            language,
            enable_notification,
        }
    }
}
