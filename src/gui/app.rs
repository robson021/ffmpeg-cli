use super::constants::*;
use super::system::{copy_to_clipboard, notify_done, pick_file};
use crate::command::command_form::{CommandForm, FormField};
use crate::command::command_runner::RunEvent;
use crate::command::ffmpeg_command::CommandType;
use iced::widget::{
    button, column, container, pick_list, row, scrollable, stack, text, text_input,
};
use iced::{Alignment, Element, Font, Length, Subscription, Task, Theme};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::LazyLock;
use std::time::Instant;
use strum::VariantArray;

#[derive(Debug, Clone)]
enum Message {
    OperationChanged(CommandType),
    InputChanged(String),
    BrowseInput,
    InputPicked(Option<String>),
    OutputChanged(String),
    FieldChanged(FormField, String),
    Run,
    PollCommand,
    BlinkTick,
    OpenOutputDir,
    CopyPreview,
}

#[derive(Default)]
struct App {
    form: CommandForm,
    preview: String,
    status: String,
    blink_until: Option<Instant>,
    log: Vec<String>,
    finished_output_dir: Option<PathBuf>,
    result_receiver: Option<Receiver<RunEvent>>,
}

impl App {
    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OperationChanged(operation) => {
                let was_custom = self.form.is_custom();
                self.form.set_operation(operation);
                self.refresh_preview();
                if was_custom != self.form.is_custom() {
                    let delta = if was_custom {
                        -CUSTOM_FIELDS_HEIGHT
                    } else {
                        CUSTOM_FIELDS_HEIGHT
                    };
                    return resize_window_height(delta);
                }
            }
            Message::InputChanged(input) => {
                self.form.set_input(input);
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
                self.form.output = output;
                self.refresh_preview();
            }
            Message::FieldChanged(field, value) => {
                self.form.set_field(field, value);
                self.refresh_preview();
            }
            Message::Run => self.start_command(),
            Message::PollCommand => return self.poll_command(),
            Message::BlinkTick => {
                if self
                    .blink_until
                    .is_some_and(|until| Instant::now() >= until)
                {
                    self.blink_until = None;
                }
            }
            Message::CopyPreview => {
                if !self.preview.is_empty() {
                    return copy_to_clipboard(self.preview.clone());
                }
            }
            Message::OpenOutputDir => {
                if let Some(dir) = &self.finished_output_dir {
                    if let Err(error) = open::that(dir) {
                        self.status = format!("Could not open directory: {error}");
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let operation = pick_list(
            CommandType::VARIANTS,
            self.form.operation,
            Message::OperationChanged,
        )
        .placeholder("Choose an operation")
        .width(Length::Fill);

        let mut content = column![
            row![text("Operation").width(LABEL_WIDTH), operation]
                .spacing(ROW_SPACING)
                .align_y(Alignment::Center),
            row![
                text("Input file").width(LABEL_WIDTH),
                text_input("Path to a video file", &self.form.input)
                    .on_input(Message::InputChanged)
                    .padding(INPUT_PADDING)
                    .width(Length::Fill),
                button("Browse...").on_press(Message::BrowseInput),
            ]
            .spacing(ROW_SPACING)
            .align_y(Alignment::Center),
            labeled_input(
                "Output file",
                &self.form.output,
                "Output path",
                Message::OutputChanged
            ),
        ]
        .spacing(SECTION_SPACING)
        .width(Length::Fill);

        if self.form.is_custom() {
            content = content
                .push(labeled_input(
                    "Video codec",
                    &self.form.video_codec,
                    "e.g. libx264",
                    |value| Message::FieldChanged(FormField::VideoCodec, value),
                ))
                .push(labeled_input(
                    "Audio codec",
                    &self.form.audio_codec,
                    "e.g. aac",
                    |value| Message::FieldChanged(FormField::AudioCodec, value),
                ))
                .push(labeled_input(
                    "Scale",
                    &self.form.scale,
                    "e.g. 1280",
                    |value| Message::FieldChanged(FormField::Scale, value),
                ))
                .push(labeled_input(
                    "Audio bitrate",
                    &self.form.bitrate,
                    "e.g. 320 kbps",
                    |value| Message::FieldChanged(FormField::Bitrate, value),
                ))
                .push(labeled_input("CRF", &self.form.crf, "e.g. 24", |value| {
                    Message::FieldChanged(FormField::Crf, value)
                }))
                .push(labeled_input(
                    "Preset",
                    &self.form.preset,
                    "e.g. medium",
                    |value| Message::FieldChanged(FormField::Preset, value),
                ));
        }

        content = content
            .push(text("Command preview:").size(LABEL_SIZE))
            .push(stack![
                container(text(if self.preview.is_empty() {
                    "Choose an operation and provide input and output paths.".to_owned()
                } else {
                    self.preview.clone()
                }))
                .padding(BOX_PADDING)
                .width(Length::Fill)
                .style(container::bordered_box),
                container(
                    button(text("Copy").size(COPY_BUTTON_TEXT_SIZE))
                        .padding(COPY_BUTTON_PADDING)
                        .on_press_maybe((!self.preview.is_empty()).then_some(Message::CopyPreview)),
                )
                .padding(COPY_BUTTON_MARGIN)
                .width(Length::Fill)
                .align_x(Alignment::End),
            ])
            .push(
                container(
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
                )
                .center_x(Length::Fill),
            );

        content = content.push(text("Logs output:").size(LABEL_SIZE)).push(
            container(
                scrollable(
                    text(self.log.join("\n"))
                        .font(Font::MONOSPACE)
                        .size(LOG_FONT_SIZE),
                )
                .id(LOG_ID.clone())
                .width(Length::Fill),
            )
            .padding(BOX_PADDING)
            .height(LOG_HEIGHT)
            .width(Length::Fill)
            .style(container::bordered_box),
        );

        if !self.status.is_empty() {
            let succeeded = self.finished_output_dir.is_some();
            let blinking = self.blink_until.is_some();
            content = content.push(
                container(text(&self.status))
                    .padding(BOX_PADDING)
                    .width(Length::Fill)
                    .style(move |theme: &Theme| status_box(theme, succeeded, blinking)),
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

        container(scrollable(content.padding(CONTENT_PADDING)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let poll = if self.result_receiver.is_some() {
            iced::time::every(POLL_INTERVAL).map(|_| Message::PollCommand)
        } else {
            Subscription::none()
        };
        let blink = if self.blink_until.is_some() {
            iced::time::every(BLINK_TICK_INTERVAL).map(|_| Message::BlinkTick)
        } else {
            Subscription::none()
        };
        Subscription::batch([poll, blink])
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
                    self.finished_output_dir = Some(self.form.output_directory());
                    self.blink_until = Some(Instant::now() + BLINK_DURATION);
                    notify_done();
                }
                Err(error) => self.status = format!("FFmpeg failed: {error}"),
            }
        }
        iced::widget::operation::snap_to_end(LOG_ID.clone())
    }

    fn refresh_preview(&mut self) {
        self.preview = self.form.preview();
    }

    fn start_command(&mut self) {
        if self.result_receiver.is_some() {
            return;
        }
        match self.form.start() {
            Ok(receiver) => {
                self.status = "Running FFmpeg...".to_owned();
                self.log.clear();
                self.finished_output_dir = None;
                self.blink_until = None;
                self.result_receiver = Some(receiver);
            }
            Err(error) => self.status = error,
        }
    }
}

fn labeled_input<'a>(
    label: &'static str,
    value: &'a str,
    placeholder: &'static str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    row![
        text(label).width(LABEL_WIDTH),
        text_input(placeholder, value)
            .on_input(on_input)
            .padding(INPUT_PADDING)
            .width(Length::Fill),
    ]
    .spacing(ROW_SPACING)
    .align_y(Alignment::Center)
    .into()
}

fn resize_window_height(delta: f32) -> Task<Message> {
    iced::window::latest().and_then(move |id| {
        iced::window::size(id).then(move |size| {
            let height = (size.height + delta).max(MIN_WINDOW_HEIGHT);
            iced::window::resize(id, iced::Size::new(size.width, height))
        })
    })
}

fn status_box(theme: &Theme, succeeded: bool, blinking: bool) -> container::Style {
    let mut style = container::bordered_box(theme);
    if succeeded {
        let success = theme.extended_palette().success.strong;
        style.border = style.border.color(success.color).width(2);
        if blinking {
            style.background = Some(success.color.into());
            style.text_color = Some(success.text);
        }
    }
    style
}

pub fn run() -> iced::Result {
    LazyLock::force(&LOG_ID);
    iced::application(App::new, App::update, App::view)
        .title("FFmpeg CLI")
        .theme(|_: &App| Theme::Dark)
        .subscription(App::subscription)
        .window_size(iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .run()
}
