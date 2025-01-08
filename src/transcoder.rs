use crate::codecs::{AudioCodec, CodecAsString, CodecType, VideoCodec};
use crate::error::TranscoderError;
use crate::error::TranscoderError::InvalidCommand;
use crate::ffmpeg_command::{CommandType, FfmpegCommand, FfmpegCommandBuilderError};
use crate::{codecs, ffmpeg_command, string_utils, user_input};
use log::debug;
use std::error::Error;

pub fn convert() -> Result<FfmpegCommand, Box<dyn Error>> {
    let (input, output, format) = user_input::ask_input_and_output_file()?;

    if input.ends_with(&format) {
        return Err(TranscoderError::SameInputAndOutput.into());
    }

    let cmd = ffmpeg_command::builder()
        .command_type(CommandType::ConvertFormat)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::default())
        .video_codec(VideoCodec::default())
        .build();

    unwrap_ffmpeg_command(cmd)
}

pub fn compress() -> Result<FfmpegCommand, Box<dyn Error>> {
    let input = user_input::ask_input_file()?;
    let output = string_utils::change_file_extension(&input, "_compressed.mp4")?;

    let cmd = ffmpeg_command::builder()
        .command_type(CommandType::Compress)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::Aac)
        .video_codec(VideoCodec::Libx264)
        .build();

    unwrap_ffmpeg_command(cmd)
}

pub fn youtube_optimized() -> Result<FfmpegCommand, Box<dyn Error>> {
    let input = user_input::ask_input_file()?;
    let audio_codec = codecs::get_codec(&input, CodecType::Audio);
    let video_codec = codecs::get_codec(&input, CodecType::Video);

    println!(
        "The video codecs are: {} (audio) and {} (video).",
        audio_codec, video_codec
    );

    {
        let h264 = VideoCodec::H264.as_string() == video_codec;
        let libx264 = VideoCodec::Libx264.as_string() == video_codec;

        let video = h264 || libx264;
        let audio = AudioCodec::Aac.as_string() == audio_codec;
        let ext = string_utils::find_file_extension(&input)?;
        let ext = ext == ".mp4";

        if audio && video && ext {
            let reason = "The file already has recommended codecs and mp4 format.";
            return Err(TranscoderError::AbortTranscoding(reason.to_owned()).into());
        }
    }

    let output = string_utils::change_file_extension(&input, "_yt.mp4")?;

    let cmd = ffmpeg_command::builder()
        .command_type(CommandType::YoutubeOptimized)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::Aac)
        .video_codec(VideoCodec::Libx264)
        .build();

    unwrap_ffmpeg_command(cmd)
}

pub fn multi_task() -> Result<FfmpegCommand, Box<dyn Error>> {
    let (input, output, _) = user_input::ask_input_and_output_file()?;

    let read_input = user_input::read_input;

    println!("Provide video codec (e.g. h264):");
    let video_codec = read_input();

    println!("Provide audio codec (e.g. acc):");
    let audio_codec = read_input();

    let mut cmd = ffmpeg_command::builder();
    let cmd = cmd
        .command_type(CommandType::MultiTask)
        .input_file(input)
        .output_file(output)
        .video_codec(VideoCodec::Custom(video_codec))
        .audio_codec(AudioCodec::Custom(audio_codec));

    println!(
        "You will be asked a few optional parameters. Leave the input blank to skip any of them."
    );

    println!("Scale (e.g. 1280):");
    let scale = read_input();
    if !scale.is_empty() {
        match scale.parse::<i16>() {
            Ok(scale) => {
                cmd.scale(scale);
            }
            Err(_) => eprintln!("Invalid scale."),
        };
    }
    println!("Audio bitrate (e.g. 128)");
    let bitrate = read_input();
    if !bitrate.is_empty() {
        match bitrate.parse::<i16>() {
            Ok(bitrate) => {
                cmd.scale(bitrate);
            }
            Err(_) => eprintln!("Invalid bitrate."),
        };
    }
    println!("Preset (e.g. medium):");
    let preset = read_input();
    if !preset.is_empty() {
        cmd.preset(preset);
    }

    println!("Constant Rate Factor [CRF] (e.g. 24):");
    let crf = read_input();
    if !crf.is_empty() {
        match crf.parse::<i16>() {
            Ok(crf) => {
                cmd.crf(crf);
            }
            Err(_) => eprintln!("Invalid CRF."),
        };
    }
    unwrap_ffmpeg_command(cmd.build())
}

#[inline(always)]
fn unwrap_ffmpeg_command(
    cmd: Result<FfmpegCommand, FfmpegCommandBuilderError>,
) -> Result<FfmpegCommand, Box<dyn Error>> {
    match cmd {
        Ok(cmd) => Ok(cmd),
        Err(err) => {
            debug!("{}", err);
            Err(InvalidCommand.into())
        }
    }
}
