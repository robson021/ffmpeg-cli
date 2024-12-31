use crate::ffmpeg_command::{
    AudioCodec, CommandType, FfmpegCommand, FfmpegCommandBuilder, VideoCodec,
};
use crate::string_utils::read_input;
use crate::{string_utils, video_check};
use log::{debug, error};

pub fn convert() -> Result<FfmpegCommand, &'static str> {
    println!("Provide video path (e.g. /aaa/bbb/ccc/video.mp4):");
    let input = read_input();

    println!("Provide output format. The valid formats are:");
    println!("{:?}", video_check::VALID_VIDEO_FORMATS);
    let format = read_input();

    let valid_extension = video_check::has_valid_extension(&format);
    if !valid_extension {
        return Err("Invalid extension.");
    }
    let format = ".".to_string() + &format;

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

fn compress() {
    // ffmpeg -i input.mp4  -vcodec libx265 -crf 28 output.mp4
    todo!()
}

fn multi_task() {
    todo!()
}