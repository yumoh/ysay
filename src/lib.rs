mod error;
mod play;
mod tts;

pub use play::{play_file, play_stream, PackSequentceStream};
pub use tts::TextToSpeech;
