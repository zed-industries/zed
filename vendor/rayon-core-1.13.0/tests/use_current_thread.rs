use rayon_core::ThreadPoolBuilder;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn use_current_thread_basic() {
    static JOIN_HANDLES: Mutex<Vec<JoinHandle<()>>> = Mutex::new(Vec::new());
    let pool = ThreadPoolBuilder::new()
        .num_threads(2)
        .use_current_thread()
        .spawn_handler(|builder| {
            let handle = thread::Builder::new().spawn(|| builder.run())?;
            JOIN_HANDLES.lock().unwrap().push(handle);
            Ok(())
        })
        .build()
        .unwrap();
    assert_eq!(rayon_core::current_thread_index(), Some(0));
    assert_eq!(
        JOIN_HANDLES.lock().unwrap().len(),
        1,
        "Should only spawn one extra thread"
    );

    let another_pool = ThreadPoolBuilder::new()
        .num_threads(2)
        .use_current_thread()
        .build();
    assert!(
        another_pool.is_err(),
        "Should error if the thread is already part of a pool"
    );

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    pool.spawn(move || {
        assert_ne!(rayon_core::current_thread_index(), Some(0));
        // This should execute even if the current thread is blocked, since we have two threads in
        // the pool.
        let (started, condvar) = &*pair2;
        *started.lock().unwrap() = true;
        condvar.notify_one();
    });

    let _guard = pair
        .1
        .wait_while(pair.0.lock().unwrap(), |ran| !*ran)
        .unwrap();
    std::mem::drop(pool); // Drop the pool.

    // Wait until all threads have actually exited. This is not really needed, other than to
    // reduce noise of leak-checking tools.
    for handle in std::mem::take(&mut *JOIN_HANDLES.lock().unwrap()) {
        let _ = handle.join();
    }
}
