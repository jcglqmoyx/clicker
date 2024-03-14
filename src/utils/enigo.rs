use enigo::{Enigo, Settings};

pub(crate) fn get_enigo_instance() -> Enigo {
    Enigo::new(
        &Settings {
            mac_delay: 0,
            linux_delay: 0,
            x11_display: None,
            wayland_display: None,
            release_keys_when_dropped: true,
        }
    ).unwrap()
}