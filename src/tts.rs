use super::play::PackSequentceStream;
use crate::error::BoxResult;
use std::io::{Cursor, Read, Write};
use reqwest::blocking::{Client,Response};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TextToSpeech {
    service: String,
    speaker: String,
    model: String,
    token: Option<String>,
}

fn read_env_token(key: &str) -> Option<String>{
    std::env::var(key).ok()
}

impl Default for TextToSpeech {
    fn default() -> Self {
        TextToSpeech {
            service: "https://api.siliconflow.cn/v1/audio/speech".to_string(),
            speaker: "diana".to_string(),
            model: "fishaudio/fish-speech-1.4".to_string(),
            token: read_env_token("TTS_TOKEN"),
        }
    }
}

impl TextToSpeech {
    /// 从toml文件中加载
    pub fn load(path: impl AsRef<std::path::Path>) -> BoxResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let tts: TextToSpeech = toml::from_str(&content)?;
        Ok(tts)
    }
    /// 保存到toml文件
    pub fn dump(&self, path: impl AsRef<std::path::Path>) -> BoxResult<()> {
        let content = toml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    /// 使用自定义其它说话人（1-173）
    pub fn tts_speaker(mut self, speaker: impl ToString) -> Self {
        self.speaker = speaker.to_string();
        self
    }
    fn client(&self) -> BoxResult<Client>
    {
        let mut builder = Client::builder()
        .use_rustls_tls();
        // 使用 Authorization: Bearer {token} 认证
        if let Some(token) = &self.token {
            log::warn!("use token: {}", token);
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse()?);
            headers.insert("Content-Type", "application/json".parse()?);
            builder = builder.default_headers(headers);
        } else {
            log::warn!("no token found");
        }
        let client =builder.build()?;
        Ok(client)
    }
    fn request_params(&self,txt:impl ToString) -> BoxResult<String> {
        let val = serde_json::json!(
            {
                "model": self.model,
                "input": txt.to_string(),
                "voice": format!("{}:{}", self.model, self.speaker),
                "response_format": "mp3"
            }
        );
        log::warn!("request_params: {}", val);
        Ok(val.to_string())
    }
    /// get http[s] stream
    fn get_audio_resp(&self, text: &String) -> BoxResult<Response> {
        let client = self.client()?;
        let resp = client.post(self.service.as_str())
            .body(self.request_params(text)?)
            .send()?;
        Ok(resp)
    }

    /// speech
    /// 获取到初始数据后立即播放直到完成
    #[allow(dead_code)]
    pub fn speech(&self, text: &String) -> BoxResult<()> {
        log::debug!("[yumo tts speech] speech {}", &text);
        let resp = self.get_audio_resp(text)?;
        let bytes = resp.bytes()?;
        log::info!("resp: {:?}", bytes.len());
        let data = Cursor::new(bytes);
        PackSequentceStream::new(data).play()?;
        // std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    }

        /// speech
    /// 获取到初始数据后立即播放直到完成
    #[allow(dead_code)]
    pub fn speech_to_file(&self, text: &String,output:impl AsRef<std::path::Path>) -> BoxResult<()> {
        log::debug!("[yumo tts speech] speech {}", &text);
        let resp = self.get_audio_resp(text)?;
        std::fs::write(output, resp.bytes()?)?;
        Ok(())
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

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Once;
    use env_logger::{Builder,Target};
    static INIT: Once = Once::new();
    fn init_log(level:&str) {
        INIT.call_once(|| {
            let mut builder = Builder::from_default_env();
            let level = match level {
                "trace" => log::LevelFilter::Trace,
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                "error" => log::LevelFilter::Error,
                _ => log::LevelFilter::Error,
            };
            builder.filter_level(level);
            builder.target(Target::Stderr);
            builder.init(); 
        });

    }
    #[test]
    fn test_audio_base() -> BoxResult<()>{
        init_log("info");
        let tts = TextToSpeech::default();
        let text = "你好，世界".to_string();
        let resp = tts.get_audio_resp(&text)?;
        let outstring = String::from_utf8_lossy(resp.bytes()?.as_ref()).to_string();
        log::info!("outstring: {}", outstring);
        Ok(())
    }
}