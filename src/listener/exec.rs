use std::{cmp::max, thread, time::Duration};
use std::fs::File;
use std::io::BufReader;

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Direction::Click, Mouse};
use enigo::Coordinate::Abs;
use fltk::enums::Color;
use fltk::prelude::{InputExt, WidgetExt};
use rodio::{Decoder, OutputStream, source::Source};

use crate::{
    global::{
        click::EVENTS,
        count::CLICK_COUNT,
        hotkey::{CLICK_HOT_KEY, LAST_TIME_CLICK_HOT_KEY_CHANGED, LAST_TIME_RECORD_HOT_KEY_CHANGED, RECORD_HOT_KEY},
        mode::{BUTTON_TO_CLICK, CLICKING_MODE, Mode, MOUSE_CLICK_TYPE},
        status::{LAST_TIME_KEY_PRESSED, STATUS_OF_CLICKER},
        time::TIME_INTERVAL,
    },
    utils::{
        enigo::get_enigo_instance,
        key::get_key_value_in_device_query,
        time::timestamp,
    },
};
use crate::global::click::COUNT_RECORD_INPUT;
use crate::global::mode::PLAY_RECORD_SOUND;
use crate::global::status::STATUS_BUTTON;
use crate::persistence::entity::event::Event;

pub(crate) static mut EVENT_LISTENER: fn() = || unsafe {
    loop {
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.is_empty() { continue; }
        let mut keys: Vec<i32> = keys.iter().map(|x| get_key_value_in_device_query(x)).collect();
        keys.sort();

        let mut click_hotkey = CLICK_HOT_KEY.clone().iter().map(|x| x.int_value).collect::<Vec<i32>>();
        click_hotkey.sort();
        let mut record_hotkey = RECORD_HOT_KEY.clone().iter().map(|x| x.int_value).collect::<Vec<i32>>();
        record_hotkey.sort();

        let now = timestamp();
        if keys == click_hotkey {
            if STATUS_OF_CLICKER > 1 {
                continue;
            }
            if now - max(LAST_TIME_CLICK_HOT_KEY_CHANGED, LAST_TIME_KEY_PRESSED) < 500 {
                continue;
            }
            STATUS_OF_CLICKER ^= 1; // toggle status between clicking and stopped
            if STATUS_OF_CLICKER == 1 {
                thread::spawn(|| {
                    let mut enigo = get_enigo_instance();
                    match CLICKING_MODE {
                        Mode::OrdinaryMode => {
                            for _ in 0..CLICK_COUNT {
                                if STATUS_OF_CLICKER == 1 {
                                    let _ = enigo.button(BUTTON_TO_CLICK, Click);
                                    STATUS_BUTTON.set_label_color(Color::Blue);
                                    STATUS_BUTTON.set_label("Clicking");
                                } else {
                                    STATUS_BUTTON.set_label_color(Color::Red);
                                    STATUS_BUTTON.set_label("Stopped clicking");
                                    break;
                                }
                                if STATUS_OF_CLICKER == 1 {
                                    STATUS_BUTTON.set_label_color(Color::Blue);
                                    STATUS_BUTTON.set_label("Clicking");
                                    thread::sleep(Duration::from_millis(100));
                                } else {
                                    STATUS_BUTTON.set_label_color(Color::Red);
                                    STATUS_BUTTON.set_label("Stopped clicking");
                                    break;
                                }
                                if MOUSE_CLICK_TYPE == 2 {
                                    if STATUS_OF_CLICKER == 1 {
                                        STATUS_BUTTON.set_label_color(Color::Blue);
                                        STATUS_BUTTON.set_label("Clicking");
                                        let _ = enigo.button(BUTTON_TO_CLICK, Click);
                                    } else {
                                        STATUS_BUTTON.set_label_color(Color::Red);
                                        STATUS_BUTTON.set_label("Stopped clicking");
                                        break;
                                    }
                                }
                                if STATUS_OF_CLICKER == 1 {
                                    STATUS_BUTTON.set_label_color(Color::Blue);
                                    STATUS_BUTTON.set_label("Clicking");
                                    thread::sleep(Duration::from_millis(TIME_INTERVAL));
                                } else {
                                    STATUS_BUTTON.set_label_color(Color::Red);
                                    STATUS_BUTTON.set_label("Stopped clicking");
                                    break;
                                }
                            }
                            STATUS_BUTTON.set_label_color(Color::Red);
                            STATUS_BUTTON.set_label("Stopped clicking");
                        }
                        Mode::IntelligentMode => {
                            for _ in 0..CLICK_COUNT {
                                let mut working = true;
                                for event in &EVENTS {
                                    if STATUS_OF_CLICKER == 1 {
                                        STATUS_BUTTON.set_label_color(Color::Blue);
                                        STATUS_BUTTON.set_label("Clicking");
                                        let _ = enigo.move_mouse(event.x, event.y, Abs);
                                    } else {
                                        working = false;
                                        STATUS_BUTTON.set_label_color(Color::Red);
                                        STATUS_BUTTON.set_label("Stopped clicking");
                                        break;
                                    }
                                    if STATUS_OF_CLICKER == 1 {
                                        STATUS_BUTTON.set_label_color(Color::Blue);
                                        STATUS_BUTTON.set_label("Clicking");
                                        thread::sleep(Duration::from_millis(50));
                                    } else {
                                        working = false;
                                        STATUS_BUTTON.set_label_color(Color::Red);
                                        STATUS_BUTTON.set_label("Stopped clicking");
                                        break;
                                    }
                                    if STATUS_OF_CLICKER == 1 {
                                        STATUS_BUTTON.set_label_color(Color::Blue);
                                        STATUS_BUTTON.set_label("Clicking");
                                        let _ = enigo.button(event.button, Click);
                                    } else {
                                        working = false;
                                        STATUS_BUTTON.set_label_color(Color::Red);
                                        STATUS_BUTTON.set_label("Stopped clicking");
                                        break;
                                    }
                                    if event.click_type == 2 {
                                        if STATUS_OF_CLICKER == 1 {
                                            STATUS_BUTTON.set_label_color(Color::Blue);
                                            STATUS_BUTTON.set_label("Clicking");
                                            thread::sleep(Duration::from_millis(100));
                                        } else {
                                            working = false;
                                            STATUS_BUTTON.set_label_color(Color::Red);
                                            STATUS_BUTTON.set_label("Stopped clicking");
                                            break;
                                        }
                                        if STATUS_OF_CLICKER == 1 {
                                            STATUS_BUTTON.set_label_color(Color::Blue);
                                            STATUS_BUTTON.set_label("Clicking");
                                            let _ = enigo.button(event.button, Click);
                                        } else {
                                            working = false;
                                            STATUS_BUTTON.set_label_color(Color::Red);
                                            STATUS_BUTTON.set_label("Stopped clicking");
                                            break;
                                        }
                                    }
                                    if STATUS_OF_CLICKER == 1 {
                                        STATUS_BUTTON.set_label_color(Color::Blue);
                                        STATUS_BUTTON.set_label("Clicking");
                                        thread::sleep(Duration::from_millis(event.sleep));
                                    } else {
                                        working = false;
                                        STATUS_BUTTON.set_label_color(Color::Red);
                                        STATUS_BUTTON.set_label("Stopped clicking");
                                        break;
                                    }
                                }
                                if !working {
                                    STATUS_BUTTON.set_label_color(Color::Red);
                                    STATUS_BUTTON.set_label("Stopped clicking");
                                    break;
                                }
                            }
                            STATUS_BUTTON.set_label_color(Color::Red);
                            STATUS_BUTTON.set_label("Stopped clicking");
                        }
                    }
                });
            }
            LAST_TIME_KEY_PRESSED = now;
        } else if keys == record_hotkey {
            if STATUS_OF_CLICKER != 0 {
                continue;
            }
            if CLICKING_MODE != Mode::IntelligentMode {
                continue;
            }
            if now - max(LAST_TIME_RECORD_HOT_KEY_CHANGED, LAST_TIME_KEY_PRESSED) < 500 {
                continue;
            }
            let enigo = get_enigo_instance();
            let location = enigo.location().unwrap();

            EVENTS.push(Event {
                x: location.0,
                y: location.1,
                button: BUTTON_TO_CLICK,
                click_type: MOUSE_CLICK_TYPE,
                sleep: TIME_INTERVAL,
            });

            if PLAY_RECORD_SOUND {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let file = BufReader::new(File::open("./resources/audio/record_sound.mp3").unwrap());
                let source = Decoder::new(file).unwrap();
                stream_handle.play_raw(source.convert_samples()).unwrap();
                thread::sleep(Duration::from_millis(300));
            }

            COUNT_RECORD_INPUT.set_value(&EVENTS.len().to_string());

            LAST_TIME_KEY_PRESSED = now;
        }
        thread::sleep(Duration::from_millis(20));
    }
};
