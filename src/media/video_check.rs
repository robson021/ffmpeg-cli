use crate::command::command_runner;
use std::collections::HashSet;
use std::sync::LazyLock;

static VALID_VIDEO_FORMATS: LazyLock<HashSet<String>> =
    LazyLock::new(command_runner::get_supported_formats);

pub fn has_valid_extension(extension: &str) -> bool {
    VALID_VIDEO_FORMATS.contains(extension)
}
