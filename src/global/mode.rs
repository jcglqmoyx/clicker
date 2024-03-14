use enigo::Button;

#[derive(PartialEq)]
pub(crate) enum Mode {
    OrdinaryMode,
    IntelligentMode,
}

pub(crate) static mut CLICKING_MODE: Mode = Mode::OrdinaryMode;


pub(crate) static mut BUTTON_TO_CLICK: Button = Button::Left;

// 1: single click, 2: double click
pub(crate) static mut MOUSE_CLICK_TYPE: u8 = 1;