// This example requires Rust 1.70.0 or newer.
#![allow(clippy::incompatible_msrv)]

//! This example demonstrates how to write an eviction listener that will reinsert
//! the expired entries.
//!
//! We cannot make the eviction listener directly reinsert the entries, because it
//! will lead to a deadlock in some conditions. Instead, we will create a worker
//! thread to do the reinsertion, and create a mpsc channel to send commands from the
//! eviction listener to the worker thread.

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc, Mutex, OnceLock,
    },
    thread,
    time::{Duration, Instant},
};

use moka::{notification::RemovalCause, sync::Cache};

/// The cache key type.
pub type Key = String;
/// The cache value type.
pub type Value = u32;

/// Command for the worker thread.
pub enum Command {
    /// (Re)insert the entry with the given key and value.
    Insert(Key, Value),
    /// Shutdown the worker thread.
    Shutdown,
}

fn main() {
    // Create a multi-producer single-consumer (mpsc) channel to send commands
    // from the eviction listener to the worker thread.
    let (snd, rcv) = mpsc::channel();

    // Wrap the Sender (snd) with a Mutex and set to a static OnceLock.
    //
    // Cache requires an eviction listener to be Sync as it will be executed by
    // multiple threads. However the Sender (snd) of the channel is not Sync, so the
    // eviction listener cannot capture the Sender directly.
    //
    // We are going to solve this by making the Sender globally accessible via the
    // static OnceLock, and make the eviction listener to clone it per thread.
    static SND: OnceLock<Mutex<Sender<Command>>> = OnceLock::new();
    #[cfg_attr(beta_clippy, allow(clippy::incompatible_msrv))]
    // `set` is stable since 1.70.0.
    SND.set(Mutex::new(snd.clone())).unwrap();

    // Create the eviction listener.
    let listener = move |key: Arc<String>, value: u32, cause: RemovalCause| {
        // Keep a clone of the Sender in our thread-local variable, so that we can
        // send a command without locking the Mutex every time.
        thread_local! {
            #[cfg_attr(beta_clippy, allow(clippy::incompatible_msrv))]
            // `get` is stable since 1.70.0.
            static THREAD_SND: Sender<Command> = SND.get().unwrap().lock().unwrap().clone();
        }

        println!("{} was evicted. value: {} ({:?})", key, value, cause);

        // If the entry was removed due to expiration, send a command to the channel
        // to reinsert the entry with a modified value.
        if cause == RemovalCause::Expired {
            let new_value = value * 2;
            let command = Command::Insert(key.to_string(), new_value);
            THREAD_SND.with(|snd| snd.send(command).expect("Cannot send"));
        }

        // Do nothing if the entry was removed by one of the following reasons:
        // - Reached to the capacity limit. (RemovalCause::Size)
        // - Manually invalidated. (RemovalCause::Explicit)
    };

    const MAX_CAPACITY: u64 = 7;
    const TTL: Duration = Duration::from_secs(3);

    // Create a cache with the max capacity, time-to-live and the eviction listener.
    let cache = Arc::new(
        Cache::builder()
            .max_capacity(MAX_CAPACITY)
            .time_to_live(TTL)
            .eviction_listener(listener)
            .build(),
    );

    // Spawn the worker thread that receives commands from the channel and reinserts
    // the entries.
    let worker1 = {
        let cache = Arc::clone(&cache);

        thread::spawn(move || {
            // Repeat until receiving a shutdown command.
            loop {
                match rcv.recv() {
                    Ok(Command::Insert(key, value)) => {
                        println!("Reinserting {} with value {}.", key, value);
                        cache.insert(key, value);
                    }
                    Ok(Command::Shutdown) => break,
                    Err(e) => {
                        eprintln!("Cannot receive a command: {:?}", e);
                        break;
                    }
                }
            }

            println!("Shutdown the worker thread.");
        })
    };

    // Spawn another worker thread that calls `cache.run_pending_tasks()` every 300
    // milliseconds.
    let shutdown = Arc::new(AtomicBool::new(false));
    let worker2 = {
        let cache = Arc::clone(&cache);
        let shutdown = Arc::clone(&shutdown);

        thread::spawn(move || {
            let interval = Duration::from_millis(300);
            let mut sleep_duration = interval;

            // Repeat until the shutdown latch is set.
            while !shutdown.load(Ordering::Relaxed) {
                thread::sleep(sleep_duration);
                let start = Instant::now();
                cache.run_pending_tasks();
                sleep_duration = interval.saturating_sub(start.elapsed());
            }
        })
    };

    // Insert 9 entries.
    // - The last 2 entries will be evicted due to the capacity limit.
    // - The remaining 7 entries will be evicted after 3 seconds, and then the worker
    //   thread will reinsert them with modified values.
    for i in 1..=9 {
        thread::sleep(Duration::from_millis(100));
        let key = i.to_string();
        let value = i;
        println!("Inserting {} with value {}.", key, value);
        cache.insert(key, value);
    }

    // Wait for 8 seconds.
    thread::sleep(Duration::from_secs(8));

    // Shutdown the worker threads.
    snd.send(Command::Shutdown).expect("Cannot send");
    worker1.join().expect("The worker thread 1 panicked");

    shutdown.store(true, Ordering::Relaxed);
    worker2.join().expect("The worker thread 2 panicked");
}
