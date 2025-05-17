use log::debug;

pub fn change_file_extension(path: &str, new_extension: &str) -> Result<String, &'static str> {
    let extension = find_file_extension(path)?;
    let new_path = path.replace(&extension, new_extension);
    Ok(new_path)
}

pub fn find_file_extension(path: &str) -> Result<String, &'static str> {
    match last_index_of_char(path, '.') {
        Some(idx) => {
            let ext = &path[idx..];
            debug!("Found video with an extension: {}", ext);
            Ok(ext.to_owned())
        }
        None => {
            debug!("Found no extension for the path: {}", path);
            Err("No extension found.")
        }
    }
}

fn last_index_of_char(s: &str, to_find: char) -> Option<usize> {
    s.chars()
        .rev()
        .position(|c| c == to_find)
        .map(|rev_pos| s.chars().count() - rev_pos - 1)
}

pub fn add_quotes_if_whitespace_present(input: &str, output: &str) -> (String, String) {
    let input = wrap_with_quotes_if_whitespace(input);
    let output = wrap_with_quotes_if_whitespace(output);
    (input, output)
}

#[inline(always)]
fn wrap_with_quotes_if_whitespace(s: &str) -> String {
    if s.contains(' ') {
        let mut with_quotes = String::from("\"");
        with_quotes.push_str(s);
        with_quotes.push('"');
        with_quotes
    } else {
        s.to_owned()
    }
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
        assert!(ext.is_err());
    }

    #[test]
    fn should_change_extension() {
        let new_path = change_file_extension("aaa/bbb/cc/video.mp4", ".avi").unwrap();
        assert_eq!(new_path, "aaa/bbb/cc/video.avi");
    }

    #[test]
    fn should_clean_input_and_output_whitespaces() {
        let input = "/some/path with whitespaces/file name.mp4";
        let result = wrap_with_quotes_if_whitespace(input);
        assert_eq!(result, format!("\"{}\"", input));

        let input = "some/path/no_whitespace/file.mp4";
        let result = wrap_with_quotes_if_whitespace(input);
        assert_eq!(result, input);
    }
}
