use fltk::button::CheckButton;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::input::IntInput;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::register::count::{change_click_count, toggle_click_counter};

pub fn count_click_panel() -> Pack {
    let mut panel = Pack::new(300, 30, 200, 0, "");
    panel.set_spacing(10);
    panel.set_type(PackType::Vertical);

    let mut counter_pack = Pack::new(300, 20, 200, 30, "");
    let mut count_button = CheckButton::new(300, 30, 80, 30, "Counter");
    toggle_click_counter(&mut count_button);
    counter_pack.add(&count_button);
    counter_pack.end();

    let mut times_pack = Pack::new(0, 30, 150, 30, "");
    times_pack.set_type(PackType::Horizontal);
    let mut times_input = IntInput::new(300, 40, 100, 30, "");
    change_click_count(&mut times_input);
    let times_label = Frame::new(400, 40, 50, 30, "times");
    times_pack.add(&times_input);
    times_pack.add(&times_label);
    times_pack.end();

    panel.add(&counter_pack);
    panel.add(&times_pack);
    panel.end();

    panel
}
