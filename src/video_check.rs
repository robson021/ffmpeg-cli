pub const VALID_VIDEO_FORMATS: [&str; 4] = ["mp4", "avi", "mov", "mkv"];

pub fn has_valid_extension(extension: &str) -> bool {
    VALID_VIDEO_FORMATS.contains(&extension)
}
