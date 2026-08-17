use crate::runtime::{self, Runtime};

use std::sync::Arc;
use std::time::Duration;

#[test]
fn blocking_shutdown() {
    loom::model(|| {
        let v = Arc::new(());

        let rt = mk_runtime(1);
        {
            let _enter = rt.enter();
            for _ in 0..2 {
                let v = v.clone();
                crate::task::spawn_blocking(move || {
                    assert!(1 < Arc::strong_count(&v));
                });
            }
        }

        drop(rt);
        assert_eq!(1, Arc::strong_count(&v));
    });
}

#[test]
fn spawn_mandatory_blocking_should_always_run() {
    use crate::runtime::tests::loom_oneshot;
    loom::model(|| {
        let rt = runtime::Builder::new_current_thread().build().unwrap();

        let (tx, rx) = loom_oneshot::channel();
        let _enter = rt.enter();
        runtime::spawn_blocking(|| {});
        runtime::spawn_mandatory_blocking(move || {
            let _ = tx.send(());
        })
        .unwrap();

        drop(rt);

        // This call will deadlock if `spawn_mandatory_blocking` doesn't run.
        let () = rx.recv();
    });
}

#[test]
fn spawn_mandatory_blocking_should_run_even_when_shutting_down_from_other_thread() {
    use crate::runtime::tests::loom_oneshot;
    loom::model(|| {
        let rt = runtime::Builder::new_current_thread().build().unwrap();
        let handle = rt.handle().clone();

        // Drop the runtime in a different thread
        {
            loom::thread::spawn(move || {
                drop(rt);
            });
        }

        let _enter = handle.enter();
        let (tx, rx) = loom_oneshot::channel();
        let handle = runtime::spawn_mandatory_blocking(move || {
            let _ = tx.send(());
        });

        // handle.is_some() means that `spawn_mandatory_blocking`
        // promised us to run the blocking task
        if handle.is_some() {
            // This call will deadlock if `spawn_mandatory_blocking` doesn't run.
            let () = rx.recv();
        }
    });
}

#[test]
fn spawn_blocking_when_paused() {
    loom::model(|| {
        let rt = crate::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .unwrap();
        let handle = rt.handle();
        let _enter = handle.enter();
        let a = crate::task::spawn_blocking(|| {});
        let b = crate::task::spawn_blocking(|| {});
        rt.block_on(crate::time::timeout(Duration::from_millis(1), async move {
            a.await.expect("blocking task should finish");
            b.await.expect("blocking task should finish");
        }))
        .expect("timeout should not trigger");
    });
}

#[test]
/// See <https://github.com/tokio-rs/tokio/pull/7922>
fn spawn_blocking_then_shutdown() {
    loom::model(|| {
        let rt = crate::runtime::Builder::new_current_thread()
            .max_blocking_threads(1)
            .thread_keep_alive(Duration::from_secs(7200)) // don't let the thread exit on its own
            .build()
            .unwrap();
        let rt_hdl = rt.handle().clone();

        // Currently, there is no live blocking thread,
        // so `spawn_blocking` will spawn a new blocking thread.
        let jh0 = rt_hdl.spawn_blocking(|| {});
        loom::future::block_on(jh0).unwrap();

        // Now, there is a idle blocking threads park on the condvar,
        // so the following `spawn_blocking` will decrease the `num_idle_threads`
        // and then notify one of the idle threads to run the task.

        // this will decrease the `num_idle_threads`
        // and then notify one of the idle threads to run the task.
        let jh3 = rt_hdl.spawn_blocking(|| {});

        // shutdown the runtime, which also shutdown the blocking pool
        drop(rt);

        // loom will emulate two parrel operations:
        //
        // 1. the blocking thread is woken up on the condvar
        // 2. the main thread is waiting for the blocking thread to finish the task
        //
        // So, if the `num_idle_threads` is not counted correctly,
        // it will trigger the assertions inside the `Inner::run` function.
        let _ = loom::future::block_on(jh3);
    });
}

fn mk_runtime(num_threads: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(num_threads)
        .build()
        .unwrap()
}
