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
}
#[derive(Debug, Clone, Default)]
pub enum VideoCodec {
    #[default]
    Libx264,
    H264,
}

pub trait CodecAsString {
    fn as_string(&self) -> String;
}

impl CodecAsString for AudioCodec {
    fn as_string(&self) -> String {
        match self {
            AudioCodec::Aac => "aac".to_owned(),
        }
    }
}

impl CodecAsString for VideoCodec {
    fn as_string(&self) -> String {
        match self {
            VideoCodec::H264 => "h264".to_owned(),
            VideoCodec::Libx264 => "libx264".to_owned(),
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
}
impl FfmpegCommand {
    pub fn as_cmd_string(&self) -> String {
        let mut cmd = String::from("ffmpeg");
        cmd.push_str(" -i ");
        cmd.push_str(&self.input_file);

        let video_codec = self.video_codec.as_string().to_lowercase();
        let audio_codec = self.audio_codec.as_string().to_lowercase();
        cmd.push_str(" -c:v ");
        cmd.push_str(&video_codec);
        cmd.push_str(" -c:a ");
        cmd.push_str(&audio_codec);

        match self.command_type {
            CommandType::ConvertFormat => {}
            CommandType::Compress => {
                cmd.push_str(" -preset veryslow -crf 24");
            }
            CommandType::MultiTask => {
                todo!()
            }
            CommandType::YoutubeOptimized => {
                // ffmpeg -i input_video.mp4 -vf "scale=1280:-2" -c:v libx264 -crf 23 -preset medium -c:a aac -b:a 320k output.mp4
                cmd.push_str(" -crf 23 -preset medium -b:a 320k -qscale 0");
            }
        }
        cmd.push(' ');
        cmd.push_str(self.output_file.as_str());
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_build_convert_format_command() {
        let cmd = FfmpegCommandBuilder::default()
            .command_type(CommandType::ConvertFormat)
            .input_file("/aaa/bbb/input_video.mp4")
            .output_file("/ccc/ddd/output_video.avi")
            .audio_codec(AudioCodec::default())
            .video_codec(VideoCodec::default())
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            "ffmpeg -i /aaa/bbb/input_video.mp4 -c:v h264 -c:a aac /ccc/ddd/output_video.avi",
            cmd,
        );
    }

    #[test]
    fn should_build_compress_command() {
        let cmd = FfmpegCommandBuilder::default()
            .command_type(CommandType::Compress)
            .input_file("/aaa/bbb/input_video.avi")
            .output_file("/ccc/ddd/output_video.avi")
            .audio_codec(AudioCodec::Aac)
            .video_codec(VideoCodec::Libx264)
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            "ffmpeg -i /aaa/bbb/input_video.avi -c:v libx264 -c:a aac -preset veryslow -crf 24 /ccc/ddd/output_video.avi",
            cmd,
        );
    }
}
