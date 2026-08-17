#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
#![cfg(not(target_os = "wasi"))] // Wasi doesn't support threads

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::sync::Notify;

#[test]
fn before_park_wakes_block_on_task() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    let woken = Arc::new(AtomicBool::new(false));
    let woken2 = woken.clone();

    let rt = Builder::new_current_thread()
        .enable_all()
        .on_thread_park(move || {
            // Only wake once to avoid busy loop if something goes wrong,
            // though in this test we expect it to unpark immediately.
            if !woken2.swap(true, Ordering::SeqCst) {
                notify2.notify_one();
            }
        })
        .build()
        .unwrap();

    rt.block_on(async {
        // This will block until `notify` is notified.
        // `before_park` should run when the runtime is about to park.
        // It will notify `notify`, which should wake this task.
        // The runtime should then see the task is woken and NOT park.
        notify.notified().await;
    });

    assert!(woken.load(Ordering::SeqCst));
}

#[test]
fn before_park_spawns_task() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    let woken = Arc::new(AtomicBool::new(false));
    let woken2 = woken.clone();

    let rt = Builder::new_current_thread()
        .enable_all()
        .on_thread_park(move || {
            if !woken2.swap(true, Ordering::SeqCst) {
                let notify = notify2.clone();
                tokio::spawn(async move {
                    notify.notify_one();
                });
            }
        })
        .build()
        .unwrap();

    rt.block_on(async {
        // This will block until `notify` is notified.
        // `before_park` should run when the runtime is about to park.
        // It will spawn a task that notifies `notify`.
        // The runtime should see the new task and NOT park.
        // If it parks, it will deadlock.
        notify.notified().await;
    });

    assert!(woken.load(Ordering::SeqCst));
}

#[test]
fn wake_from_other_thread_block_on() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let handle = rt.handle().clone();
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let th = std::thread::spawn(move || {
        // Give the main thread time to park
        std::thread::sleep(std::time::Duration::from_millis(5));
        handle.block_on(async move {
            notify2.notify_one();
        });
    });

    rt.block_on(async {
        notify.notified().await;
    });

    th.join().unwrap();
}
