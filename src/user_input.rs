use crate::error::TranscoderError;
use crate::media::video_check;
use crate::string_utils;
use log::debug;
use std::error::Error;
use std::path::Path;

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input.trim().to_owned()
}

pub fn ask_input_and_output_file() -> Result<(String, String, String), Box<dyn Error>> {
    let input = ask_input_file()?;
    let format = ask_output_format()?;
    let format = ".".to_owned() + &format;
    let output = string_utils::change_file_extension(&input, &format)?;
    debug!("Input: {}, Output: {}, Format: {}", input, output, format);
    Ok((input, output, format))
}

#[inline]
pub fn ask_input_file() -> Result<String, Box<dyn Error>> {
    println!("Provide video path (e.g. /some/directory/video.mp4):");
    let path = read_input();
    let file = match Path::new(&path).try_exists() {
        Ok(exists) => match exists {
            true => {
                debug!("File exists: {}", path);
                Ok(path)
            }
            false => Err(TranscoderError::FileNotFound(path).into()),
        },
        Err(_) => Err(TranscoderError::CouldNotCheckFileExistence(path).into()),
    };
    file
}

#[inline]
fn ask_output_format() -> Result<String, &'static str> {
    println!("Provide output format (e.g. mp4):");
    let format = read_input();
    match video_check::has_valid_extension(&format) {
        true => Ok(format),
        false => Err("Invalid format."),
    }
}
