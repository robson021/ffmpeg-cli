#[derive(Debug, Clone, Default)]
pub enum CommandType {
    #[default]
    Compress,
    ConvertFormat,
    YoutubeReady,
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
    H264,
    Libx264,
}

trait CodecAsString {
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
        cmd.push_str(self.input_file.as_str());
        match self.command_type {
            CommandType::ConvertFormat => {
                cmd.push_str(" -vcodec ");
                cmd.push_str(self.video_codec.as_string().to_lowercase().as_str());
                cmd.push_str(" -acodec ");
                cmd.push_str(self.audio_codec.as_string().to_lowercase().as_str());
            }
            CommandType::Compress => {
                todo!()
            }
            CommandType::MultiTask => {
                todo!()
            }
            CommandType::YoutubeReady => {
                todo!()
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
    fn should_convert_format_ffmpeg_command() {
        let command = FfmpegCommandBuilder::default()
            .command_type(CommandType::ConvertFormat)
            .input_file("/aaa/bbb/input_video.mp4")
            .output_file("/ccc/ddd/output_video.avi")
            .audio_codec(AudioCodec::default())
            .video_codec(VideoCodec::default())
            .build()
            .unwrap()
            .as_cmd_string();

        assert_eq!(
            "ffmpeg -i /aaa/bbb/input_video.mp4 -vcodec h264 -acodec aac /ccc/ddd/output_video.avi",
            command
        );
    }
}
