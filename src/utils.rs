

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub type BoxResult<T> = Result<T,Box<dyn std::error::Error>>;

pub fn sound_file(path:&String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}