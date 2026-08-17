use std::error::Error;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.wav")?;
    player.append(rodio::Decoder::try_from(file)?);

    // lets increment a number after `music.wav` has played. We are going to use atomics
    // however you could also use a `Mutex` or send a message through a `std::sync::mpsc`.
    let playlist_pos = Arc::new(AtomicU32::new(0));

    // The closure needs to own everything it uses. We move a clone of
    // playlist_pos into the closure. That way we can still access playlist_pos
    // after appending the EmptyCallback.
    let playlist_pos_clone = playlist_pos.clone();
    player.append(rodio::source::EmptyCallback::new(Box::new(move || {
        println!("empty callback is now running");
        playlist_pos_clone.fetch_add(1, Ordering::Relaxed);
    })));

    assert_eq!(playlist_pos.load(Ordering::Relaxed), 0);
    println!(
        "playlist position is: {}",
        playlist_pos.load(Ordering::Relaxed)
    );
    player.sleep_until_end();
    assert_eq!(playlist_pos.load(Ordering::Relaxed), 1);
    println!(
        "playlist position is: {}",
        playlist_pos.load(Ordering::Relaxed)
    );

    Ok(())
}
