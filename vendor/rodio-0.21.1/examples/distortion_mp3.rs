use std::error::Error;

use rodio::Source;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.mp3")?;
    // Apply distortion effect before appending to the sink
    let source = rodio::Decoder::try_from(file)?.distortion(4.0, 0.3);
    player.append(source);

    player.sleep_until_end();

    Ok(())
}
