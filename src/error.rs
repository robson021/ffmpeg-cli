use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProcessFailure {
    Spawn,
    Await,
    CommandExecution(i32),
}

#[derive(Debug)]
pub enum TranscoderError {
    FileNotFound(String),
    InvalidCommand,
    SameInputAndOutput,
    FileAlreadyInProperOutput,
    UnsupportedOperation,
}

impl Display for TranscoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscoderError::FileNotFound(path) => format!("File not found: {}.", path).fmt(f),
            TranscoderError::InvalidCommand => f.write_str("Invalid command."),
            TranscoderError::SameInputAndOutput => {
                f.write_str("Input and output formats are the same.")
            }
            TranscoderError::FileAlreadyInProperOutput => {
                f.write_str("The file already has recommended codecs and mp4 format.")
            }
            TranscoderError::UnsupportedOperation => f.write_str("Unsupported operation."),
        }
    }
}

impl Display for ProcessFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessFailure::Await => f.write_str("Failed to await process."),
            ProcessFailure::Spawn => f.write_str("Failed to spawn process."),
            ProcessFailure::CommandExecution(status) => {
                format!("Failed to execute command. Status: {}.", status).fmt(f)
            }
        }
    }
}

impl Error for TranscoderError {}
impl Error for ProcessFailure {}
