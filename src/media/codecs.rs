use crate::command::command_runner;
use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Default, PartialEq, Eq, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum AudioCodec {
    #[default]
    Aac,
    #[strum(default, to_string = "{0}")]
    Custom(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum VideoCodec {
    #[default]
    Libx264,
    H264,
    #[strum(default, to_string = "{0}")]
    Custom(String),
}

pub fn probe_audio_codec(video_path: &str) -> Option<AudioCodec> {
    probe_codec(video_path, "a:0")
}

pub fn probe_video_codec(video_path: &str) -> Option<VideoCodec> {
    probe_codec(video_path, "v:0")
}

fn probe_codec<T: FromStr>(video_path: &str, stream: &str) -> Option<T> {
    let cmd = format!(
        "ffprobe -v error -select_streams {stream} -show_entries stream=codec_name -of default=noprint_wrappers=1:nokey=1 {video_path}"
    );
    command_runner::execute_cmd_get_lines(&cmd)
        .pop()
        .and_then(|codec| codec.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_known_and_custom_codecs() {
        assert_eq!("AAC".parse::<AudioCodec>().unwrap(), AudioCodec::Aac);
        assert_eq!("h264".parse::<VideoCodec>().unwrap(), VideoCodec::H264);
        let custom = "vp9".parse::<VideoCodec>().unwrap();
        assert_eq!(custom, VideoCodec::Custom("vp9".to_owned()));
        assert_eq!(custom.to_string(), "vp9");
        assert_eq!(VideoCodec::Libx264.to_string(), "libx264");
    }
}
