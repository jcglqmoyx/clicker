use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::Coordinate::Abs;
use enigo::Direction::Click;
use enigo::Mouse;
use fltk::app::Sender;
use fltk::enums::Color;

use crate::persistence::entity::event::Event;
use crate::state::{AppState, Mode, UiMsg};
use crate::utils::audio::play_record_sound;
use crate::utils::enigo::get_enigo_instance;
use crate::utils::key::get_key_value_in_device_query;
use crate::utils::time::timestamp;

pub(crate) fn start(state: Arc<Mutex<AppState>>, ui: Sender<UiMsg>) {
    thread::spawn(move || listener(state, ui));
}

fn listener(state: Arc<Mutex<AppState>>, ui: Sender<UiMsg>) {
    // Create the global keyboard listener once; re-creating it every loop
    // iteration is wasteful and re-registers event taps on every pass.
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.is_empty() {
            thread::sleep(Duration::from_millis(20));
            continue;
        }
        let mut keys: Vec<i32> = keys.iter().map(get_key_value_in_device_query).collect();
        keys.sort();

        let (
            mut click_hotkey,
            mut record_hotkey,
            last_click_hotkey_changed,
            last_record_hotkey_changed,
            last_key_pressed,
            clicking_mode,
            status,
        ) = {
            let s = state.lock().unwrap();
            (
                s.click_hotkey.iter().map(|k| k.int_value).collect::<Vec<i32>>(),
                s.record_hotkey.iter().map(|k| k.int_value).collect::<Vec<i32>>(),
                s.last_click_hotkey_changed,
                s.last_record_hotkey_changed,
                s.last_key_pressed,
                s.clicking_mode,
                s.status,
            )
        };
        click_hotkey.sort();
        record_hotkey.sort();

        let now = timestamp();

        if keys == click_hotkey {
            if status > 1 {
                // Recording in progress; ignore the click hotkey.
                thread::sleep(Duration::from_millis(20));
                continue;
            }
            if now - max(last_click_hotkey_changed, last_key_pressed) < 500 {
                thread::sleep(Duration::from_millis(20));
                continue;
            }
            if status == 0 {
                state.lock().unwrap().status = 1;
                state.lock().unwrap().last_key_pressed = now;
                ui.send(UiMsg::Status {
                    label: "Clicking..".to_string(),
                    color: Color::Blue,
                });
                let st = state.clone();
                let ui2 = ui.clone();
                thread::spawn(move || click_loop(st, ui2));
            } else {
                state.lock().unwrap().status = 0;
                state.lock().unwrap().last_key_pressed = now;
                ui.send(UiMsg::Status {
                    label: "Stopped clicking".to_string(),
                    color: Color::Red,
                });
            }
        } else if keys == record_hotkey {
            if status != 0 {
                thread::sleep(Duration::from_millis(20));
                continue;
            }
            if clicking_mode != Mode::IntelligentMode {
                thread::sleep(Duration::from_millis(20));
                continue;
            }
            if now - max(last_record_hotkey_changed, last_key_pressed) < 500 {
                thread::sleep(Duration::from_millis(20));
                continue;
            }

            // Mark recording so a click hotkey can't fire mid-recording.
            state.lock().unwrap().status = 2;
            state.lock().unwrap().last_key_pressed = now;
            ui.send(UiMsg::Status {
                label: "Recording..".to_string(),
                color: Color::Yellow,
            });

            let enigo = match get_enigo_instance() {
                Some(e) => e,
                None => {
                    // Could not initialize input simulation (e.g. missing
                    // Accessibility permission); abort recording cleanly.
                    state.lock().unwrap().status = 0;
                    state.lock().unwrap().last_key_pressed = now;
                    ui.send(UiMsg::Status {
                        label: "Stopped clicking".to_string(),
                        color: Color::Red,
                    });
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
            };
            let location = match enigo.location() {
                Ok(l) => l,
                Err(_) => {
                    state.lock().unwrap().status = 0;
                    state.lock().unwrap().last_key_pressed = now;
                    ui.send(UiMsg::Status {
                        label: "Stopped clicking".to_string(),
                        color: Color::Red,
                    });
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
            };
            let (button, click_type, interval) = {
                let s = state.lock().unwrap();
                (s.button_to_click, s.mouse_click_type, s.time_interval)
            };
            {
                let mut s = state.lock().unwrap();
                s.events.push(Event::new(
                    location.0,
                    location.1,
                    button,
                    click_type,
                    interval,
                ));
                s.last_record_hotkey_changed = now;
            }
            let count = state.lock().unwrap().events.len();
            ui.send(UiMsg::RecordCount(count));

            if state.lock().unwrap().enable_sound {
                play_record_sound();
            }

            state.lock().unwrap().status = 0;
            state.lock().unwrap().last_key_pressed = now;
            ui.send(UiMsg::Status {
                label: "Stopped clicking".to_string(),
                color: Color::Red,
            });
        }

        thread::sleep(Duration::from_millis(20));
    }
}

fn click_loop(state: Arc<Mutex<AppState>>, ui: Sender<UiMsg>) {
    let (mode, button, count, click_type, interval, events) = {
        let s = state.lock().unwrap();
        (
            s.clicking_mode,
            s.button_to_click,
            s.click_count,
            s.mouse_click_type,
            s.time_interval,
            if s.clicking_mode == Mode::IntelligentMode {
                s.events.clone()
            } else {
                Vec::new()
            },
        )
    };

    let stopped = UiMsg::Status {
        label: "Stopped clicking".to_string(),
        color: Color::Red,
    };

    let mut enigo = match get_enigo_instance() {
        Some(e) => e,
        None => {
            // Could not initialize input simulation (e.g. missing Accessibility
            // permission); report stopped and bail out instead of crashing.
            let mut s = state.lock().unwrap();
            if s.status == 1 {
                s.status = 0;
            }
            drop(s);
            ui.send(stopped);
            return;
        }
    };

    match mode {
        Mode::OrdinaryMode => {
            for _ in 0..count {
                if is_clicking(&state) {
                    let _ = enigo.button(button, Click);
                } else {
                    ui.send(stopped.clone());
                    break;
                }
                if is_clicking(&state) {
                    thread::sleep(Duration::from_millis(100));
                } else {
                    ui.send(stopped.clone());
                    break;
                }
                if click_type == 2 {
                    if is_clicking(&state) {
                        let _ = enigo.button(button, Click);
                    } else {
                        ui.send(stopped.clone());
                        break;
                    }
                }
                if is_clicking(&state) {
                    thread::sleep(Duration::from_millis(interval));
                } else {
                    ui.send(stopped.clone());
                    break;
                }
            }
        }
        Mode::IntelligentMode => {
            for _ in 0..count {
                let mut working = true;
                for event in &events {
                    if is_clicking(&state) {
                        let _ = enigo.move_mouse(event.x, event.y, Abs);
                    } else {
                        working = false;
                        ui.send(stopped.clone());
                        break;
                    }
                    if is_clicking(&state) {
                        thread::sleep(Duration::from_millis(50));
                    } else {
                        working = false;
                        ui.send(stopped.clone());
                        break;
                    }
                    if is_clicking(&state) {
                        let _ = enigo.button(event.button, Click);
                    } else {
                        working = false;
                        ui.send(stopped.clone());
                        break;
                    }
                    if event.click_type == 2 {
                        if is_clicking(&state) {
                            thread::sleep(Duration::from_millis(100));
                        } else {
                            working = false;
                            ui.send(stopped.clone());
                            break;
                        }
                        if is_clicking(&state) {
                            let _ = enigo.button(event.button, Click);
                        } else {
                            working = false;
                            ui.send(stopped.clone());
                            break;
                        }
                    }
                    if is_clicking(&state) {
                        thread::sleep(Duration::from_millis(event.sleep));
                    } else {
                        working = false;
                        ui.send(stopped.clone());
                        break;
                    }
                }
                if !working {
                    break;
                }
            }
        }
    }

    // The click run finished; reset the status so a new run can start.
    let mut s = state.lock().unwrap();
    if s.status == 1 {
        s.status = 0;
    }
    drop(s);
    ui.send(stopped);
}

fn is_clicking(state: &Arc<Mutex<AppState>>) -> bool {
    state.lock().unwrap().status == 1
}
