use crate::string_utils::{read_input, rfind_utf8};

mod ffmpeg_command;
mod string_utils;

fn main() {
    println!("1. Convert format (e.g. avi -> mp4)");
    println!("2. Compress using codec");
    let option = read_input().parse::<i32>().expect("A number was expected.");
    match option {
        1 => convert(),
        2 => compress(),
        3 => multi_task(),
        _ => println!("Invalid choice."),
    }
}

fn convert() {
    // ffmpeg -i {in-video}.mov -vcodec h264 -acodec aac {out-video}.mp4
    println!("Output format (e.g. mp4):");
    let format = read_input();

    let valid_extension = ["mp4", "avi", "mov"].contains(&&*format);
    if !valid_extension {
        println!("Invalid extension.");
        return;
    }

    println!("Provide video path:");
    let path = read_input();

    match rfind_utf8(path.as_str(), '.') {
        None => {
            println!("No file with extension has been found.");
            return;
        }
        Some(position) => {
            let video_name = &path[position..];
            println!("Found video with an extension: {}", video_name);
            let video_name = (&path[..position]); //.to_owned().push_str(&format);
            let xxx = video_name.to_owned().push_str(&format);
            println!("New video: {:?}", video_name);
        }
    };

    // /Users/robertnowak/Movies/out-video.mp4
    // println!("Path: {}", path);
    // todo!()
}

fn multi_task() {
    todo!()
}

fn compress() {
    // ffmpeg -i input.mp4  -vcodec libx265 -crf 28 output.mp4
    todo!()
}
