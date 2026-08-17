//! Plays a tone alternating between right and left ears, with right being first.

use rodio::Source;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/RL.ogg")?;
    player.append(rodio::Decoder::try_from(file)?.amplify(0.2));

    player.sleep_until_end();

    Ok(())
}
