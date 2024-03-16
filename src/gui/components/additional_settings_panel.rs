use fltk::button::CheckButton;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::prelude::{ButtonExt, GroupExt, WidgetBase, WidgetExt};

use crate::register::coordinate::on_mouse_move;
use crate::register::smart_click::toggle_enable_sound_effect;

pub fn additional_settings_panel() -> Pack {
    let mut panel = Pack::new(50, 240, 200, 120, "Pointer");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let mut pointer_coordinate_pack = Pack::new(50, 240, 180, 30, "");
    pointer_coordinate_pack.set_type(PackType::Horizontal);
    let x_coordinate_input = Frame::new(0, 0, 60, 10, "");
    let y_coordinate_input = Frame::new(0, 0, 60, 10, "");
    pointer_coordinate_pack.add(&x_coordinate_input);
    pointer_coordinate_pack.add(&y_coordinate_input);
    on_mouse_move(x_coordinate_input, y_coordinate_input);
    pointer_coordinate_pack.end();

    let mut freeze_pointer_pack = Pack::new(50, 280, 200, 0, "");
    let mut play_record_sound_button = CheckButton::new(50, 190, 0, 30, "Play record sound");
    play_record_sound_button.set(true);
    toggle_enable_sound_effect(&mut play_record_sound_button);
    freeze_pointer_pack.add(&play_record_sound_button);
    freeze_pointer_pack.end();

    panel.add(&pointer_coordinate_pack);
    panel.add(&freeze_pointer_pack);
    panel.end();
    panel
}
