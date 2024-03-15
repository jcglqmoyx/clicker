use fltk::button::Button;
use fltk::prelude::WidgetBase;
use once_cell::sync::Lazy;

pub(crate) static mut LAST_TIME_KEY_PRESSED: u64 = 0;

// 0: stopped click
// 1: clicking
// 2: recording
pub(crate) static mut STATUS_OF_CLICKER: i32 = 0;

pub(crate) static mut STATUS_BUTTON: Lazy<Button> = Lazy::new(|| Button::new(50, 300, 60, 30, "Stopped clicking"));