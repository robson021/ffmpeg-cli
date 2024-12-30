use crate::string_utils::read_input;
use log::debug;

mod ffmpeg_command;
mod logger_config;
mod string_utils;
mod video_check;

fn print_menu() {
    println!("\nChose one option:");
    println!("1. Convert format (e.g. avi -> mp4).");
    println!("2. Compress using codec.");
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
        match option.unwrap() {
            1 => convert(),
            2 => compress(),
            3 => multi_task(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn convert() {
    // ffmpeg -i {in-video}.mov -vcodec h264 -acodec aac {out-video}.mp4
    println!("Provide video path (e.g. /aaa/bbb/ccc/video.mp4):");
    let path = read_input();

    println!("Provide output format (e.g. mp4):");
    let format = read_input();

    let valid_extension = video_check::has_valid_extension(&format);
    if !valid_extension {
        println!("Invalid extension.");
        return;
    }
    let format = ".".to_string() + &format;

    let path = string_utils::change_file_extension(&path, format.as_str()).unwrap();
    debug!("Path with changed file extension: {}", path);
}

fn multi_task() {
    todo!()
}

fn compress() {
    // ffmpeg -i input.mp4  -vcodec libx265 -crf 28 output.mp4
    todo!()
}
