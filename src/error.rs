use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessFailure {
    #[error("Failed to spawn process.")]
    Spawn,
    #[error("Failed to await process.")]
    Await,
    #[error("Failed to execute command. Status: {0}.")]
    CommandExecution(i32),
}

#[derive(Debug, Error)]
pub enum TranscoderError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Failed to check file existence: {0}.")]
    CouldNotCheckFileExistence(String),
    #[error("Invalid command.")]
    InvalidCommand,
    #[error("Input and output formats are the same.")]
    SameInputAndOutput,
    #[error("Unsupported operation.")]
    UnsupportedOperation,
    #[error("Aborted. {0}")]
    AbortTranscoding(String),
}
