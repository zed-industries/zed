use std::time::Duration;

use wasm_thread as thread;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().unwrap();
        console_error_panic_hook::set_once();
    }

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    log::info!("Available parallelism: {:?}", thread::available_parallelism());

    let mut threads = vec![];

    for _ in 0..2 {
        threads.push(thread::spawn(|| {
            for i in 1..3 {
                log::info!("hi number {} from the spawned thread {:?}!", i, thread::current().id());
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    for i in 1..3 {
        log::info!("hi number {} from the main thread {:?}!", i, thread::current().id());
    }

    // It's not possible to do a scope on the main thread, because blocking waits are not supported, but we can use
    // scope inside web workers.
    threads.push(thread::spawn(|| {
        log::info!("Start scope test on thread {:?}", thread::current().id());

        let mut a = vec![1, 2, 3];
        let mut x = 0;

        thread::scope(|s| {
            let handle = s.spawn(|| {
                log::info!("hello from the first scoped thread {:?}", thread::current().id());
                // We can borrow `a` here.
                log::info!("a = {:?}", &a);
                // Return a subslice of borrowed `a`
                &a[0..2]
            });

            // Wait for the returned value from first thread
            log::info!("a[0..2] = {:?}", handle.join().unwrap());

            s.spawn(|| {
                log::info!("hello from the second scoped thread {:?}", thread::current().id());
                // We can even mutably borrow `x` here,
                // because no other threads are using it.
                x += a[0] + a[2];
            });

            log::info!(
                "Hello from scope \"main\" thread {:?} inside scope.",
                thread::current().id()
            );
        });

        // After the scope, we can modify and access our variables again:
        a.push(4);
        assert_eq!(x, a.len());
        log::info!("Scope done x = {}, a.len() = {}", x, a.len());
    }));

    // Wait for all threads, otherwise program exits before threads finish execution.
    // We can't do blocking join on wasm main thread though, but the browser window will continue running.
    #[cfg(not(target_arch = "wasm32"))]
    for handle in threads {
        handle.join().unwrap();
    }
}
