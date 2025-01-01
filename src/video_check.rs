use crate::command_runner;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref VALID_VIDEO_FORMATS: HashSet<String> = command_runner::get_supported_formats();
}

pub fn has_valid_extension(extension: &str) -> bool {
    VALID_VIDEO_FORMATS.contains(extension)
}
