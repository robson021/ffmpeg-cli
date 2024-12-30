#[derive(Debug, Clone, Default)]
enum CommandType {
    #[default]
    Compress,
    ConvertFormat,
    MultiTask,
}

#[derive(Default, Debug, derive_builder::Builder)]
#[builder(setter(into))]
pub struct FfmpegCommand {
    command_type: CommandType,
    input_file: String,
    output_file: String,
    codec: String,
}
