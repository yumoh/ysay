#[macro_use]
extern crate log;

mod utils;
mod config;
mod baidu_tts;

// use baidu_tts;
// use utils;



fn main() {
    env_logger::init();

    let args:Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("[usage] say \"说一些什么\"");
        return;
    }
    let s = &args[1];
    if std::path::Path::new(s).exists() {
        utils::sound_file(s);
    } else {
        baidu_tts::BaiduTTS::default().speech(s).expect("speech error")
    }
}
