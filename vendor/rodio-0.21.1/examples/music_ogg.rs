use std::{error::Error, io::Cursor};

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = include_bytes!("../assets/music.ogg");
    let cursor = Cursor::new(file);
    player.append(rodio::Decoder::try_from(cursor)?);

    player.sleep_until_end();

    Ok(())
}
