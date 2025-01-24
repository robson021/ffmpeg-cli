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
    fn as_str(&self) -> &str;
}

impl CodecAsString for AudioCodec {
    fn as_str(&self) -> &str {
        match self {
            AudioCodec::Aac => "aac",
            AudioCodec::Custom(codec) => codec,
        }
    }
}

impl CodecAsString for VideoCodec {
    fn as_str(&self) -> &str {
        match self {
            VideoCodec::H264 => "h264",
            VideoCodec::Libx264 => "libx264",
            VideoCodec::Custom(codec) => codec,
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
