use crate::media::codecs::{AudioCodec, CodecAsString, VideoCodec};
use crate::string_utils;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CommandType {
    #[default]
    Compress,
    ConvertFormat,
    YoutubeOptimized,
    MultiTask,
}

impl CommandType {
    pub const ALL: [Self; 4] = [
        Self::ConvertFormat,
        Self::Compress,
        Self::MultiTask,
        Self::YoutubeOptimized,
    ];
}

impl CommandType {
    pub fn output_suffix(self) -> &'static str {
        match self {
            Self::ConvertFormat => ".mp4",
            Self::Compress => "_compressed.mp4",
            Self::MultiTask => "_output.mp4",
            Self::YoutubeOptimized => "_yt.mp4",
        }
    }
}

impl std::fmt::Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ConvertFormat => "Convert format",
            Self::Compress => "Compress video",
            Self::MultiTask => "Custom command",
            Self::YoutubeOptimized => "YouTube optimized",
        })
    }
}

#[derive(Default, Debug, derive_builder::Builder)]
#[builder(setter(into))]
pub struct FfmpegCommand {
    command_type: CommandType,
    input_file: String,
    output_file: String,
    audio_codec: AudioCodec,
    video_codec: VideoCodec,
    scale: Option<i16>,
    audio_bitrate: Option<i16>,
    preset: Option<String>,
    crf: Option<i16>,
}

pub fn builder() -> FfmpegCommandBuilder {
    FfmpegCommandBuilder::default()
        .scale(None)
        .audio_bitrate(None)
        .preset(None)
        .crf(None)
        .to_owned()
}

impl FfmpegCommand {
    pub fn as_cmd_string(&self) -> String {
        let (input, output) =
            string_utils::add_quotes_if_whitespace_present(&self.input_file, &self.output_file);

        let mut cmd = String::from("ffmpeg -i ");
        cmd.push_str(&input);

        cmd.push_str(&self.codecs_args());

        match self.command_type {
            CommandType::ConvertFormat => { /* skip */ }
            CommandType::Compress => {
                cmd.push_str(r#" -vf "scale=1280:-2" -preset veryslow -crf 24"#);
            }
            CommandType::YoutubeOptimized => {
                cmd.push_str(" -crf 23 -preset medium -b:a 320k -qscale 0");
            }
            CommandType::MultiTask => {
                if let Some(v) = &self.scale {
                    cmd.push_str(format!(r#" -vf "scale={v}-2""#).as_str());
                }
                if let Some(v) = &self.audio_bitrate {
                    cmd.push_str(format!(" -b:a {v}k").as_str());
                }
                if let Some(v) = &self.preset {
                    cmd.push_str(format!(" -preset {v}").as_str());
                }
                if let Some(v) = &self.crf {
                    cmd.push_str(format!(" -crf {v}").as_str());
                }
            }
        }
        cmd.push(' ');
        cmd.push_str(&output);
        cmd
    }

    fn codecs_args(&self) -> String {
        let mut cmd = String::new();
        match self.command_type {
            CommandType::ConvertFormat => { /* skip additional params */ }
            _ => {
                let video_codec = self.video_codec.as_str().to_lowercase();
                let audio_codec = self.audio_codec.as_str().to_lowercase();
                cmd.push_str(" -c:v ");
                cmd.push_str(&video_codec);
                cmd.push_str(" -c:a ");
                cmd.push_str(&audio_codec);
            }
        }
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_build_convert_format_command() {
        let cmd = builder()
            .command_type(CommandType::ConvertFormat)
            .input_file("/aaa/bbb/input_video.mp4")
            .output_file("/ccc/ddd/output_video.avi")
            .audio_codec(AudioCodec::default())
            .video_codec(VideoCodec::default())
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            "ffmpeg -i /aaa/bbb/input_video.mp4 /ccc/ddd/output_video.avi",
            cmd,
        );
    }

    #[test]
    fn should_build_compress_command() {
        let cmd = builder()
            .command_type(CommandType::Compress)
            .input_file("/aaa/bbb/input_video.avi")
            .output_file("/ccc/ddd/output_video.avi")
            .audio_codec(AudioCodec::Aac)
            .video_codec(VideoCodec::Libx264)
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            r#"ffmpeg -i /aaa/bbb/input_video.avi -c:v libx264 -c:a aac -vf "scale=1280:-2" -preset veryslow -crf 24 /ccc/ddd/output_video.avi"#,
            cmd,
        );
    }

    #[test]
    fn should_build_youtube_command() {
        let cmd = builder()
            .command_type(CommandType::YoutubeOptimized)
            .input_file("/aaa/bbb/input_video.avi")
            .output_file("/ccc/ddd/output_video.mp4")
            .audio_codec(AudioCodec::Aac)
            .video_codec(VideoCodec::Libx264)
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            "ffmpeg -i /aaa/bbb/input_video.avi -c:v libx264 -c:a aac -crf 23 -preset medium -b:a 320k -qscale 0 /ccc/ddd/output_video.mp4",
            cmd,
        );
    }

    #[test]
    fn should_build_multi_task_command() {
        let cmd = builder()
            .command_type(CommandType::MultiTask)
            .input_file("/aaa/bbb/input_video.avi")
            .output_file("/ccc/ddd/output_video.mp4")
            .audio_codec(AudioCodec::Aac)
            .video_codec(VideoCodec::Libx264)
            .scale(1280)
            .preset("medium".to_owned())
            .audio_bitrate(320)
            .crf(24)
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            r#"ffmpeg -i /aaa/bbb/input_video.avi -c:v libx264 -c:a aac -vf "scale=1280-2" -b:a 320k -preset medium -crf 24 /ccc/ddd/output_video.mp4"#,
            cmd,
        );
    }

    #[test]
    fn should_build_multi_task_command_and_skip_optional_arguments() {
        let cmd = builder()
            .command_type(CommandType::MultiTask)
            .input_file("/aaa/input_video.avi")
            .output_file("/bbb/output_video.mp4")
            .audio_codec(AudioCodec::Aac)
            .video_codec(VideoCodec::H264)
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            r"ffmpeg -i /aaa/input_video.avi -c:v h264 -c:a aac /bbb/output_video.mp4",
            cmd,
        );
    }
}
