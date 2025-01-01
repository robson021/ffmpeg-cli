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
        None => Err("Input file does have an invalid extension."),
        Some(extension) => {
            let new_path = path.replace(&extension, new_extension);
            Ok(new_path)
        }
    }
}

fn find_file_extension(path: &str) -> Option<String> {
    match last_index_of_char(path, '.') {
        Some(idx) => {
            let ext = &path[idx..];
            debug!("Found video with an extension: {}", ext);
            Some(ext.to_owned())
        }
        None => {
            debug!("Found no extension for the path: {}", path);
            None
        }
    }
}

fn last_index_of_char(s: &str, to_find: char) -> Option<usize> {
    s.chars()
        .rev()
        .position(|c| c == to_find)
        .map(|rev_pos| s.chars().count() - rev_pos - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_find_last_index_of_char() {
        let s = "aaaaa.bbb.c";
        let idx = last_index_of_char(s, '.').unwrap();
        assert_eq!(idx, 9);

        let idx = last_index_of_char(s, '?');
        assert!(idx.is_none());
    }

    #[test]
    fn should_find_extension_of_the_file() {
        let ext = find_file_extension("aaaa/bbb/cc/video.mp4").unwrap();
        assert_eq!(ext, ".mp4");

        let ext = find_file_extension("abcd");
        assert!(ext.is_none());
    }

    #[test]
    fn should_change_extension() {
        let new_path = change_file_extension("aaa/bbb/cc/video.mp4", ".avi").unwrap();
        assert_eq!(new_path, "aaa/bbb/cc/video.avi");
    }
}
