use crate::media::codecs::{AudioCodec, VideoCodec};
use crate::string_utils;
use strum::{Display, VariantArray};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Display, VariantArray)]
pub enum CommandType {
    #[strum(to_string = "Convert format")]
    ConvertFormat,
    #[default]
    #[strum(to_string = "Compress video")]
    Compress,
    #[strum(to_string = "Custom command")]
    MultiTask,
    #[strum(to_string = "YouTube optimized")]
    YoutubeOptimized,
}

impl CommandType {
    pub fn suggest_output(self, input: &str) -> anyhow::Result<String> {
        string_utils::change_file_extension(input, self.output_suffix())
    }

    fn output_suffix(self) -> &'static str {
        match self {
            Self::ConvertFormat => ".mp4",
            Self::Compress => "_compressed.mp4",
            Self::MultiTask => "_output.mp4",
            Self::YoutubeOptimized => "_yt.mp4",
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
        .clone()
}

impl FfmpegCommand {
    pub fn as_cmd_string(&self) -> String {
        let (input, output) =
            string_utils::add_quotes_if_whitespace_present(&self.input_file, &self.output_file);

        let mut args = vec!["ffmpeg".to_owned(), "-i".to_owned(), input];
        args.extend(self.codec_args());
        args.extend(self.quality_args());
        args.push(output);
        args.join(" ")
    }

    fn codec_args(&self) -> Vec<String> {
        if self.command_type == CommandType::ConvertFormat {
            return Vec::new();
        }
        let video_codec = self.video_codec.to_string().to_lowercase();
        let audio_codec = self.audio_codec.to_string().to_lowercase();
        vec![
            "-c:v".to_owned(),
            video_codec,
            "-c:a".to_owned(),
            audio_codec,
        ]
    }

    fn quality_args(&self) -> Vec<String> {
        match self.command_type {
            CommandType::ConvertFormat => Vec::new(),
            CommandType::Compress => {
                vec![r#"-vf "scale=1280:-2" -preset veryslow -crf 24"#.to_owned()]
            }
            CommandType::YoutubeOptimized => {
                vec!["-crf 23 -preset medium -b:a 320k -qscale 0".to_owned()]
            }
            CommandType::MultiTask => self.custom_args(),
        }
    }

    fn custom_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(scale) = self.scale {
            args.push(format!(r#"-vf "scale={scale}-2""#));
        }
        if let Some(bitrate) = self.audio_bitrate {
            args.push(format!("-b:a {bitrate}k"));
        }
        if let Some(preset) = &self.preset {
            args.push(format!("-preset {preset}"));
        }
        if let Some(crf) = self.crf {
            args.push(format!("-crf {crf}"));
        }
        args
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
