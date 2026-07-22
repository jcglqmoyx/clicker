use fltk::draw;
use fltk::enums::Color;
use fltk::prelude::WidgetExt;

pub fn draw_border<W: WidgetExt>(pack: &W, left: i32, up: i32, right: i32, bottom: i32) {
    draw::set_draw_color(Color::Black);
    draw::draw_rect(pack.x() - left, pack.y() - up, pack.width() + right, pack.height() + bottom);
}