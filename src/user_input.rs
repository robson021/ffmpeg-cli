use crate::media::video_check;
use crate::string_utils;
use anyhow::{ensure, Context, Result};
use log::debug;
use std::path::Path;

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input.trim().to_owned()
}

pub fn ask_input_and_output_file() -> Result<(String, String, String)> {
    let input = ask_input_file()?;
    let format = ask_output_format()?;
    let format = ".".to_owned() + &format;
    let output = string_utils::change_file_extension(&input, &format)?;
    debug!("Input: {input}, Output: {output}, Format: {format}");
    Ok((input, output, format))
}

pub fn ask_input_file() -> Result<String> {
    println!("Provide video path (e.g. /some/directory/video.mp4):");
    let path = read_input();
    let exists = Path::new(&path)
        .try_exists()
        .with_context(|| format!("Failed to check file existence: {path}."))?;
    ensure!(exists, "File not found: {path}");
    debug!("File exists: {path}");
    Ok(path)
}

fn ask_output_format() -> Result<String> {
    println!("Provide output format (e.g. mp4):");
    let format = read_input();
    ensure!(video_check::has_valid_extension(&format), "Invalid format.");
    Ok(format)
}
