use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::menu::Choice;
use fltk::prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt};

use crate::register::mouse::{register_button_to_click_choice, register_mouse_click_type_choice};

pub fn mouse_panel() -> Pack {
    let mut panel = Pack::new(50, 30, 200, 0, "");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let mut mouse_pack = Pack::new(100, 20, 150, 30, "");
    mouse_pack.set_type(PackType::Horizontal);
    let mouse_label = Frame::new(100, 20, 70, 30, "Mouse");
    let mut mouse_choice = Choice::new(100, 20, 100, 30, "");
    mouse_choice.add_choice("left|middle|right");
    mouse_choice.set_value(0);
    register_button_to_click_choice(&mut mouse_choice);
    mouse_pack.add(&mouse_label);
    mouse_pack.add(&mouse_choice);
    mouse_pack.end();

    let mut click_pack = Pack::new(100, 60, 150, 30, "");
    click_pack.set_type(PackType::Horizontal);
    let click_label = Frame::new(100, 50, 70, 30, "Click");
    let mut click_choice = Choice::new(100, 50, 100, 30, "");
    click_choice.add_choice("single|double");
    click_choice.set_value(0);
    register_mouse_click_type_choice(&mut click_choice);
    click_pack.add(&click_label);
    click_pack.add(&click_choice);
    click_pack.end();

    panel.add(&mouse_pack);
    panel.add(&click_pack);
    panel.end();
    return panel;
}