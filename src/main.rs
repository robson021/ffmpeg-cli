use log::debug;

mod codecs;
mod command_runner;
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
        let option = user_input::read_input().parse::<i32>();
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
        2 => transcoder::compress(),
        3 => transcoder::multi_task(),
        4 => transcoder::youtube_optimized(),
        _ => Err("Invalid choice."),
    };

    match ffmpeg_command {
        Ok(cmd) => {
            let result = command_runner::run_command(&cmd);
            match result {
                Ok(_) => {
                    debug!("Successfully executed command: {}.", cmd.as_cmd_string());
                }
                Err(code) => eprintln!("Error. Process exit with the status: {}", code),
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
