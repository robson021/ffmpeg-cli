use crate::command::ffmpeg_command::FfmpegCommand;
use anyhow::{ensure, Context, Result};
use log::debug;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::time::Instant;

pub enum RunEvent {
    Output(String),
    Finished(Result<(), String>),
}

pub fn run_command(command: &FfmpegCommand) -> Result<()> {
    let command = command.as_cmd_string();
    println!("Running command: {command:?}");

    let start = Instant::now();
    let (program, arg) = get_system_specific_program_and_arg();
    let status = Command::new(program)
        .arg(arg)
        .arg(&command)
        .status()
        .context("Failed to spawn process.")?;
    ensure!(status.success(), "Failed to execute command. {status}.");
    let time = start.elapsed().as_secs();
    println!("Task executed successfully! Time elapsed: {time}s");
    Ok(())
}

pub fn run_command_with_output(command: &FfmpegCommand, on_line: &dyn Fn(String)) -> Result<()> {
    let command = command.as_cmd_string();
    on_line(format!("$ {command}"));

    let (program, arg) = get_system_specific_program_and_arg();
    let reader = duct::cmd(program, [arg, command.as_str()])
        .stdin_null()
        .stderr_to_stdout()
        .unchecked()
        .reader()
        .context("Failed to spawn process.")?;

    for chunk in BufReader::new(&reader).split(b'\r') {
        let chunk = chunk.context("Failed to read process output.")?;
        String::from_utf8_lossy(&chunk)
            .lines()
            .filter(|line| !line.trim().is_empty())
            .for_each(|line| on_line(line.to_owned()));
    }

    let output = reader
        .try_wait()
        .context("Failed to await process.")?
        .context("The process has not finished.")?;
    ensure!(
        output.status.success(),
        "Failed to execute command. {}.",
        output.status
    );
    Ok(())
}

fn get_system_specific_program_and_arg() -> (&'static str, &'static str) {
    if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    }
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
        formats.push(line.to_owned());
    }
    let set: HashSet<String> = formats.into_iter().collect();
    debug!("Found: {} supported formats", set.len());
    set
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
    lines.into_iter().map(ToOwned::to_owned).collect()
}

pub fn get_ffmpeg_version() -> String {
    let v = execute_cmd_get_lines("ffmpeg -version")[0].to_owned();
    let prefix = "ffmpeg version ";
    if !v.contains(prefix) {
        return "Could not find ffmpeg version. Is ffmpeg installed?".to_owned();
    }
    let idx = prefix.len();
    v.as_str()[idx..idx + 4].trim().to_owned()
}
