

fn speech(text: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = ureq::get("https://api.dnnmind.com:8088/y-api/tools/tts")
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