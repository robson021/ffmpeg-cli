pub fn has_valid_extension(extension: &str) -> bool {
    ["mp4", "avi", "mov", "mkv"].contains(&extension)
}
