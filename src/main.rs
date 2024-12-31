use crate::ffmpeg_command::{
    AudioCodec, CommandType, FfmpegCommand, FfmpegCommandBuilder, VideoCodec,
};
use crate::string_utils::read_input;
use crate::video_check::VALID_VIDEO_FORMATS;
use log::{debug, error};

mod ffmpeg_command;
mod logger_config;
mod string_utils;
mod video_check;

fn print_menu() {
    println!("\nChose an option:");
    println!("1. Convert format (e.g. avi -> mp4).");
    println!("2. Compress using specific codec.");
    println!("3. Complex command.");
    println!("0. Exit program.");
}

// aaa/bbb/ccc/video.mp4

fn main() {
    logger_config::setup_logger();
    println!("Welcome to ffmpeg-cli!");
    loop {
        print_menu();
        let option = read_input().parse::<i32>();
        if option.is_err() {
            println!("Invalid option. A number is required.");
            continue;
        }
        let result = match option.unwrap() {
            1 => convert(),
            // todo: add more options
            0 => break,
            _ => Err("Invalid choice."),
        };
        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
        } else {
            let command = result.unwrap().as_string();
            println!("ffmpeg command to run: {:?}", command);
            // todo: run in system terminal
        }
    }
}

fn convert() -> Result<FfmpegCommand, &'static str> {
    println!("Provide video path (e.g. /aaa/bbb/ccc/video.mp4):");
    let input = read_input();

    println!("Provide output format. The valid formats are:");
    println!("{:?}", VALID_VIDEO_FORMATS);
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

fn multi_task() {
    todo!()
}

fn compress() {
    // ffmpeg -i input.mp4  -vcodec libx265 -crf 28 output.mp4
    todo!()
}
