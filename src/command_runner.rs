use crate::ffmpeg_command::FfmpegCommand;
use std::collections::HashSet;
use std::process::Command;
use std::time::Instant;

pub fn run_command(command: &FfmpegCommand) -> Result<(), i32> {
    let command = command.as_cmd_string();
    println!("Running command: {:?}", command);
    let start = Instant::now();
    let status_code = execute_and_wait(command);
    if status_code != 0 {
        return Err(status_code);
    };
    let time = start.elapsed().as_secs();
    println!("Task executed successfully! Time elapsed: {}s", time);
    Ok(())
}

fn get_command_builder() -> Command {
    if cfg!(target_os = "windows") {
        todo!()
    }
    Command::new("sh")
}

fn execute_and_wait(command: String) -> i32 {
    get_command_builder()
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on a process")
        .code()
        .unwrap()
}

pub fn get_supported_formats() -> HashSet<String> {
    let lines = execute_cmd_get_lines("ffmpeg -formats")
        .into_iter()
        .filter(|line| {
            line.contains(" D ")
                || line.contains(" E ")
                || line.contains(" DE ")
                || line.contains(" ED ")
        });
    let mut formats: Vec<String> = Vec::new();

    for line in lines {
        let line = line.split_whitespace().collect::<Vec<_>>();
        if line.len() < 2 {
            continue;
        }
        let line = line[1];
        formats.push(line.to_owned())
    }
    formats.into_iter().collect()
}

fn execute_cmd_get_lines(cmd: &str) -> Vec<String> {
    let output = get_command_builder()
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command. Is ffmpeg installed?");

    let std_out = String::from_utf8_lossy(&output.stdout);
    let lines = std_out.lines().collect::<Vec<&str>>();
    lines.into_iter().map(|line| line.to_owned()).collect()
}

pub enum CodecType {
    Audio,
    Video,
}

pub fn get_codec(file_path: &str, codec_type: CodecType) -> String {
    let audio_or_video = match codec_type {
        CodecType::Audio => "a:0",
        CodecType::Video => "v:0",
    };
    let cmd = format!(
        "ffprobe -v error -select_streams {} -show_entries stream=codec_name -of default=noprint_wrappers=1:nokey=1 {}",
        audio_or_video,
        file_path
    );
    let lines = execute_cmd_get_lines(&cmd);
    let result = lines.last();
    match result {
        Some(r) => r.to_owned(),
        None => "".to_owned(),
    }
}
