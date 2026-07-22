use fltk::misc::Spinner;

use crate::state::{Mode, STATE};

pub(crate) fn on_time_interval_change(
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
    let interval = hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + tenths * 100 + hundredths * 10;

    let mut state = STATE.lock().unwrap();
    state.time_interval = interval;
    if state.clicking_mode == Mode::IntelligentMode {
        if state.events.is_empty() {
            return;
        }
        let last = state.events.len() - 1;
        state.events[last].sleep = interval;
    }
}
