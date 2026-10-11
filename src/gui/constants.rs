use iced::widget::Id;
use std::sync::LazyLock;
use std::time::Duration;

pub const CUSTOM_FIELDS_HEIGHT: f32 = 232.0;
pub const MIN_WINDOW_HEIGHT: f32 = 400.0;
pub const BLINK_DURATION: Duration = Duration::from_millis(700);
pub const MAX_LOG_LINES: usize = 500;
pub const LABEL_SIZE: u32 = 20;
pub const LABEL_WIDTH: f32 = 120.0;
pub const INPUT_PADDING: f32 = 10.0;
pub const ROW_SPACING: f32 = 12.0;
pub const BOX_PADDING: f32 = ROW_SPACING;
pub const SECTION_SPACING: f32 = 16.0;
pub const CONTENT_PADDING: f32 = 24.0;
pub const LOG_HEIGHT: f32 = 260.0;
pub const LOG_FONT_SIZE: u32 = 13;
pub const POLL_INTERVAL: Duration = Duration::from_millis(150);
pub const BLINK_TICK_INTERVAL: Duration = Duration::from_millis(50);
pub const COPY_BUTTON_TEXT_SIZE: u32 = 12;
pub const COPY_BUTTON_PADDING: f32 = 4.0;
pub const COPY_BUTTON_MARGIN: f32 = 6.0;
pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub static LOG_ID: LazyLock<Id> = LazyLock::new(|| Id::new("ffmpeg-log"));
