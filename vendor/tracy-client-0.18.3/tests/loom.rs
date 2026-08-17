#[cfg(loom)]
mod loom {

    use loom::thread;
    use tracy_client::Client;

    fn model<F>(f: F)
    where
        F: Fn() + Sync + Send + 'static,
    {
        #[cfg(not(loom))]
        {
            f()
        }
        #[cfg(loom)]
        {
            let mut builder = loom::model::Builder::new();
            builder.preemption_bound = Some(3);
            builder.check(f)
        }
    }

    fn main() {
        model(|| {
            let client = Client::start();
            assert!(Client::is_running());
            drop(client);
            unsafe {
                ___tracy_shutdown_profiler();
            }
        });

        model(|| {
            let t1 = thread::spawn(|| {
                let client = Client::start();
                assert!(Client::is_running());
                drop(client);
            });
            let client = Client::start();
            assert!(Client::is_running());
            drop(client);
            t1.join().unwrap();
            unsafe {
                ___tracy_shutdown_profiler();
            }
        });

        model(|| {
            let t1 = thread::spawn(move || {
                let client = Client::start();
                assert!(Client::is_running());
                let client2 = client.clone();
                assert!(Client::is_running());
                drop(client);
                assert!(Client::is_running());
                drop(client2);
            });
            let client = Client::start();
            assert!(Client::is_running());
            let client2 = client.clone();
            assert!(Client::is_running());
            drop(client2);
            assert!(Client::is_running());
            drop(client);
            t1.join().unwrap();
            unsafe {
                ___tracy_shutdown_profiler();
            }
        });
    }
}

fn main() {
    #[cfg(loom)]
    loom::main();
}
