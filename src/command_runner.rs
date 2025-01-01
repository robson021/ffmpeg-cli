use crate::ffmpeg_command::FfmpegCommand;
use std::collections::HashSet;
use std::process::Command;
use std::time::Instant;

pub fn run_command(command: FfmpegCommand) -> Result<(), i32> {
    let command = command.as_string();
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
    let output = get_command_builder()
        .arg("-c")
        .arg("ffmpeg -formats")
        .output()
        .expect("Failed to list supported ffmpeg formats. Is ffmpeg installed?");

    let std_out = String::from_utf8_lossy(&output.stdout);
    let lines = std_out.lines().filter(|line| {
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
