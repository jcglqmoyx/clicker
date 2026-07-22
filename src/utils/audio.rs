use std::thread;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

const RECORD_SOUND_DATA: &'static [u8] = include_bytes!("../../resources/audio/record_sound.wav");
const CLEAR_RECORD_SOUND_DATA: &'static [u8] = include_bytes!("../../resources/audio/clear_record_sound.mp3");
const SAVE_RECORD_SUCCESS_DATA: &'static [u8] = include_bytes!("../../resources/audio/save_record_success.mp3");

/// Play `data` on a dedicated thread so the caller (e.g. the hotkey listener)
/// is never blocked, and the output stream stays alive until the sound ends.
fn play(data: &'static [u8]) {
    thread::spawn(move || {
        let cursor = std::io::Cursor::new(data);
        let Ok(decoder) = Decoder::new(cursor) else {
            return;
        };
        let duration = decoder.total_duration();
        let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
            return;
        };
        let _ = stream_handle.play_raw(decoder.convert_samples());
        match duration {
            Some(d) => thread::sleep(d + Duration::from_millis(50)),
            None => thread::sleep(Duration::from_millis(500)),
        }
    });
}

pub(crate) fn play_record_sound() {
    play(RECORD_SOUND_DATA);
}

pub(crate) fn play_clear_record_sound() {
    play(CLEAR_RECORD_SOUND_DATA);
}

pub(crate) fn play_save_record_success_sound() {
    play(SAVE_RECORD_SUCCESS_DATA);
}
