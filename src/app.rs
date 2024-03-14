use std::{
    error,
    thread,
};

use fltk::app;

use crate::{
    gui::home,
    persistence::dao::init,
};
use crate::listener::exec::EVENT_LISTENER;

pub unsafe fn run() -> Result<(), Box<dyn error::Error>> {
    thread::spawn(EVENT_LISTENER);
    let _ = init::init();
    let app = app::App::default();
    home::window();
    app.run().unwrap();
    Ok(())
}