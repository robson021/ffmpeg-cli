use crate::command_runner::CodecType;
use crate::ffmpeg_command::{
    AudioCodec, CodecAsString, CommandType, FfmpegCommand, FfmpegCommandBuilder,
    FfmpegCommandBuilderError, VideoCodec,
};
use crate::string_utils::read_input;
use crate::{command_runner, string_utils, video_check};
use log::debug;
use std::path::Path;

pub fn convert() -> Result<FfmpegCommand, &'static str> {
    let input = ask_input_file()?;

    println!("Provide output format:");
    let format = read_input();

    let valid_extension = video_check::has_valid_extension(&format);
    if !valid_extension {
        return Err("Invalid format.");
    }
    let format = ".".to_string() + &format;
    if input.ends_with(&format) {
        return Err("Input and output formats are the same.");
    }

    let output = string_utils::change_file_extension(&input, &format)?;
    debug!("Path with changed file extension: {}", output);

    let cmd = FfmpegCommandBuilder::default()
        .command_type(CommandType::ConvertFormat)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::default())
        .video_codec(VideoCodec::default())
        .build();

    build_command(cmd)
}

pub fn compress() -> Result<FfmpegCommand, &'static str> {
    let input = ask_input_file()?;
    let output = string_utils::change_file_extension(&input, "_compressed.mp4")?;

    let cmd = FfmpegCommandBuilder::default()
        .command_type(CommandType::Compress)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::Aac)
        .video_codec(VideoCodec::Libx264)
        .build();

    build_command(cmd)
}

pub fn youtube_optimized() -> Result<FfmpegCommand, &'static str> {
    let input = ask_input_file()?;
    let audio_codec = command_runner::get_codec(&input, CodecType::Audio);
    let video_codec = command_runner::get_codec(&input, CodecType::Video);

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
            return Err("File already has recommended codecs and mp4 format.");
        }
    }

    let output = string_utils::change_file_extension(&input, "_yt.mp4")?;

    let cmd = FfmpegCommandBuilder::default()
        .command_type(CommandType::YoutubeOptimized)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::Aac)
        .video_codec(VideoCodec::Libx264)
        .build();

    build_command(cmd)
}

pub fn multi_task() -> Result<FfmpegCommand, &'static str> {
    todo!()
}

#[inline(always)]
fn ask_input_file() -> Result<String, &'static str> {
    println!("Provide video path (e.g. /aaa/bbb/ccc/video.mp4):");
    let path = read_input();
    match Path::new(&path).exists() {
        true => Ok(path),
        false => Err("File does not exist."),
    }
}

#[inline(always)]
fn build_command(
    cmd: Result<FfmpegCommand, FfmpegCommandBuilderError>,
) -> Result<FfmpegCommand, &'static str> {
    match cmd {
        Ok(cmd) => Ok(cmd),
        Err(err) => {
            debug!("{}", err);
            Err("Failed to build convert ffmpeg command.")
        }
    }
}
