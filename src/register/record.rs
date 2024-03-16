use fltk::{app, button};
use fltk::button::Button;
use fltk::enums::Event;
use fltk::frame::Frame;
use fltk::input::{Input, IntInput};
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::global::click::{COUNT_RECORD_INPUT, EVENTS};
use crate::global::mode::ENABLE_SOUND_EFFECT;
use crate::persistence::dao::record;
use crate::persistence::dao::record::{add_record, list_records};
use crate::persistence::entity::record::Record;
use crate::utils::audio::play_audio;

static mut SAVE_RECORD_WINDOW_OPENED: bool = false;

pub(crate) fn on_clear_record_button_clicked(button: &mut button::Button) {
    button.set_callback(|_| unsafe {
        EVENTS.clear();
        COUNT_RECORD_INPUT.set_value("0");
        if ENABLE_SOUND_EFFECT {
            play_audio("./resources/audio/clear_record_sound.mp3", 300);
        }
    });
}

pub(crate) fn on_save_record_button_clicked(button: &mut Button) {
    button.set_callback(move |_| unsafe {
        if SAVE_RECORD_WINDOW_OPENED {
            return;
        }
        SAVE_RECORD_WINDOW_OPENED = true;
        let mut save_record_window = Window::new(200, 200, 500, 200, "Save");

        let mut x = 0;
        let mut y = 0;
        save_record_window.set_callback(move |w| {
            match app::event() {
                Event::Push => {
                    let coords = app::event_coords();
                    x = coords.0 - w.x();
                    y = coords.1 - w.y();
                }
                Event::Drag => {
                    let coords = app::event_coords();
                    w.set_pos(coords.0 - x, coords.1 - y);
                }
                _ => {}
            }
        });

        let title_input = Input::new(100, 20, 260, 30, "Title:");
        let description_input = Input::new(100, 60, 260, 30, "Description:");
        let mut show_record_count_input = IntInput::new(100, 100, 260, 30, "Records:");
        show_record_count_input.set_value(&EVENTS.len().to_string());
        show_record_count_input.deactivate();

        let mut save_button = Button::new(80, 150, 60, 30, "Save");
        let mut cancel_button = Button::new(160, 150, 60, 30, "Cancel");

        save_button.set_callback({
            let mut save_window = save_record_window.clone();
            move |_| {
                let title = title_input.value();
                let description = description_input.value();

                if title.is_empty() || description.is_empty() || EVENTS.is_empty() {
                    return;
                }

                let record = Record::new(title, EVENTS.clone());
                let _ = add_record(record);

                if ENABLE_SOUND_EFFECT {
                    play_audio("./resources/audio/save_record_success.mp3", 500);
                }


                save_window.hide();
                SAVE_RECORD_WINDOW_OPENED = false;
            }
        });

        cancel_button.set_callback({
            let mut save_window = save_record_window.clone();
            move |_| {
                save_window.hide();
                SAVE_RECORD_WINDOW_OPENED = false;
            }
        });

        save_record_window.end();
        save_record_window.show();
    });
}

pub(crate) fn on_load_record_button_clicked(button: &mut Button) {
    button.set_callback(|_| {
        let mut load_record_window = Window::new(200, 200, 400, 300, "Load");

        let records = list_records().unwrap();

        let mut scroll = fltk::group::Scroll::new(0, 0, 400, 300, "");

        for (i, record) in records.into_iter().enumerate() {
            let frame = Frame::new(20, 30 + 30 * i as i32, 200, 30, record.title.as_str());
            let load_record_button = Button::new(200, 30 + 30 * i as i32, 80, 30, "Load");
            let mut delete_record_button = Button::new(290, 30 + 30 * i as i32, 80, 30, "Delete");

            scroll.add(&frame);
            scroll.add(&load_record_button);
            scroll.add(&delete_record_button);


            let mut frame_clone = frame.clone();
            let mut load_record_button_clone = load_record_button.clone();
            let mut delete_record_button_clone = delete_record_button.clone();
            delete_record_button.set_callback(move |_| {
                frame_clone.hide();
                load_record_button_clone.hide();
                delete_record_button_clone.hide();
                let _ = record::delete_record(record.id.unwrap());
            });
        }

        scroll.end();
        load_record_window.add(&scroll);

        load_record_window.end();
        load_record_window.show();
    });
}
