#[derive(Debug, Clone, Default)]
pub enum AudioCodec {
    #[default]
    Aac,
    Custom(String),
}
#[derive(Debug, Clone, Default)]
pub enum VideoCodec {
    #[default]
    Libx264,
    H264,
    Custom(String),
}

pub trait CodecAsString {
    fn as_string(&self) -> String;
}

impl CodecAsString for AudioCodec {
    fn as_string(&self) -> String {
        match self {
            AudioCodec::Aac => "aac".to_owned(),
            AudioCodec::Custom(codec) => codec.to_owned(),
        }
    }
}

impl CodecAsString for VideoCodec {
    fn as_string(&self) -> String {
        match self {
            VideoCodec::H264 => "h264".to_owned(),
            VideoCodec::Libx264 => "libx264".to_owned(),
            VideoCodec::Custom(codec) => codec.to_owned(),
        }
    }
}

pub enum CodecType {
    Audio,
    Video,
}

pub fn get_codec(video_path: &str, codec_type: CodecType) -> String {
    let audio_or_video = match codec_type {
        CodecType::Audio => "a:0",
        CodecType::Video => "v:0",
    };
    let cmd = format!(
        "ffprobe -v error -select_streams {} -show_entries stream=codec_name -of default=noprint_wrappers=1:nokey=1 {}",
        audio_or_video,
        video_path
    );
    let lines = crate::command_runner::execute_cmd_get_lines(&cmd);
    let result = lines.last();
    match result {
        Some(r) => r.to_owned(),
        None => "".to_owned(),
    }
}
