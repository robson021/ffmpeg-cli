use crate::command::command_runner::{self, RunEvent};
use crate::command::ffmpeg_command::{self, CommandType, FfmpegCommand};
use crate::media::codecs::{AudioCodec, VideoCodec};
use crate::string_utils;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};

#[derive(Debug, Clone, Copy)]
pub enum FormField {
    VideoCodec,
    AudioCodec,
    Scale,
    Bitrate,
    Preset,
    Crf,
}

pub struct CommandForm {
    pub operation: Option<CommandType>,
    pub input: String,
    pub output: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub scale: String,
    pub bitrate: String,
    pub preset: String,
    pub crf: String,
}

impl Default for CommandForm {
    fn default() -> Self {
        Self {
            operation: Some(CommandType::YoutubeOptimized),
            input: String::new(),
            output: String::new(),
            video_codec: String::new(),
            audio_codec: String::new(),
            scale: String::new(),
            bitrate: String::new(),
            preset: String::new(),
            crf: String::new(),
        }
    }
}

impl CommandForm {
    pub fn is_custom(&self) -> bool {
        self.operation == Some(CommandType::MultiTask)
    }

    pub fn set_operation(&mut self, operation: CommandType) {
        self.operation = Some(operation);
        self.suggest_output();
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
        self.suggest_output();
    }

    pub fn set_field(&mut self, field: FormField, value: String) {
        match field {
            FormField::VideoCodec => self.video_codec = value,
            FormField::AudioCodec => self.audio_codec = value,
            FormField::Scale => self.scale = value,
            FormField::Bitrate => self.bitrate = value,
            FormField::Preset => self.preset = value,
            FormField::Crf => self.crf = value,
        }
    }

    fn suggest_output(&mut self) {
        let Some(operation) = self.operation else {
            return;
        };
        self.output = if self.input.trim().is_empty() {
            String::new()
        } else {
            string_utils::change_file_extension(&self.input, operation.output_suffix())
                .unwrap_or_default()
        };
    }

    pub fn build_command(&self) -> Result<FfmpegCommand, String> {
        let operation = self
            .operation
            .ok_or_else(|| "Choose an operation.".to_owned())?;
        if self.input.trim().is_empty() || self.output.trim().is_empty() {
            return Err("Provide both input and output paths.".to_owned());
        }

        let mut builder = ffmpeg_command::builder();
        builder
            .command_type(operation)
            .input_file(self.input.trim())
            .output_file(self.output.trim())
            .video_codec(VideoCodec::Custom(or_default(&self.video_codec, "libx264")))
            .audio_codec(AudioCodec::Custom(or_default(&self.audio_codec, "aac")));

        if operation == CommandType::MultiTask {
            builder
                .scale(parse_optional(&self.scale, "scale")?)
                .audio_bitrate(parse_optional(&self.bitrate, "audio bitrate")?)
                .preset(Some(self.preset.trim().to_owned()).filter(|preset| !preset.is_empty()))
                .crf(parse_optional(&self.crf, "CRF")?);
        }

        builder.build().map_err(|error| error.to_string())
    }

    pub fn preview(&self) -> String {
        self.build_command()
            .map(|command| command.as_cmd_string())
            .unwrap_or_default()
    }

    pub fn start(&self) -> Result<Receiver<RunEvent>, String> {
        let command = self.build_command()?;
        if !Path::new(&self.input).is_file() {
            return Err(format!("Input file does not exist: {}", self.input));
        }
        if self.input == self.output {
            return Err("Input and output files must be different.".to_owned());
        }

        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let result = command_runner::run_command_with_output(&command, &|line| {
                let _ = sender.send(RunEvent::Output(line));
            })
            .map_err(|error| format!("{error:#}"));
            let _ = sender.send(RunEvent::Finished(result));
        });
        Ok(receiver)
    }

    pub fn output_directory(&self) -> PathBuf {
        match Path::new(self.output.trim()).parent() {
            Some(dir) if !dir.as_os_str().is_empty() => dir.to_path_buf(),
            _ => PathBuf::from("."),
        }
    }
}

fn or_default(value: &str, default: &str) -> String {
    if value.is_empty() {
        default.to_owned()
    } else {
        value.trim().to_owned()
    }
}

fn parse_optional(value: &str, name: &str) -> Result<Option<i16>, String> {
    if value.trim().is_empty() {
        return Ok(None);
    }
    value
        .trim()
        .parse::<i16>()
        .map(Some)
        .map_err(|_| format!("{name} must be a number between -32768 and 32767."))
}
