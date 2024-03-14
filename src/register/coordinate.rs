use std::thread;
use std::time::Duration;
use enigo::Mouse;
use fltk::frame::Frame;
use fltk::prelude::WidgetExt;
use crate::utils::enigo::get_enigo_instance;

pub(crate) fn on_mouse_move(mut x_coordinate_input: Frame, mut y_coordinate_input: Frame){
    thread::spawn(move || {
        let enigo = get_enigo_instance();
        loop {
            let location = enigo.location().unwrap();
            x_coordinate_input.set_label(&format!("X: {}", location.0));
            y_coordinate_input.set_label(&format!("Y: {}", location.1));
            thread::sleep(Duration::from_millis(100));
        }
    });
}