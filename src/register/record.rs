use fltk::button;
use fltk::button::Button;
use fltk::input::Input;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::global::click::{COUNT_RECORD_INPUT, EVENTS};
use crate::utils::audio::play_audio;

pub(crate) fn on_clear_record_button_clicked(button: &mut button::Button) {
    button.set_callback(|_| unsafe {
        EVENTS.clear();
        COUNT_RECORD_INPUT.set_value("0");
        play_audio("./resources/audio/clear_record_sound.mp3", 300);
    });
}

pub(crate) fn on_save_record_button_clicked(button: &mut Button) {
    button.set_callback(move |_| {
        let mut save_window = Window::new(200, 200, 300, 200, "Save");
        let mut title_input = Input::new(20, 20, 260, 30, "Title:");
        let mut description_input = Input::new(20, 60, 260, 30, "Description:");
        let mut content_input = Input::new(20, 100, 260, 30, "Content:");
        let mut save_button = Button::new(80, 150, 60, 30, "Save");
        let mut cancel_button = Button::new(160, 150, 60, 30, "Cancel");

        save_button.set_callback({
            let mut save_window = save_window.clone();
            move |_| {
                let title = title_input.value();
                let description = description_input.value();
                let content = content_input.value();

                // save_to_database(&title, &description, &content);

                save_window.hide();
            }
        });

        cancel_button.set_callback({
            let mut save_window = save_window.clone();
            move |_| {
                save_window.hide();
            }
        });

        save_window.end();
        save_window.show();
    });
}