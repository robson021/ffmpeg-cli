## GUI and CLI tool for ffmpeg

[![CI](https://github.com/robson021/ffmpeg-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/robson021/ffmpeg-cli/actions/workflows/ci.yml)

- You need to have ffmpeg installed. If you are Mac user, use Homebrew - https://formulae.brew.sh/formula/ffmpeg
- Docs: https://ffmpeg.org/ffmpeg.html

Run `cargo run` to launch the Iced graphical interface, or `cargo run -- --cmd` to
use the interactive terminal menu. The GUI supports format conversion,
compression, YouTube-optimized conversion, and custom commands with optional
scale, audio bitrate, preset, and CRF settings. It previews the generated
command before execution. FFmpeg must be installed and available on `PATH`.