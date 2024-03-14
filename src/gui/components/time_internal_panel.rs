use fltk::enums::Align;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType};
use fltk::misc::Spinner;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::register::time::on_time_interval_change;

pub fn time_interval_panel() -> Pack {
    let mut panel = Pack::new(50, 130, 460, 0, "");

    let mut label_panel = Pack::new(50, 150, 150, 30, "");
    let labels = vec!["hour", "minute", "second", "0.1s", "0.01s"];
    label_panel.set_spacing(20);
    label_panel.set_type(PackType::Horizontal);

    let _frames: Vec<Frame> = labels
        .iter()
        .enumerate()
        .map(|(i, &label)| {
            let mut f = Frame::new(10 + 90 * i as i32, 10, 70, 25, label);
            f.set_align(Align::Inside | Align::Center);
            f
        })
        .collect();
    label_panel.add(&_frames[0]);
    label_panel.add(&_frames[1]);
    label_panel.add(&_frames[2]);
    label_panel.add(&_frames[3]);
    label_panel.add(&_frames[4]);
    label_panel.end();

    let mut spinner_panel = Pack::new(50, 180, 150, 30, "");
    spinner_panel.set_spacing(20);
    spinner_panel.set_type(PackType::Horizontal);

    let mut hour_spinner = Spinner::new(20, 35, 70, 0, "");
    hour_spinner.set_minimum(0.0);
    hour_spinner.set_maximum(23.0);
    hour_spinner.set_step(1.0);
    hour_spinner.set_value(0.0);
    spinner_panel.add(&hour_spinner);

    let mut minute_spinner = Spinner::new(100, 35, 70, 0, "");
    minute_spinner.set_minimum(0.0);
    minute_spinner.set_maximum(59.0);
    minute_spinner.set_step(1.0);
    minute_spinner.set_value(0.0);
    spinner_panel.add(&minute_spinner);

    let mut second_spinner = Spinner::new(170, 35, 70, 0, "");
    second_spinner.set_minimum(0.0);
    second_spinner.set_maximum(59.0);
    second_spinner.set_step(1.0);
    second_spinner.set_value(0.0);
    second_spinner.set_value(1.0);
    spinner_panel.add(&second_spinner);

    let mut tenth_spinner = Spinner::new(230, 35, 70, 0, "");
    tenth_spinner.set_minimum(0.0);
    tenth_spinner.set_maximum(9.0);
    tenth_spinner.set_step(1.0);
    tenth_spinner.set_value(0.0);
    spinner_panel.add(&tenth_spinner);

    let mut hundredth_spinner = Spinner::new(300, 35, 70, 0, "");
    hundredth_spinner.set_minimum(0.0);
    hundredth_spinner.set_maximum(9.0);
    hundredth_spinner.set_step(1.0);
    hundredth_spinner.set_value(0.0);
    spinner_panel.add(&hundredth_spinner);

    unsafe {
        hour_spinner.set_callback(
            {
                let mut hour_spinner_clone = hour_spinner.clone();
                let mut minute_spinner_clone = minute_spinner.clone();
                let mut second_spinner_clone = second_spinner.clone();
                let mut tenth_spinner_clone = tenth_spinner.clone();
                let mut hundredth_spinner_clone = hundredth_spinner.clone();
                move |_| on_time_interval_change(&mut hour_spinner_clone, &mut minute_spinner_clone, &mut second_spinner_clone, &mut tenth_spinner_clone, &mut hundredth_spinner_clone)
            });
        minute_spinner.set_callback(
            {
                let mut hour_spinner_clone = hour_spinner.clone();
                let mut minute_spinner_clone = minute_spinner.clone();
                let mut second_spinner_clone = second_spinner.clone();
                let mut tenth_spinner_clone = tenth_spinner.clone();
                let mut hundredth_spinner_clone = hundredth_spinner.clone();
                move |_| on_time_interval_change(&mut hour_spinner_clone, &mut minute_spinner_clone, &mut second_spinner_clone, &mut tenth_spinner_clone, &mut hundredth_spinner_clone)
            });
        second_spinner.set_callback(
            {
                let mut hour_spinner_clone = hour_spinner.clone();
                let mut minute_spinner_clone = minute_spinner.clone();
                let mut second_spinner_clone = second_spinner.clone();
                let mut tenth_spinner_clone = tenth_spinner.clone();
                let mut hundredth_spinner_clone = hundredth_spinner.clone();
                move |_| on_time_interval_change(&mut hour_spinner_clone, &mut minute_spinner_clone, &mut second_spinner_clone, &mut tenth_spinner_clone, &mut hundredth_spinner_clone)
            });
        tenth_spinner.set_callback(
            {
                let mut hour_spinner_clone = hour_spinner.clone();
                let mut minute_spinner_clone = minute_spinner.clone();
                let mut second_spinner_clone = second_spinner.clone();
                let mut tenth_spinner_clone = tenth_spinner.clone();
                let mut hundredth_spinner_clone = hundredth_spinner.clone();
                move |_| on_time_interval_change(&mut hour_spinner_clone, &mut minute_spinner_clone, &mut second_spinner_clone, &mut tenth_spinner_clone, &mut hundredth_spinner_clone)
            });
        hundredth_spinner.set_callback(
            {
                let mut hour_spinner_clone = hour_spinner.clone();
                let mut minute_spinner_clone = minute_spinner.clone();
                let mut second_spinner_clone = second_spinner.clone();
                let mut tenth_spinner_clone = tenth_spinner.clone();
                let mut hundredth_spinner_clone = hundredth_spinner.clone();
                move |_| on_time_interval_change(&mut hour_spinner_clone, &mut minute_spinner_clone, &mut second_spinner_clone, &mut tenth_spinner_clone, &mut hundredth_spinner_clone)
            });
    }

    spinner_panel.end();

    panel.add(&label_panel);
    panel.add(&spinner_panel);
    panel.end();
    panel
}