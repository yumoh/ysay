use super::error::BoxResult;
use rodio::{Decoder, OutputStream, Sink,Source};
use std::fs::File;
use std::io::BufReader;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// 顺序数据转换支持随机读写
pub struct PackSequentceStream {
    reader: Box<dyn Send + Read + Sync + 'static>,
    buf: Cursor<Vec<u8>>,
}

impl Read for PackSequentceStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let s1 = self.buf.position();
        self.buf.seek(SeekFrom::End(0))?;
        let s2 = self.buf.position();
        let s0 = (s2 - s1) as usize;
        if s0 < buf.len() {
            let min_len = if buf.len() > 1024 { buf.len() } else { 1024 };
            let mut tbuf = vec![0u8; min_len];
            let sr = self.reader.read(&mut tbuf)?;
            self.buf.write_all(&tbuf[0..sr])?;
            // log::info!("read from stream {} < {} data: {}", s0, buf.len(),String::from_utf8_lossy(&tbuf[0..sr]));
        }
        self.buf.seek(SeekFrom::Start(s1))?;
        // log::info!("read: {}", buf.len());
        self.buf.read(buf)
    }
}

impl Seek for PackSequentceStream {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        // log::info!("seek: {:?}", pos);
        self.buf.seek(pos)
    }
}

/// 播放音频文件
pub fn play_file<P: AsRef<Path>>(path: P) -> BoxResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    log::info!("sample rate: {} duration: {:?} frames: {:?}", source.sample_rate(), source.total_duration(),source.current_frame_len());
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

/// 播放输入流数据
pub fn play_stream() -> BoxResult<()> {
    let stdin = std::io::stdin();
    let p = PackSequentceStream::new(stdin);
    p.play()
}


impl PackSequentceStream {
    pub fn new<T: Send + Read + Sync + 'static>(reader: T) -> Self {
        Self {
            reader: Box::new(reader),
            buf: Cursor::new(vec![]),
        }
    }

    pub fn play(self) -> BoxResult<()> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let random_stream = BufReader::new(self);
        let source = Decoder::new(random_stream)?;
        log::info!("sample rate: {} duration: {:?} frames: {:?}", source.sample_rate(), source.total_duration(),source.current_frame_len());
        sink.append(source);
        sink.sleep_until_end();
        Ok(())
    }
}
