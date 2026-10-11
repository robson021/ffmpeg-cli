use crate::command::command_runner;
use crate::command::ffmpeg_command::{self, CommandType, FfmpegCommand};
use crate::media::codecs::{AudioCodec, VideoCodec};
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Alignment, Element, Font, Length, Subscription, Task, Theme};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::Duration;

const CUSTOM_FIELDS_HEIGHT: f32 = 232.0;
const MIN_WINDOW_HEIGHT: f32 = 400.0;
const MAX_LOG_LINES: usize = 500;

enum RunEvent {
    Output(String),
    Finished(Result<(), String>),
}

fn log_id() -> iced::widget::Id {
    iced::widget::Id::new("ffmpeg-log")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    ConvertFormat,
    Compress,
    MultiTask,
    YoutubeOptimized,
}

impl Operation {
    const ALL: [Self; 4] = [
        Self::ConvertFormat,
        Self::Compress,
        Self::MultiTask,
        Self::YoutubeOptimized,
    ];

    fn command_type(self) -> CommandType {
        match self {
            Self::ConvertFormat => CommandType::ConvertFormat,
            Self::Compress => CommandType::Compress,
            Self::MultiTask => CommandType::MultiTask,
            Self::YoutubeOptimized => CommandType::YoutubeOptimized,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ConvertFormat => "Convert format",
            Self::Compress => "Compress video",
            Self::MultiTask => "Custom command",
            Self::YoutubeOptimized => "YouTube optimized",
        })
    }
}

#[derive(Debug, Clone)]
enum Message {
    OperationChanged(Operation),
    InputChanged(String),
    BrowseInput,
    InputPicked(Option<String>),
    OutputChanged(String),
    VideoCodecChanged(String),
    AudioCodecChanged(String),
    ScaleChanged(String),
    BitrateChanged(String),
    PresetChanged(String),
    CrfChanged(String),
    Run,
    PollCommand,
    OpenOutputDir,
}

struct App {
    operation: Option<Operation>,
    input: String,
    output: String,
    video_codec: String,
    audio_codec: String,
    scale: String,
    bitrate: String,
    preset: String,
    crf: String,
    preview: String,
    status: String,
    log: Vec<String>,
    finished_output_dir: Option<PathBuf>,
    result_receiver: Option<Receiver<RunEvent>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            operation: Some(Operation::YoutubeOptimized),
            input: String::new(),
            output: String::new(),
            video_codec: String::new(),
            audio_codec: String::new(),
            scale: String::new(),
            bitrate: String::new(),
            preset: String::new(),
            crf: String::new(),
            preview: String::new(),
            status: String::new(),
            log: Vec::new(),
            finished_output_dir: None,
            result_receiver: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OperationChanged(operation) => {
                let was_custom = self.operation == Some(Operation::MultiTask);
                self.operation = Some(operation);
                self.output = suggest_output(&self.input, operation);
                self.refresh_preview();
                let is_custom = operation == Operation::MultiTask;
                if was_custom != is_custom {
                    let delta = if is_custom {
                        CUSTOM_FIELDS_HEIGHT
                    } else {
                        -CUSTOM_FIELDS_HEIGHT
                    };
                    return resize_window_height(delta);
                }
            }
            Message::InputChanged(input) => {
                self.input = input;
                if let Some(operation) = self.operation {
                    self.output = suggest_output(&self.input, operation);
                }
                self.refresh_preview();
            }
            Message::BrowseInput => {
                return Task::perform(pick_file(), Message::InputPicked);
            }
            Message::InputPicked(Some(path)) => {
                return self.update(Message::InputChanged(path));
            }
            Message::InputPicked(None) => {}
            Message::OutputChanged(output) => {
                self.output = output;
                self.refresh_preview();
            }
            Message::VideoCodecChanged(value) => {
                self.video_codec = value;
                self.refresh_preview();
            }
            Message::AudioCodecChanged(value) => {
                self.audio_codec = value;
                self.refresh_preview();
            }
            Message::ScaleChanged(value) => {
                self.scale = value;
                self.refresh_preview();
            }
            Message::BitrateChanged(value) => {
                self.bitrate = value;
                self.refresh_preview();
            }
            Message::PresetChanged(value) => {
                self.preset = value;
                self.refresh_preview();
            }
            Message::CrfChanged(value) => {
                self.crf = value;
                self.refresh_preview();
            }
            Message::Run => self.start_command(),
            Message::PollCommand => return self.poll_command(),
            Message::OpenOutputDir => {
                if let Some(dir) = &self.finished_output_dir {
                    if let Err(error) = open::that(dir) {
                        self.status = format!("Could not open directory: {}", error);
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let operation = pick_list(Operation::ALL, self.operation, Message::OperationChanged)
            .placeholder("Choose an operation")
            .width(Length::Fill);

        let mut content = column![
            text("FFmpeg CLI").size(30),
            text("Build and run FFmpeg commands").size(16),
            row![text("Operation").width(120), operation]
                .spacing(12)
                .align_y(Alignment::Center),
            row![
                text("Input file").width(120),
                text_input("Path to a video file", &self.input)
                    .on_input(Message::InputChanged)
                    .padding(10)
                    .width(Length::Fill),
                button("Browse...").on_press(Message::BrowseInput),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            labeled_input(
                "Output file",
                &self.output,
                "Output path",
                Message::OutputChanged
            ),
        ]
        .spacing(16)
        .width(Length::Fill);

        if self.operation == Some(Operation::MultiTask) {
            content = content
                .push(labeled_input(
                    "Video codec",
                    &self.video_codec,
                    "e.g. libx264",
                    Message::VideoCodecChanged,
                ))
                .push(labeled_input(
                    "Audio codec",
                    &self.audio_codec,
                    "e.g. aac",
                    Message::AudioCodecChanged,
                ))
                .push(row![
                    labeled_input("Scale", &self.scale, "e.g. 1280", Message::ScaleChanged),
                    labeled_input(
                        "Audio bitrate",
                        &self.bitrate,
                        "e.g. 320 kbps",
                        Message::BitrateChanged
                    ),
                ])
                .push(row![
                    labeled_input(
                        "Preset",
                        &self.preset,
                        "e.g. medium",
                        Message::PresetChanged
                    ),
                    labeled_input("CRF", &self.crf, "e.g. 24", Message::CrfChanged),
                ]);
        }

        content = content
            .push(text("Command preview").size(20))
            .push(
                container(text(if self.preview.is_empty() {
                    "Choose an operation and provide input and output paths.".to_owned()
                } else {
                    self.preview.clone()
                }))
                .padding(12)
                .width(Length::Fill)
                .style(input_like_box),
            )
            .push(
                button(text(if self.result_receiver.is_some() {
                    "FFmpeg is running..."
                } else {
                    "Run FFmpeg"
                }))
                .on_press_maybe(if self.result_receiver.is_none() {
                    Some(Message::Run)
                } else {
                    None
                }),
            );

        if !self.status.is_empty() {
            content = content.push(text(&self.status));
        }

        if !self.log.is_empty() {
            content = content.push(text("Output").size(20)).push(
                container(
                    scrollable(text(self.log.join("\n")).font(Font::MONOSPACE).size(13))
                        .id(log_id())
                        .width(Length::Fill),
                )
                .padding(12)
                .height(260)
                .width(Length::Fill)
                .style(container::bordered_box),
            );
        }

        if self.finished_output_dir.is_some() {
            content = content.push(
                container(
                    button(text("Open output folder"))
                        .on_press(Message::OpenOutputDir)
                        .style(button::success),
                )
                .center_x(Length::Fill),
            );
        }

        container(scrollable(content.padding(24)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.result_receiver.is_some() {
            iced::time::every(Duration::from_millis(150)).map(|_| Message::PollCommand)
        } else {
            Subscription::none()
        }
    }

    fn poll_command(&mut self) -> Task<Message> {
        let mut finished = None;
        if let Some(receiver) = &self.result_receiver {
            loop {
                match receiver.try_recv() {
                    Ok(RunEvent::Output(line)) => self.log.push(line),
                    Ok(RunEvent::Finished(result)) => {
                        finished = Some(result);
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        finished = Some(Err("execution stopped unexpectedly".to_owned()));
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                }
            }
        }
        if self.log.len() > MAX_LOG_LINES {
            self.log.drain(..self.log.len() - MAX_LOG_LINES);
        }
        if let Some(result) = finished {
            self.result_receiver = None;
            match result {
                Ok(()) => {
                    self.status = "FFmpeg completed successfully.".to_owned();
                    self.finished_output_dir = Some(output_directory(&self.output));
                    notify_done();
                }
                Err(error) => self.status = format!("FFmpeg failed: {}", error),
            }
        }
        iced::widget::operation::snap_to_end(log_id())
    }

    fn refresh_preview(&mut self) {
        self.preview = self
            .build_command()
            .map(|command| command.as_cmd_string())
            .unwrap_or_default();
    }

    fn build_command(&self) -> Result<FfmpegCommand, String> {
        let operation = self
            .operation
            .ok_or_else(|| "Choose an operation.".to_owned())?;
        if self.input.trim().is_empty() || self.output.trim().is_empty() {
            return Err("Provide both input and output paths.".to_owned());
        }

        let mut builder = ffmpeg_command::builder();
        builder
            .command_type(operation.command_type())
            .input_file(self.input.trim())
            .output_file(self.output.trim())
            .video_codec(VideoCodec::Custom(if self.video_codec.is_empty() {
                "libx264".to_owned()
            } else {
                self.video_codec.trim().to_owned()
            }))
            .audio_codec(AudioCodec::Custom(if self.audio_codec.is_empty() {
                "aac".to_owned()
            } else {
                self.audio_codec.trim().to_owned()
            }));

        if operation == Operation::MultiTask {
            builder
                .scale(parse_optional(&self.scale, "scale")?)
                .audio_bitrate(parse_optional(&self.bitrate, "audio bitrate")?)
                .preset(if self.preset.trim().is_empty() {
                    None
                } else {
                    Some(self.preset.trim().to_owned())
                })
                .crf(parse_optional(&self.crf, "CRF")?);
        }

        builder.build().map_err(|error| error.to_string())
    }

    fn start_command(&mut self) {
        if self.result_receiver.is_some() {
            return;
        }
        let command = match self.build_command() {
            Ok(command) => command,
            Err(error) => {
                self.status = error;
                return;
            }
        };
        if !Path::new(&self.input).is_file() {
            self.status = format!("Input file does not exist: {}", self.input);
            return;
        }
        if self.input == self.output {
            self.status = "Input and output files must be different.".to_owned();
            return;
        }

        self.status = "Running FFmpeg...".to_owned();
        self.log.clear();
        self.finished_output_dir = None;
        let (sender, receiver) = mpsc::channel();
        self.result_receiver = Some(receiver);
        std::thread::spawn(move || {
            let result = command_runner::run_command_with_output(&command, &|line| {
                let _ = sender.send(RunEvent::Output(line));
            })
            .map_err(|error| error.to_string());
            let _ = sender.send(RunEvent::Finished(result));
        });
    }
}

fn labeled_input<'a>(
    label: &'static str,
    value: &'a str,
    placeholder: &'static str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    row![
        text(label).width(120),
        text_input(placeholder, value)
            .on_input(on_input)
            .padding(10)
            .width(Length::Fill),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

async fn pick_file() -> Option<String> {
    rfd::AsyncFileDialog::new()
        .set_title("Select a video file")
        .add_filter(
            "Video",
            &["mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v"],
        )
        .add_filter("All files", &["*"])
        .pick_file()
        .await
        .map(|file| file.path().to_string_lossy().into_owned())
}

fn resize_window_height(delta: f32) -> Task<Message> {
    iced::window::latest().and_then(move |id| {
        iced::window::size(id).then(move |size| {
            let height = (size.height + delta).max(MIN_WINDOW_HEIGHT);
            iced::window::resize(id, iced::Size::new(size.width, height))
        })
    })
}

fn input_like_box(theme: &Theme) -> container::Style {
    let weak = theme.extended_palette().background.weak;
    container::Style {
        background: Some(weak.color.into()),
        text_color: Some(weak.text),
        border: iced::border::rounded(2),
        ..container::Style::default()
    }
}

fn notify_done() {
    std::thread::spawn(|| {
        if let Err(error) = notify_rust::Notification::new()
            .summary("Done!")
            .body("FFmpeg finished successfully.")
            .show()
        {
            log::warn!("Could not show system notification: {}", error);
        }
    });
}

fn output_directory(output: &str) -> PathBuf {
    match Path::new(output.trim()).parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir.to_path_buf(),
        _ => PathBuf::from("."),
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
        .map_err(|_| format!("{} must be a number between -32768 and 32767.", name))
}

fn suggest_output(input: &str, operation: Operation) -> String {
    if input.trim().is_empty() {
        return String::new();
    }
    let suffix = match operation {
        Operation::ConvertFormat => ".mp4",
        Operation::Compress => "_compressed.mp4",
        Operation::MultiTask => "_output.mp4",
        Operation::YoutubeOptimized => "_yt.mp4",
    };
    crate::string_utils::change_file_extension(input, suffix).unwrap_or_default()
}

pub fn run() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("FFmpeg CLI")
        .theme(|_: &App| Theme::Dark)
        .subscription(App::subscription)
        .window_size(iced::Size::new(1000.0, 930.0))
        .run()
}
