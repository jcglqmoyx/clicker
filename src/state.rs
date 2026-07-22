use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use enigo::Button;
use fltk::enums::Color;
use once_cell::sync::Lazy;

use crate::persistence::entity::event::Event;

/// Clicking mode shared between the GUI and the worker threads.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Mode {
    OrdinaryMode,
    IntelligentMode,
}

/// A key participating in a hotkey combination.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub(crate) struct Key {
    pub(crate) int_value: i32,
    pub(crate) string_value: String,
}

impl Key {
    pub(crate) fn new(int_value: i32, string_value: String) -> Self {
        Key {
            int_value,
            string_value,
        }
    }
}

/// All mutable application state, shared across the GUI thread and the worker
/// threads behind a single mutex. UI widgets themselves are never stored here;
/// they live on the main thread and are updated through [`UiMsg`].
pub(crate) struct AppState {
    pub(crate) clicking_mode: Mode,
    pub(crate) button_to_click: Button,
    // 1: single click, 2: double click
    pub(crate) mouse_click_type: u8,
    pub(crate) enable_sound: bool,
    pub(crate) click_count: i32,
    pub(crate) counter_enabled: bool,
    pub(crate) time_interval: u64,
    pub(crate) click_hotkey: HashSet<Key>,
    pub(crate) record_hotkey: HashSet<Key>,
    pub(crate) last_click_hotkey_changed: u64,
    pub(crate) last_record_hotkey_changed: u64,
    pub(crate) last_key_pressed: u64,
    pub(crate) events: Vec<Event>,
    // 0: stopped, 1: clicking, 2: recording
    pub(crate) status: i32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            clicking_mode: Mode::OrdinaryMode,
            button_to_click: Button::Left,
            mouse_click_type: 1,
            enable_sound: true,
            click_count: i32::MAX,
            counter_enabled: false,
            time_interval: 1000,
            click_hotkey: HashSet::new(),
            record_hotkey: HashSet::new(),
            last_click_hotkey_changed: 0,
            last_record_hotkey_changed: 0,
            last_key_pressed: 0,
            events: Vec::new(),
            status: 0,
        }
    }
}

/// Process-wide shared state. Safe as a `static` because `Arc<Mutex<T>>` is
/// `Sync` for any `Send` `T` (and `AppState` is `Send`).
pub(crate) static STATE: Lazy<Arc<Mutex<AppState>>> =
    Lazy::new(|| Arc::new(Mutex::new(AppState::default())));

/// Messages sent from worker threads to the main GUI thread. The main thread
/// drains them in an idle callback and mutates the widgets, keeping all FLTK
/// widget access on the thread that owns them.
#[derive(Clone)]
pub(crate) enum UiMsg {
    Status { label: String, color: Color },
    RecordCount(usize),
    Coordinates { x: i32, y: i32 },
}

/// Widget handles owned by the main thread, used by the idle callback to apply
/// [`UiMsg`] updates.
pub(crate) struct WindowWidgets {
    pub(crate) status_button: fltk::button::Button,
    pub(crate) count_record_input: fltk::input::Input,
    pub(crate) x_coord_frame: fltk::frame::Frame,
    pub(crate) y_coord_frame: fltk::frame::Frame,
}
