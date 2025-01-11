use crate::command::ffmpeg_command::FfmpegCommand;
use crate::error::ProcessFailure;
use std::collections::HashSet;
use std::process::Command;
use std::time::Instant;

pub fn run_command(command: &FfmpegCommand) -> Result<(), ProcessFailure> {
    let command = command.as_cmd_string();
    println!("Running command: {:?}", command);

    let start = Instant::now();
    let status_code = execute_and_wait(command)?;
    if status_code != 0 {
        return Err(ProcessFailure::CommandExecution(status_code));
    };
    let time = start.elapsed().as_secs();
    println!("Task executed successfully! Time elapsed: {}s", time);
    Ok(())
}

#[inline]
fn get_system_specific_program_and_arg() -> (&'static str, &'static str) {
    if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    }
}

fn execute_and_wait(command: String) -> Result<i32, ProcessFailure> {
    let program_arg = get_system_specific_program_and_arg();
    let cmd = Command::new(program_arg.0)
        .arg(program_arg.1)
        .arg(command)
        .spawn();
    if cmd.is_err() {
        return Err(ProcessFailure::Spawn);
    }
    let cmd = cmd.unwrap().wait();
    if cmd.is_err() {
        return Err(ProcessFailure::Await);
    }
    Ok(cmd.unwrap().code().unwrap())
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

pub fn execute_cmd_get_lines(cmd: &str) -> Vec<String> {
    let program_arg = get_system_specific_program_and_arg();
    let output = Command::new(program_arg.0)
        .arg(program_arg.1)
        .arg(cmd)
        .output()
        .expect("Failed to execute command. Is ffmpeg installed?");

    let std_out = String::from_utf8_lossy(&output.stdout);
    let lines = std_out.lines().collect::<Vec<&str>>();
    lines.into_iter().map(|line| line.to_owned()).collect()
}
