use anyhow::{bail, Result};
use log::debug;

pub fn change_file_extension(path: &str, new_extension: &str) -> Result<String> {
    let extension_start = find_extension_start(path)?;
    Ok(format!("{}{new_extension}", &path[..extension_start]))
}

pub fn find_file_extension(path: &str) -> Result<String> {
    let extension_start = find_extension_start(path)?;
    Ok(path[extension_start..].to_owned())
}

fn find_extension_start(path: &str) -> Result<usize> {
    match path.rfind('.') {
        Some(index) => Ok(index),
        None => {
            debug!("Found no extension for the path: {path}");
            bail!("No extension found.")
        }
    }
}

pub fn add_quotes_if_whitespace_present(input: &str, output: &str) -> (String, String) {
    (
        wrap_with_quotes_if_whitespace(input),
        wrap_with_quotes_if_whitespace(output),
    )
}

fn wrap_with_quotes_if_whitespace(text: &str) -> String {
    if text.contains(' ') {
        format!("\"{text}\"")
    } else {
        text.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(result, format!("\"{input}\""));

        let input = "some/path/no_whitespace/file.mp4";
        let result = wrap_with_quotes_if_whitespace(input);
        assert_eq!(result, input);
    }
}
