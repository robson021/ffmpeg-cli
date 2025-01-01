use crate::ffmpeg_command::{
    AudioCodec, CommandType, FfmpegCommand, FfmpegCommandBuilder, VideoCodec,
};
use crate::string_utils::read_input;
use crate::{string_utils, video_check};
use log::{debug, error};

pub fn convert() -> Result<FfmpegCommand, &'static str> {
    println!("Provide video path (e.g. /aaa/bbb/ccc/video.mp4):");
    let input = read_input();

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

    if cmd.is_err() {
        error!("{}", cmd.err().unwrap());
        Err("Failed to build convert ffmpeg command.")
    } else {
        Ok(cmd.unwrap())
    }
}

pub fn compress() -> Result<FfmpegCommand, &'static str> {
    // ffmpeg -i input.mp4  -vcodec libx265 -crf 28 output.mp4
    todo!()
}

pub fn multi_task() -> Result<FfmpegCommand, &'static str> {
    todo!()
}
