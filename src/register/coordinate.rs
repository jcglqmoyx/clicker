use std::thread;
use std::time::Duration;

use enigo::Mouse;
use fltk::app::Sender;

use crate::state::UiMsg;
use crate::utils::enigo::get_enigo_instance;

pub(crate) fn on_mouse_move(ui: Sender<UiMsg>) {
    thread::spawn(move || {
        let Some(enigo) = get_enigo_instance() else {
            // Without input access we can't read the pointer; stop the thread.
            return;
        };
        loop {
            match enigo.location() {
                Ok(location) => ui.send(UiMsg::Coordinates {
                    x: location.0,
                    y: location.1,
                }),
                Err(_) => {}
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}
