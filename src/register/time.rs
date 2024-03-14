use fltk::misc::Spinner;

use crate::global::click::EVENTS;
use crate::global::mode::CLICKING_MODE;
use crate::global::mode::Mode;
use crate::global::time::TIME_INTERVAL;

pub(crate) unsafe fn on_time_interval_change(
    hour_spinner: &mut Spinner,
    minute_spinner: &mut Spinner,
    second_spinner: &mut Spinner,
    tenth_spinner: &mut Spinner,
    hundredth_spinner: &mut Spinner,
) {
    let hours = hour_spinner.value() as u64;
    let minutes = minute_spinner.value() as u64;
    let seconds = second_spinner.value() as u64;
    let tenths = tenth_spinner.value() as u64;
    let hundredths = hundredth_spinner.value() as u64;
    TIME_INTERVAL = hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + tenths * 100 + hundredths * 10;
    if CLICKING_MODE == Mode::IntelligentMode {
        if EVENTS.is_empty() {
            return;
        }
        EVENTS[EVENTS.len() - 1].sleep = TIME_INTERVAL;
    }
}
