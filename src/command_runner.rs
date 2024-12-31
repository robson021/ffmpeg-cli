use crate::ffmpeg_command::FfmpegCommand;
use std::process::Command;
use std::time::Instant;

pub fn run_command(command: FfmpegCommand) -> Result<(), i32> {
    let command = command.as_string();
    println!("Running command: {:?}", command);
    let start = Instant::now();
    if cfg!(target_os = "windows") {
        todo!()
    } else {
        let status_code = build_and_run_unix_command(command);
        if status_code != 0 {
            return Err(status_code);
        }
    };
    let time = start.elapsed().as_secs();
    println!("Task executed successfully! Time elapsed: {}s", time);
    Ok(())
}

fn build_and_run_unix_command(command: String) -> i32 {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on a process")
        .code()
        .unwrap()
}
