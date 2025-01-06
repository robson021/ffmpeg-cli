use crate::user_input;

#[derive(Debug, Clone, Default)]
pub enum AudioCodec {
    #[default]
    Aac,
    Custom,
}
#[derive(Debug, Clone, Default)]
pub enum VideoCodec {
    #[default]
    Libx264,
    H264,
    Custom,
}

pub trait CodecAsString {
    fn as_string(&self) -> String;
}

impl CodecAsString for AudioCodec {
    fn as_string(&self) -> String {
        match self {
            AudioCodec::Aac => "aac".to_owned(),
            _ => {
                println!("Provide audio codec (e.g. aac):");
                user_input::read_input()
            }
        }
    }
}

impl CodecAsString for VideoCodec {
    fn as_string(&self) -> String {
        match self {
            VideoCodec::H264 => "h264".to_owned(),
            VideoCodec::Libx264 => "libx264".to_owned(),
            _ => {
                println!("Provide video codec (e.g. h264):");
                user_input::read_input()
            }
        }
    }
}
