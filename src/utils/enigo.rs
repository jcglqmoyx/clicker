use enigo::{Enigo, Settings};

/// Returns a new input simulator, or `None` if it could not be created (e.g. the
/// macOS Accessibility permission has not been granted).
pub(crate) fn get_enigo_instance() -> Option<Enigo> {
    Enigo::new(&Settings::default()).ok()
}
