use inquire::Select;
use rodio::microphone::{self, MicrophoneBuilder};
use rodio::Source;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let input = Select::new(
        "Which microphone do you want to use?",
        microphone::available_inputs()?,
    )
    .prompt()?;

    let input = MicrophoneBuilder::new()
        .device(input)?
        .default_config()?
        .open_stream()?;

    println!("Recording 5 seconds of input to play back");
    let recording = input.take_duration(Duration::from_secs(5)).record();

    println!("Playing the recording");
    let mut output = rodio::DeviceSinkBuilder::open_default_sink()?;
    output.mixer().add(recording);

    thread::sleep(Duration::from_secs(5));

    output.log_on_drop(false);
    Ok(())
}
