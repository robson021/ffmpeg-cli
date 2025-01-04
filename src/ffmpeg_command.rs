use crate::string_utils;

#[derive(Debug, Clone, Default)]
pub enum CommandType {
    #[default]
    Compress,
    ConvertFormat,
    YoutubeOptimized,
    MultiTask,
}

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
                println!("Provide audio codec:");
                string_utils::read_input()
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
                println!("Provide video codec:");
                string_utils::read_input()
            }
        }
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
        let mut cmd = self.cmd_with_codecs();

        match self.command_type {
            CommandType::ConvertFormat => {
                cmd.push_str(" -c:av copy");
            }
            CommandType::Compress => {
                cmd.push_str(r#" -vf "scale=1280:-2" -preset veryslow -crf 24"#);
            }
            CommandType::YoutubeOptimized => {
                cmd.push_str(" -crf 23 -preset medium -b:a 320k -qscale 0");
            }
            CommandType::MultiTask => {
                if let Some(v) = &self.scale {
                    cmd.push_str(format!(r#" -vf "scale={}-2""#, v).as_str());
                }
                if let Some(v) = &self.audio_bitrate {
                    cmd.push_str(format!(" -b:a {}k", v).as_str());
                }
                if let Some(v) = &self.preset {
                    cmd.push_str(format!(" -preset {}", v).as_str());
                }
                if let Some(v) = &self.crf {
                    cmd.push_str(format!(" -crf {}", v).as_str());
                }
            }
        }
        cmd.push(' ');
        cmd.push_str(self.output_file.as_str());
        cmd
    }

    fn cmd_with_codecs(&self) -> String {
        let mut cmd = String::from("ffmpeg -i ");
        cmd.push_str(&self.input_file);
        match self.command_type {
            CommandType::ConvertFormat => {}
            _ => {
                let video_codec = self.video_codec.as_string().to_lowercase();
                let audio_codec = self.audio_codec.as_string().to_lowercase();
                cmd.push_str(" -c:v ");
                cmd.push_str(&video_codec);
                cmd.push_str(" -c:a ");
                cmd.push_str(&audio_codec);
            }
        };
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
            "ffmpeg -i /aaa/bbb/input_video.mp4 -c:av copy /ccc/ddd/output_video.avi",
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
        )
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
        )
    }
}
