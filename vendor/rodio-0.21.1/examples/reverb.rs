use rodio::Source;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.ogg")?;
    let source = rodio::Decoder::try_from(file)?;
    let with_reverb = source.buffered().reverb(Duration::from_millis(40), 0.7);
    player.append(with_reverb);

    player.sleep_until_end();

    Ok(())
}
