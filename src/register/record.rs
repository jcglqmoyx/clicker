use fltk::button;
use fltk::prelude::{GroupExt, InputExt, WidgetExt};

use crate::global::click::{COUNT_RECORD_INPUT, EVENTS};
use crate::utils::audio::play_audio;

pub(crate) fn on_clear_record_button_clicked(button: &mut button::Button) {
    button.set_callback(|_| unsafe {
        EVENTS.clear();
        COUNT_RECORD_INPUT.set_value("0");
        play_audio("./resources/audio/clear_record_sound.mp3", 300);
    });
}

// pub(crate) fn on_save_record_button_clicked(button: &mut Button) {

// }