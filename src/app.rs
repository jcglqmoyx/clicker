use std::error;

use fltk::app;

use crate::config;
use crate::gui::home;
use crate::gui::home::apply_ui;
use crate::listener::exec;
use crate::persistence::dao::init;
use crate::state::{UiMsg, STATE};

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let db_path = config::get_database_path();
    let _ = init::init(&db_path);
    let app = app::App::default();

    let (ui_sender, ui_receiver) = app::channel::<UiMsg>();
    let (_window, mut widgets) = home::window(ui_sender.clone());
    exec::start(STATE.clone(), ui_sender);

    // Drain UI messages sent from worker threads and apply them on the main
    // thread, where FLTK widgets are safe to touch.
    app::add_idle3(move |_| {
        while let Some(msg) = ui_receiver.recv() {
            apply_ui(&msg, &mut widgets);
        }
    });

    app.run().unwrap();
    Ok(())
}
