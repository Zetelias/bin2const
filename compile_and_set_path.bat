::# Compile in release mode
cargo build --release --bin bin2const
::# Set the path to the compiled binary
setx PATH "%PATH%;%cd%\target\release"
