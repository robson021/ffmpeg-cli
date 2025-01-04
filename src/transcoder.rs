use crate::command_runner::CodecType;
use crate::ffmpeg_command::{
    AudioCodec, CodecAsString, CommandType, FfmpegCommand, FfmpegCommandBuilderError, VideoCodec,
};
use crate::string_utils::read_input;
use crate::{command_runner, ffmpeg_command, string_utils, video_check};
use log::debug;
use std::path::Path;

pub fn convert() -> Result<FfmpegCommand, &'static str> {
    let input = ask_input_file()?;
    let format = ask_output_format()?;

    let format = ".".to_string() + &format;
    if input.ends_with(&format) {
        return Err("Input and output formats are the same.");
    }

    let output = string_utils::change_file_extension(&input, &format)?;
    debug!("Path with changed file extension: {}", output);

    let cmd = ffmpeg_command::builder()
        .command_type(CommandType::ConvertFormat)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::default())
        .video_codec(VideoCodec::default())
        .build();

    unwrap_ffmpeg_command(cmd)
}

pub fn compress() -> Result<FfmpegCommand, &'static str> {
    let input = ask_input_file()?;
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
            return Err("The file already has recommended codecs and mp4 format.");
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

pub fn multi_task() -> Result<FfmpegCommand, &'static str> {
    let input = ask_input_file()?;
    let output = ask_output_format()?;

    let mut cmd = ffmpeg_command::builder();
    let cmd = cmd
        .command_type(CommandType::MultiTask)
        .input_file(input)
        .output_file(output)
        .video_codec(VideoCodec::Custom)
        .audio_codec(AudioCodec::Custom);

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
fn ask_input_file() -> Result<String, &'static str> {
    println!("Provide video path (e.g. /some/directory/video.mp4):");
    let path = read_input();
    match Path::new(&path).exists() {
        true => Ok(path),
        false => Err("File does not exist."),
    }
}

fn ask_output_format() -> Result<String, &'static str> {
    println!("Provide output format:");
    let format = read_input();

    let valid_extension = video_check::has_valid_extension(&format);
    if !valid_extension {
        return Err("Invalid format.");
    }
    Ok(format)
}

#[inline(always)]
fn unwrap_ffmpeg_command(
    cmd: Result<FfmpegCommand, FfmpegCommandBuilderError>,
) -> Result<FfmpegCommand, &'static str> {
    match cmd {
        Ok(cmd) => Ok(cmd),
        Err(err) => {
            debug!("{}", err);
            Err("Failed to build ffmpeg command.")
        }
    }
}
