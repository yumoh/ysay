
use super::config::*;
use super::utils::BoxResult;
use ureq;
use std::io::Cursor;
use std::io::Read;
use rodio::{Decoder, OutputStream, Sink};


pub struct BaiduTTS {
    config: Config,
    path: String,
    update:bool,
}

impl BaiduTTS {
    pub fn new(config_path:&String) -> Self {
        let config = load_conf(config_path).expect("load config error");
        debug!("[baidu tts init]load config:{:?}",config);
        BaiduTTS {
            config: config,
            path: config_path.to_owned(),
            update: false,
        }
    }

    fn update_token(&mut self) -> BoxResult<()> {
        debug!("[baidu tts update token]");
        if let Some(expire_time) = self.config.token.expires_in {
            let expire_dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
            if expire_dur.as_secs() < expire_time {
                debug!("[baidu tts update token]token expire time check ok!");
                return Ok(())
            }
        }
        let url = "https://openapi.baidu.com/oauth/2.0/token";
        let resp = ureq::get(url)
        .query("grant_type", "client_credentials")
        .query("client_id", &self.config.auth.key)
        .query("client_secret", &self.config.auth.secret)
        .timeout(std::time::Duration::from_secs(3))
        .call()?;
        let mut data:Token = resp.into_json()?;
        if let Some(expire_time) = data.expires_in {
            // 到达expires_in时间，token过期
            let expire_dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
            data.expires_in = Some(expire_dur.as_secs() + expire_time - 20)
        }
        debug!("[update token] data:{:?}",data);
        self.config.token = data;
        self.update = true;
        self.save_state()?;
        self.update = false;
        Ok(())
    }

    pub fn save_state(&self) -> BoxResult<()> {
        debug!("save state:{:?}",&self.config);
        save_conf(&self.path, &self.config)?;
        Ok(())
    }

    pub fn get_audio_resp(&mut self,text:&String) -> BoxResult<ureq::Response> {
        self.update_token()?;
        let url = "http://tsn.baidu.com/text2audio";
        let token = self.config.token.access_token.to_owned().unwrap();
        let resp = ureq::get(url)
        .query("lan", "zh")
        .query("tok",&token)
        .query("ctp", "1")
        .query("cuid", &self.config.params.cuid)
        .query("per", &self.config.params.per)
        .query("spd", "6")
        .query("aue", &self.config.params.aue)
        .query("tex", text).call()?;
        Ok(resp)
    }

    pub fn speech(&mut self,text:&String) -> BoxResult<()> {
        let resp = self.get_audio_resp(text)?;
        let mut reader = resp.into_reader();
        let mut buf:Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf)?;
        let c = Cursor::new(buf);
        let der_source = rodio::decoder::Decoder::new_wav(c)?;
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
    
        sink.append(der_source);
        sink.sleep_until_end();
        Ok(())
    }
}

impl Default for BaiduTTS {
    fn default() -> Self {
        BaiduTTS::new(&"config.toml".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_token()  {
        env_logger::init();
        let mut tts = BaiduTTS::default();
        tts.update_token().expect("update token error");
        if tts.update {
            tts.save_state().expect("save config error");
        }
    }
    #[test]
    fn test_baidu_speech() {
        env_logger::init();
        let mut tts = BaiduTTS::default();
        tts.update_token().expect("update token error");
        if tts.update {
            tts.save_state().expect("save config error");
        }
        tts.speech(&"雨墨世界红尘，一道线隔绝了阴阳生死.".to_string()).expect("speech error");
    }
}