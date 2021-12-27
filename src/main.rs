use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use ureq;
use std::io::{Cursor, Read};


fn speech(text: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = ureq::get("https://nas.dnnmind.com:8088/y-api/tools/tts")
        .send_json(ureq::json!({"text":text}))?.into_reader();
    let mut content:Vec<u8> = Vec::new();
    resp.read_to_end(&mut content)?;
    let c = Cursor::new(content);
    let der_source = rodio::decoder::Decoder::new_wav(c)?;
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(der_source);
    sink.sleep_until_end();
    Ok(())
}

fn sound(path:&String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("[usage] say something");
        return;
    }
    let s = &args[1];
    if std::path::Path::new(s).exists() {
        sound(s);
    } else {
        speech(s).expect("speech error");
    }
}
