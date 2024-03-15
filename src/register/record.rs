use fltk::button;
use fltk::prelude::{InputExt, WidgetExt};

use crate::global::click::{COUNT_RECORD_INPUT, EVENTS};

pub(crate) fn on_clear_record_button_clicked(button: &mut button::Button) {
    button.set_callback(|_| unsafe {
        EVENTS.clear();
        COUNT_RECORD_INPUT.set_value("0");
    });
}