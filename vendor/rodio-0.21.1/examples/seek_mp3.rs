use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.mp3")?;
    player.append(rodio::Decoder::try_from(file)?);

    std::thread::sleep(std::time::Duration::from_secs(2));
    player.try_seek(Duration::from_secs(0))?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    player.try_seek(Duration::from_secs(4))?;

    player.sleep_until_end();

    // This doesn't do anything since the sound has ended already.
    player.try_seek(Duration::from_secs(5))?;
    println!("seek example ended");

    Ok(())
}
