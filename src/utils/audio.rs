use std::io::Cursor;
use std::thread;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

const RECORD_SOUND_DATA: &'static [u8] = include_bytes!("../../resources/audio/record_sound.mp3");
const CLEAR_RECORD_SOUND_DATA: &'static [u8] = include_bytes!("../../resources/audio/clear_record_sound.mp3");
const SAVE_RECORD_SUCCESS_DATA: &'static [u8] = include_bytes!("../../resources/audio/save_record_success.mp3");

pub(crate) fn play_record_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(RECORD_SOUND_DATA);
    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    thread::sleep(Duration::from_millis(300));
}

pub(crate) fn play_clear_record_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(CLEAR_RECORD_SOUND_DATA);
    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    thread::sleep(Duration::from_millis(300));
}

pub(crate) fn play_save_record_success_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(SAVE_RECORD_SUCCESS_DATA);
    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    thread::sleep(Duration::from_millis(500));
}

