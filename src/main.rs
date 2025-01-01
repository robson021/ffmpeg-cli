use crate::string_utils::read_input;
use log::error;

mod command_runner;
mod ffmpeg_command;
mod logger_config;
mod string_utils;
mod transcoder;
mod video_check;

fn print_menu() {
    println!("\nChose an option:");
    println!("1. Convert format (e.g. avi -> mp4).");
    println!("2. Compress using specific codec.");
    println!("3. Complex command.");
    println!("0. Exit program.");
}

fn main() {
    logger_config::setup_logger();
    println!("Welcome to ffmpeg-cli!");
    loop {
        print_menu();
        let option = read_input().parse::<i32>();
        match option {
            Ok(option) => {
                if option == 0 {
                    break;
                }
                handle_menu_option(option)
            }
            Err(_) => println!("Invalid option. A number was expected."),
        }
    }
    println!("Shutting down.");
}

fn handle_menu_option(option: i32) {
    let ffmpeg_command = match option {
        1 => transcoder::convert(),
        // todo: add more options
        _ => Err("Invalid choice."),
    };
    if ffmpeg_command.is_err() {
        println!(
            "Could not build ffmpeg command. Reason:\n{}",
            ffmpeg_command.err().unwrap()
        );
        return;
    }
    let ffmpeg_command = ffmpeg_command.unwrap();
    let result = command_runner::run_command(ffmpeg_command);
    if let Err(code) = result {
        error!("Error. Process exit with the status: {}", code);
    };
}
