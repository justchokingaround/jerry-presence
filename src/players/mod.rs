pub mod mpv;

use crate::stream;

pub trait Player {
    fn play(&self, media: stream::Stream) -> Result<std::process::Child, Box<dyn std::error::Error>>;
}