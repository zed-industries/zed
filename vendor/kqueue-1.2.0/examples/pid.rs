#![allow(clippy::print_stdout, reason = "This is an example file")]

use std::env;
use std::io::Result;

fn watch_pid(pid: libc::pid_t) -> Result<()> {
    let mut watcher = kqueue::Watcher::new()?;

    watcher.add_pid(
        pid,
        kqueue::EventFilter::EVFILT_PROC,
        kqueue::FilterFlag::NOTE_EXIT,
    )?;

    watcher.watch()?;

    println!("Watching for events, press Ctrl+C to stop...");
    for ev in watcher.iter() {
        println!("{ev:?}");
    }

    Ok(())
}

fn main() {
    if let Some(pid) = env::args().nth(1) {
        if let Ok(npid) = pid.parse::<libc::pid_t>() {
            if let Err(err) = watch_pid(npid) {
                println!("{err:?}");
            }
        }
    } else {
        println!("Usage: cargo run --example pid <pid>");
    }
}
