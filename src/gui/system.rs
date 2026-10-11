const SUPPORTED_EXT: &[&str; 8] =
    &["mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v"];

pub async fn pick_file() -> Option<String> {
    rfd::AsyncFileDialog::new()
        .set_title("Select a video file")
        .add_filter("Video", SUPPORTED_EXT)
        .add_filter("All files", &["*"])
        .pick_file()
        .await
        .map(|file| file.path().to_string_lossy().into_owned())
}

pub fn notify_done() {
    std::thread::spawn(|| {
        if let Err(error) = notify_rust::Notification::new()
            .summary("Done!")
            .body("FFmpeg finished successfully.")
            .show()
        {
            log::warn!("Could not show system notification: {error}");
        }
    });
}
