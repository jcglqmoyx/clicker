use device_query::Keycode::D;
use fltk::{app, button};
use fltk::button::Button;
use fltk::enums::Event;
use fltk::frame::Frame;
use fltk::group::{Pack, Scroll};
use fltk::input::{Input, IntInput};
use fltk::output::Output;
use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
use fltk::window::Window;
use rusqlite::Connection;

use crate::global::click::{COUNT_RECORD_INPUT, EVENTS};
use crate::persistence::dao::record::add_record;
use crate::persistence::entity::record::Record;
use crate::utils::audio::play_audio;

static mut SAVE_RECORD_WINDOW_OPENED: bool = false;

pub(crate) fn on_clear_record_button_clicked(button: &mut button::Button) {
    button.set_callback(|_| unsafe {
        EVENTS.clear();
        COUNT_RECORD_INPUT.set_value("0");
        play_audio("./resources/audio/clear_record_sound.mp3", 300);
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

                let record = Record::new(0, title, description, EVENTS.clone());
                let _ = add_record(record);

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
        let mut load_window = Window::new(200, 200, 400, 300, "Load");
        let mut scroll = Scroll::new(20, 20, 360, 240, "");
        let mut pack = Pack::default().with_size(360, 0).center_of(&scroll);

        // Retrieve all records from the database
        // let records = load_from_database();

        // Display the records in the pack
        // for record in records {
        //     let mut frame = Frame::default().with_size(340, 100).center_of(&pack);
        //     let mut title_output = Output::new(10, 10, 320, 20, &record.title);
        //     let mut description_output = Output::new(10, 40, 320, 20, &record.description);
        //     let mut content_output = Output::new(10, 70, 320, 20, &record.content);
        //     frame.end();
        //     pack.add(&frame);
        // }

        scroll.end();
        load_window.end();
        load_window.show();
    });
}