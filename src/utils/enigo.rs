use enigo::{Enigo, Settings};

pub(crate) fn get_enigo_instance() -> Enigo {
    Enigo::new(&Settings::default()).unwrap()
}
