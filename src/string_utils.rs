use log::debug;

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input.trim().to_owned()
}

pub fn change_file_extension(path: &str, new_extension: &str) -> Result<String, &'static str> {
    match find_file_extension(path) {
        None => Err("File does have an invalid extension."),
        Some(extension) => {
            let new_path = path.replace(&extension, new_extension);
            Ok(new_path)
        }
    }
}

fn find_file_extension(path: &str) -> Option<String> {
    match rfind_utf8(path, '.') {
        Some(position) => {
            let video_name = &path[position..];
            debug!("Found video with an extension: {}", video_name);
            Some(video_name.to_owned())
        }
        None => {
            println!("Found no extension for the given path.");
            None
        }
    }
}

fn rfind_utf8(s: &str, chr: char) -> Option<usize> {
    s.chars()
        .rev()
        .position(|c| c == chr)
        .map(|rev_pos| s.chars().count() - rev_pos - 1)
}
