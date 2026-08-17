#![allow(clippy::print_stdout, reason = "This is an example file")]

use std::env;
use std::io::Result;

fn watch_file(filename: &str) -> Result<()> {
    let mut watcher = kqueue::Watcher::new()?;

    watcher.add_filename(
        filename,
        kqueue::EventFilter::EVFILT_VNODE,
        kqueue::FilterFlag::NOTE_DELETE
            | kqueue::FilterFlag::NOTE_WRITE
            | kqueue::FilterFlag::NOTE_RENAME,
    )?;

    watcher.watch()?;

    println!("Watching for events, press Ctrl+C to stop...");
    for ev in watcher.iter() {
        println!("{ev:?}");
    }

    Ok(())
}

fn main() {
    if let Some(filename) = env::args().nth(1) {
        if let Err(err) = watch_file(&filename) {
            println!("{err:?}");
        }
    } else {
        println!("Usage: cargo run --example file <filename>");
    }
}
