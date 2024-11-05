use super::error::BoxResult;
use super::play::{play_file, play_stream};
use super::tts::TextToSpeech;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct SayCli {
    // #[structopt(short = "t", long = "text", help = "消息主体文本")]
    #[structopt(help = "消息主体文本")]
    pub text: Option<String>,
    #[structopt(short = "f", long = "file", help = "播放音频文件")]
    pub vedio_file: Option<String>,
    #[structopt(short = "o", long = "output", help = "保存到音频文件")]
    pub output_file: Option<String>,
    #[structopt(short = "s", help = "使用流输入文本内容")]
    pub text_stream: bool,
    #[structopt(long = "sf", help = "输入音频流数据")]
    pub file_stream: bool,
    #[structopt(long = "speaker", help = "alex,benjamin,anna,diana,default: diana")]
    pub tts_speaker: Option<String>,
    #[structopt(short="c",long = "config", help = "config file path")]
    pub config_path: Option<String>,
    #[structopt(long = "generate-config", help = "generate config to file")]
    pub gen_config_path: Option<String>,
}

pub fn handle_say(args: &SayCli) -> BoxResult<()> {
    log::debug!("say: {:?}", args);
    if let Some(path) = &args.gen_config_path {
        let path = if path.starts_with('~') {
            dirs::home_dir().expect("no home dir").join(path.strip_prefix('~').unwrap_or_default())
        } else {
            std::path::PathBuf::from(path)
        };
        log::info!("generate config file: {:?}", path);
        TextToSpeech::default().dump(path)?;
        return Ok(());
    }
    let default_config_path = dirs::home_dir().map(|home| home.join(".yumo/tts.toml"));
    let mut tts = if let Some(path) = &args.config_path {
        let path = if path.starts_with('~') {
            dirs::home_dir().expect("no home dir").join(path.strip_prefix('~').unwrap_or_default())
        } else {
            std::path::PathBuf::from(path)
        };
        TextToSpeech::load(path)?
    } else if let Some(default_config) = default_config_path {
        if default_config.exists() {
            TextToSpeech::load(default_config)?
        } else {
            TextToSpeech::default()
        }
    } else {
        TextToSpeech::default()
    };
    if let Some(speaker) = args.tts_speaker.clone() {
        tts = tts.tts_speaker(speaker);
    }
    if let Some(text) = &args.text {
        if let Some(output) = &args.output_file {
            tts.speech_to_file(text, output)?;
        } else {
            tts.speech(text)?;
        }
    }
    if args.text_stream {
        tts.stream_pipe()?;
    }
    if let Some(path) = &args.vedio_file {
        play_file(path)?;
    }
    if args.file_stream {
        play_stream()?;
    }
    Ok(())
}
