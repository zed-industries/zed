use rodio::source::AutomaticGainControlSettings;
use rodio::{wav_to_file, Source};
use std::error::Error;

/// Converts mp3 file to a wav file.
/// This example does not use any audio devices
/// and can be used in build configurations without `cpal` feature enabled.
fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("assets/music.mp3")?;
    let mut audio = rodio::Decoder::try_from(file)?
        .automatic_gain_control(AutomaticGainControlSettings::default())
        .speed(0.8);

    let wav_path = "music_mp3_converted.wav";
    println!("Storing converted audio into {}", wav_path);
    wav_to_file(&mut audio, wav_path)?;

    Ok(())
}
