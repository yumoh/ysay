use super::play::PackSequentceStream;
use crate::error::BoxResult;
use std::io::{Read, Write};

pub struct TextToSpeech {
    service: String,
    speaker: Option<u64>,
}

impl Default for TextToSpeech {
    fn default() -> Self {
        TextToSpeech {
            service: "https://api.yumolab.cn:8088/tts".to_string(),
            speaker: None,
        }
    }
}

impl TextToSpeech {
    /// 使用新的tts服务
    pub fn tts_server<S: ToString>(mut self, server: S) -> Self {
        self.service = server.to_string();
        self
    }
    /// 使用自定义其它说话人（1-173）
    pub fn tts_speaker(mut self, speaker: u64) -> Self {
        self.speaker = Some(speaker);
        self
    }
    /// get http[s] stream
    fn get_audio_resp(&self, text: &String) -> BoxResult<ureq::Response> {
        let url = &self.service;
        if let Some(speaker) = self.speaker {
            let resp = ureq::get(url)
                .query("text", text.as_ref())
                .query("spk_id", &speaker.to_string())
                .call()?;
            Ok(resp)
        } else {
            let resp = ureq::get(url).query("text", text.as_ref()).call()?;
            Ok(resp)
        }
    }

    /// speech
    /// 获取到初始数据后立即播放直到完成
    #[allow(dead_code)]
    pub fn speech(&self, text: &String) -> BoxResult<()> {
        log::info!("[yumo tts speech] speech {}", &text);
        let resp = self.get_audio_resp(text)?;
        PackSequentceStream::new(resp.into_reader()).play()
    }

    /// 从输入流中读文本并speech
    pub fn stream_pipe(&self) -> BoxResult<()> {
        std::io::stdout().flush()?;
        let mut msg = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut msg).expect("==== null ====");
        if msg.trim().is_empty() {
            return Ok(());
        }
        self.speech(&msg)?;
        Ok(())
    }
}
