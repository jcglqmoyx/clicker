use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

pub(crate) fn play_audio(path: &str, sleep: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    thread::sleep(Duration::from_millis(sleep));
}