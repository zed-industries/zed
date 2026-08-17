use std::error::Error;
use std::io::BufReader;

use rodio::Source;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.wav")?;
    let decoder = rodio::Decoder::new(BufReader::new(file))?;
    let source = decoder.low_pass(200);
    player.append(source);

    player.sleep_until_end();

    Ok(())
}
