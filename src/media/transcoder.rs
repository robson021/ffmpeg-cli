use crate::command::ffmpeg_command::{self, CommandType, FfmpegCommand};
use crate::media::codecs;
use crate::media::codecs::{AudioCodec, VideoCodec};
use crate::{string_utils, user_input};
use anyhow::{bail, Context, Result};
use log::warn;

pub fn convert() -> Result<FfmpegCommand> {
    let (input, output, format) = user_input::ask_input_and_output_file()?;

    if input.ends_with(&format) {
        bail!("Input and output formats are the same.");
    }

    build_default_codecs(CommandType::ConvertFormat, input, output)
}

pub fn compress() -> Result<FfmpegCommand> {
    let input = user_input::ask_input_file()?;
    let output = CommandType::Compress.suggest_output(&input)?;
    build_default_codecs(CommandType::Compress, input, output)
}

pub fn youtube_optimized() -> Result<FfmpegCommand> {
    let input = user_input::ask_input_file()?;
    let audio_codec = codecs::probe_audio_codec(&input);
    let video_codec = codecs::probe_video_codec(&input);

    println!("The video codecs are: {audio_codec:?} (audio) and {video_codec:?} (video).");

    let video = matches!(video_codec, Some(VideoCodec::H264 | VideoCodec::Libx264));
    let audio = audio_codec == Some(AudioCodec::Aac);
    let mp4 = string_utils::find_file_extension(&input)? == ".mp4";

    if audio && video && mp4 {
        let reason = "The file already has recommended codecs and mp4 format.";
        warn!("{reason} Do you want to proceed anyway (Y/n)?");
        if user_input::read_input().eq_ignore_ascii_case("n") {
            bail!("Aborted. {reason}");
        }
    }

    let output = CommandType::YoutubeOptimized.suggest_output(&input)?;
    build_default_codecs(CommandType::YoutubeOptimized, input, output)
}

pub fn multi_task() -> Result<FfmpegCommand> {
    let (input, output, _) = user_input::ask_input_and_output_file()?;

    println!("Provide video codec (e.g. h264):");
    let video_codec = user_input::read_input();

    println!("Provide audio codec (e.g. aac):");
    let audio_codec = user_input::read_input();

    let mut cmd = ffmpeg_command::builder();
    cmd.command_type(CommandType::MultiTask)
        .input_file(input)
        .output_file(output)
        .video_codec(VideoCodec::Custom(video_codec))
        .audio_codec(AudioCodec::Custom(audio_codec));

    println!(
        "You will be asked a few optional parameters. Leave the input blank to skip any of them."
    );

    if let Some(scale) = ask_optional_number("Scale (e.g. 1280):", "scale") {
        cmd.scale(scale);
    }
    if let Some(bitrate) = ask_optional_number("Audio bitrate (e.g. 128):", "bitrate") {
        cmd.audio_bitrate(bitrate);
    }
    println!("Preset (e.g. medium):");
    let preset = user_input::read_input();
    if !preset.is_empty() {
        cmd.preset(preset);
    }
    if let Some(crf) = ask_optional_number("Constant Rate Factor [CRF] (e.g. 24):", "CRF") {
        cmd.crf(crf);
    }

    cmd.build().context("Invalid command.")
}

fn build_default_codecs(
    command_type: CommandType,
    input: String,
    output: String,
) -> Result<FfmpegCommand> {
    ffmpeg_command::builder()
        .command_type(command_type)
        .input_file(input)
        .output_file(output)
        .audio_codec(AudioCodec::default())
        .video_codec(VideoCodec::default())
        .build()
        .context("Invalid command.")
}

fn ask_optional_number(prompt: &str, name: &str) -> Option<i16> {
    println!("{prompt}");
    let input = user_input::read_input();
    if input.is_empty() {
        return None;
    }
    match input.parse() {
        Ok(number) => Some(number),
        Err(_) => {
            eprintln!("Invalid {name}.");
            None
        }
    }
}
