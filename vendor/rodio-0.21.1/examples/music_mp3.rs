use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.mp3")?;
    player.append(rodio::Decoder::try_from(file)?);

    player.sleep_until_end();

    Ok(())
}
