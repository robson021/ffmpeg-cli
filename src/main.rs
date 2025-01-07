use crate::error::TranscoderError;
use crate::ffmpeg_command::FfmpegCommand;
use log::debug;
use std::error::Error;

mod codecs;
mod command_runner;
mod error;
mod ffmpeg_command;
mod logger_config;
mod string_utils;
mod transcoder;
mod user_input;
mod video_check;

fn print_menu() {
    println!("\nChose an option:");
    println!("1. Convert format (e.g. avi -> mp4).");
    println!("2. Compress video.");
    println!("3. Complex command.");
    println!("4. Convert into Youtube optimized format.");
    println!("0. Exit program.");
}

fn main() {
    logger_config::setup_logger();
    println!("Welcome to ffmpeg-cli!");
    loop {
        print_menu();
        match user_input::read_input().parse::<i32>() {
            Ok(o) => handle_menu_option(o),
            Err(_) => println!("Invalid option. A number was expected."),
        }
    }
}

fn handle_menu_option(option: i32) {
    let ffmpeg_command: Result<FfmpegCommand, Box<dyn Error>> = match option {
        1 => transcoder::convert(),
        2 => transcoder::compress(),
        3 => transcoder::multi_task(),
        4 => transcoder::youtube_optimized(),
        0 => std::process::exit(0),
        _ => Err(TranscoderError::UnsupportedOperation.into()),
    };

    match ffmpeg_command {
        Ok(cmd) => run_cmd(&cmd),
        Err(error) => eprintln!("{}", error),
    }
}

#[inline(always)]
fn run_cmd(cmd: &FfmpegCommand) {
    match command_runner::run_command(cmd) {
        Ok(_) => debug!("Successfully executed command: {}.", cmd.as_cmd_string()),
        Err(reason) => eprintln!("Failed to run the command. Reason: {}", reason),
    }
}
