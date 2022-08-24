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
    #[structopt(short = "s", help = "使用流输入文本内容")]
    pub text_stream: bool,
    #[structopt(long = "sf", help = "输入音频流数据")]
    pub file_stream: bool,
    #[structopt(long = "speaker", help = "说话人1-173 默认: 5")]
    pub tts_speaker: Option<u64>,
    #[structopt(long = "server", help = "使用特定tts服务器,默认自建服务器")]
    pub tts_server: Option<String>,
}

pub fn handle_say(args: &SayCli) -> BoxResult<()> {
    log::info!("say: {:?}", args);
    let mut tts = TextToSpeech::default();
    if let Some(server) = &args.tts_server {
        tts = tts.tts_server(server);
    }
    if let Some(speaker) = args.tts_speaker {
        tts = tts.tts_speaker(speaker);
    }
    if let Some(text) = &args.text {
        tts.speech(text)?;
    }
    if args.text_stream {
        tts.stream_pipe()?;
    }
    if let Some(path) = &args.vedio_file {
        play_file(&path)?;
    }
    if args.file_stream {
        play_stream()?;
    }
    Ok(())
}
