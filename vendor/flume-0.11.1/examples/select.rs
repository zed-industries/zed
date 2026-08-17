#[cfg(feature = "select")]
use flume::Selector;

#[cfg(feature = "select")]
fn main() {
    // Create two channels
    let (red_tx, red_rx) = flume::unbounded();
    let (blue_tx, blue_rx) = flume::unbounded();

    // Spawn two threads that each send a message into their respective channel
    std::thread::spawn(move || { let _ = red_tx.send("Red"); });
    std::thread::spawn(move || { let _ = blue_tx.send("Blue"); });

    // Race them to see which one sends their message first
    let winner = Selector::new()
        .recv(&red_rx, |msg| msg)
        .recv(&blue_rx, |msg| msg)
        .wait()
        .unwrap();

    println!("{} won!", winner);
}

#[cfg(not(feature = "select"))]
fn main() {}
