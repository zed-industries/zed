use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_std::channel::bounded as channel;
use async_std::task;
use rand::{Rng, SeedableRng};

#[cfg(not(target_os = "unknown"))]
use async_std::task::spawn;
#[cfg(target_os = "unknown")]
use async_std::task::spawn_local as spawn;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn ms(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn smoke() {
    task::block_on(async {
        let (s, r) = channel(1);

        s.send(7).await.unwrap();
        assert_eq!(r.recv().await.unwrap(), 7);

        s.send(8).await.unwrap();
        assert_eq!(r.recv().await.unwrap(), 8);

        drop(s);
        assert!(r.recv().await.is_err());
    });

    task::block_on(async {
        let (s, r) = channel(10);
        drop(r);
        assert!(s.send(1).await.is_err());
    });
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn capacity() {
    for i in 1..10 {
        let (s, r) = channel::<()>(i);
        assert_eq!(s.capacity().unwrap(), i);
        assert_eq!(r.capacity().unwrap(), i);
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn len_empty_full() {
    #![allow(clippy::cognitive_complexity)]
    task::block_on(async {
        let (s, r) = channel(2);

        assert_eq!(s.len(), 0);
        assert_eq!(s.is_empty(), true);
        assert_eq!(s.is_full(), false);
        assert_eq!(r.len(), 0);
        assert_eq!(r.is_empty(), true);
        assert_eq!(r.is_full(), false);

        s.send(()).await.unwrap();

        assert_eq!(s.len(), 1);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.is_full(), false);
        assert_eq!(r.len(), 1);
        assert_eq!(r.is_empty(), false);
        assert_eq!(r.is_full(), false);

        s.send(()).await.unwrap();

        assert_eq!(s.len(), 2);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.is_full(), true);
        assert_eq!(r.len(), 2);
        assert_eq!(r.is_empty(), false);
        assert_eq!(r.is_full(), true);

        let _ = r.recv().await;

        assert_eq!(s.len(), 1);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.is_full(), false);
        assert_eq!(r.len(), 1);
        assert_eq!(r.is_empty(), false);
        assert_eq!(r.is_full(), false);
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn recv() {
    task::block_on(async {
        let (s, r) = channel(100);

        spawn(async move {
            assert_eq!(r.recv().await.unwrap(), 7);
            task::sleep(ms(1000)).await;
            assert_eq!(r.recv().await.unwrap(), 8);
            task::sleep(ms(1000)).await;
            assert_eq!(r.recv().await.unwrap(), 9);
            assert!(r.recv().await.is_err());
        });

        task::sleep(ms(1500)).await;
        s.send(7).await.unwrap();
        s.send(8).await.unwrap();
        s.send(9).await.unwrap();
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn send() {
    task::block_on(async {
        let (s, r) = channel(1);

        spawn(async move {
            s.send(7).await.unwrap();
            task::sleep(ms(1000)).await;
            s.send(8).await.unwrap();
            task::sleep(ms(1000)).await;
            s.send(9).await.unwrap();
            task::sleep(ms(1000)).await;
            s.send(10).await.unwrap();
        });

        task::sleep(ms(1500)).await;
        assert_eq!(r.recv().await.unwrap(), 7);
        assert_eq!(r.recv().await.unwrap(), 8);
        assert_eq!(r.recv().await.unwrap(), 9);
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn recv_after_disconnect() {
    task::block_on(async {
        let (s, r) = channel(100);

        s.send(1).await.unwrap();
        s.send(2).await.unwrap();
        s.send(3).await.unwrap();

        drop(s);

        assert_eq!(r.recv().await.unwrap(), 1);
        assert_eq!(r.recv().await.unwrap(), 2);
        assert_eq!(r.recv().await.unwrap(), 3);
        assert!(r.recv().await.is_err());
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn len() {
    const COUNT: usize = 25_000;
    const CAP: usize = 1000;

    task::block_on(async {
        let (s, r) = channel(CAP);

        assert_eq!(s.len(), 0);
        assert_eq!(r.len(), 0);

        for _ in 0..CAP / 10 {
            for i in 0..50 {
                s.send(i).await.unwrap();
                assert_eq!(s.len(), i + 1);
            }

            for i in 0..50 {
                let _ = r.recv().await;
                assert_eq!(r.len(), 50 - i - 1);
            }
        }

        assert_eq!(s.len(), 0);
        assert_eq!(r.len(), 0);

        for i in 0..CAP {
            s.send(i).await.unwrap();
            assert_eq!(s.len(), i + 1);
        }

        for _ in 0..CAP {
            r.recv().await.unwrap();
        }

        assert_eq!(s.len(), 0);
        assert_eq!(r.len(), 0);

        let child = spawn({
            let r = r.clone();
            async move {
                for i in 0..COUNT {
                    assert_eq!(r.recv().await.unwrap(), i);
                    let len = r.len();
                    assert!(len <= CAP);
                }
            }
        });

        for i in 0..COUNT {
            s.send(i).await.unwrap();
            let len = s.len();
            assert!(len <= CAP);
        }

        child.await;

        assert_eq!(s.len(), 0);
        assert_eq!(r.len(), 0);
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn disconnect_wakes_receiver() {
    task::block_on(async {
        let (s, r) = channel::<()>(1);

        let child = spawn(async move {
            assert!(r.recv().await.is_err());
        });

        task::sleep(ms(1000)).await;
        drop(s);

        child.await;
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn spsc() {
    const COUNT: usize = 100_000;

    task::block_on(async {
        let (s, r) = channel(3);

        let child = spawn(async move {
            for i in 0..COUNT {
                assert_eq!(r.recv().await.unwrap(), i);
            }
            assert!(r.recv().await.is_err());
        });

        for i in 0..COUNT {
            s.send(i).await.unwrap();
        }
        drop(s);

        child.await;
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn mpmc() {
    const COUNT: usize = 25_000;
    const TASKS: usize = 4;

    task::block_on(async {
        let (s, r) = channel::<usize>(3);
        let v = (0..COUNT).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();
        let v = Arc::new(v);

        let mut tasks = Vec::new();

        for _ in 0..TASKS {
            let r = r.clone();
            let v = v.clone();
            tasks.push(spawn(async move {
                for _ in 0..COUNT {
                    let n = r.recv().await.unwrap();
                    v[n].fetch_add(1, Ordering::SeqCst);
                }
            }));
        }

        for _ in 0..TASKS {
            let s = s.clone();
            tasks.push(spawn(async move {
                for i in 0..COUNT {
                    s.send(i).await.unwrap();
                }
            }));
        }

        for t in tasks {
            t.await;
        }

        for c in v.iter() {
            assert_eq!(c.load(Ordering::SeqCst), TASKS);
        }
    });
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn oneshot() {
    const COUNT: usize = 10_000;

    task::block_on(async {
        for _ in 0..COUNT {
            let (s, r) = channel(1);

            let c1 = spawn(async move { r.recv().await.unwrap() });
            let c2 = spawn(async move { s.send(0).await.unwrap() });

            c1.await;
            c2.await;
        }
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn drops() {
    const RUNS: usize = 100;

    static DROPS: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, PartialEq)]
    struct DropCounter;

    impl Drop for DropCounter {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    for _ in 0..RUNS {
        let mut rng = rand_xorshift::XorShiftRng::seed_from_u64(0);
        task::block_on(async move {
            let steps = rng.gen_range(0..10_000);
            let additional = rng.gen_range(0..50);

            DROPS.store(0, Ordering::SeqCst);
            let (s, r) = channel::<DropCounter>(50);

            let child = spawn({
                let r = r.clone();
                async move {
                    for _ in 0..steps {
                        r.recv().await.unwrap();
                    }
                }
            });

            for _ in 0..steps {
                s.send(DropCounter).await.unwrap();
            }

            child.await;

            for _ in 0..additional {
                s.send(DropCounter).await.unwrap();
            }

            assert_eq!(DROPS.load(Ordering::SeqCst), steps);
            drop(s);
            drop(r);
            assert_eq!(DROPS.load(Ordering::SeqCst), steps + additional);
        })
    }
}
