//! Tests for the list channel flavor.

extern crate crossbeam_utils;
extern crate rand;

use std::any::Any;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use flume::{unbounded, Receiver};
use flume::{RecvError, RecvTimeoutError, TryRecvError};
use flume::{SendError, SendTimeoutError, TrySendError};
use crossbeam_utils::thread::scope;
use rand::{thread_rng, Rng};

fn ms(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

#[test]
fn smoke() {
    let (s, r) = unbounded();
    s.try_send(7).unwrap();
    assert_eq!(r.try_recv(), Ok(7));

    s.send(8).unwrap();
    assert_eq!(r.recv(), Ok(8));

    assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
    assert_eq!(r.recv_timeout(ms(1000)), Err(RecvTimeoutError::Timeout));
}

#[test]
fn capacity() {
    let (s, r) = unbounded::<()>();
    assert_eq!(s.capacity(), None);
    assert_eq!(r.capacity(), None);
}

#[test]
fn len_empty_full() {
    let (s, r) = unbounded();

    assert_eq!(s.len(), 0);
    assert_eq!(s.is_empty(), true);
    assert_eq!(s.is_full(), false);
    assert_eq!(r.len(), 0);
    assert_eq!(r.is_empty(), true);
    assert_eq!(r.is_full(), false);

    s.send(()).unwrap();

    assert_eq!(s.len(), 1);
    assert_eq!(s.is_empty(), false);
    assert_eq!(s.is_full(), false);
    assert_eq!(r.len(), 1);
    assert_eq!(r.is_empty(), false);
    assert_eq!(r.is_full(), false);

    r.recv().unwrap();

    assert_eq!(s.len(), 0);
    assert_eq!(s.is_empty(), true);
    assert_eq!(s.is_full(), false);
    assert_eq!(r.len(), 0);
    assert_eq!(r.is_empty(), true);
    assert_eq!(r.is_full(), false);
}

#[test]
fn try_recv() {
    let (s, r) = unbounded();

    scope(|scope| {
        scope.spawn(move |_| {
            assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
            thread::sleep(ms(1500));
            assert_eq!(r.try_recv(), Ok(7));
            thread::sleep(ms(500));
            assert_eq!(r.try_recv(), Err(TryRecvError::Disconnected));
        });
        scope.spawn(move |_| {
            thread::sleep(ms(1000));
            s.send(7).unwrap();
        });
    })
    .unwrap();
}

#[test]
fn recv() {
    let (s, r) = unbounded();

    scope(|scope| {
        scope.spawn(move |_| {
            assert_eq!(r.recv(), Ok(7));
            thread::sleep(ms(1000));
            assert_eq!(r.recv(), Ok(8));
            thread::sleep(ms(1000));
            assert_eq!(r.recv(), Ok(9));
            assert!(r.recv().is_err());
        });
        scope.spawn(move |_| {
            thread::sleep(ms(1500));
            s.send(7).unwrap();
            s.send(8).unwrap();
            s.send(9).unwrap();
        });
    })
    .unwrap();
}

#[test]
fn recv_timeout() {
    let (s, r) = unbounded::<i32>();

    scope(|scope| {
        scope.spawn(move |_| {
            assert_eq!(r.recv_timeout(ms(1000)), Err(RecvTimeoutError::Timeout));
            assert_eq!(r.recv_timeout(ms(1000)), Ok(7));
            assert_eq!(
                r.recv_timeout(ms(1000)),
                Err(RecvTimeoutError::Disconnected)
            );
        });
        scope.spawn(move |_| {
            thread::sleep(ms(1500));
            s.send(7).unwrap();
        });
    })
    .unwrap();
}

#[test]
fn try_send() {
    let (s, r) = unbounded();
    for i in 0..1000 {
        assert_eq!(s.try_send(i), Ok(()));
    }

    drop(r);
    assert_eq!(s.try_send(777), Err(TrySendError::Disconnected(777)));
}

#[test]
fn send() {
    let (s, r) = unbounded();
    for i in 0..1000 {
        assert_eq!(s.send(i), Ok(()));
    }

    drop(r);
    assert_eq!(s.send(777), Err(SendError(777)));
}

#[test]
fn send_timeout() {
    let (s, r) = unbounded();
    for i in 0..1000 {
        assert_eq!(s.send_timeout(i, ms(i as u64)), Ok(()));
    }

    drop(r);
    assert_eq!(
        s.send_timeout(777, ms(0)),
        Err(SendTimeoutError::Disconnected(777))
    );
}

#[test]
fn send_after_disconnect() {
    let (s, r) = unbounded();

    s.send(1).unwrap();
    s.send(2).unwrap();
    s.send(3).unwrap();

    drop(r);

    assert_eq!(s.send(4), Err(SendError(4)));
    assert_eq!(s.try_send(5), Err(TrySendError::Disconnected(5)));
    assert_eq!(
        s.send_timeout(6, ms(0)),
        Err(SendTimeoutError::Disconnected(6))
    );
}

#[test]
fn recv_after_disconnect() {
    let (s, r) = unbounded();

    s.send(1).unwrap();
    s.send(2).unwrap();
    s.send(3).unwrap();

    drop(s);

    assert_eq!(r.recv(), Ok(1));
    assert_eq!(r.recv(), Ok(2));
    assert_eq!(r.recv(), Ok(3));
    assert!(r.recv().is_err());
}

#[test]
fn len() {
    let (s, r) = unbounded();

    assert_eq!(s.len(), 0);
    assert_eq!(r.len(), 0);

    for i in 0..50 {
        s.send(i).unwrap();
        assert_eq!(s.len(), i + 1);
    }

    for i in 0..50 {
        r.recv().unwrap();
        assert_eq!(r.len(), 50 - i - 1);
    }

    assert_eq!(s.len(), 0);
    assert_eq!(r.len(), 0);
}

#[test]
fn disconnect_wakes_receiver() {
    let (s, r) = unbounded::<()>();

    scope(|scope| {
        scope.spawn(move |_| {
            assert!(r.recv().is_err());
        });
        scope.spawn(move |_| {
            thread::sleep(ms(1000));
            drop(s);
        });
    })
    .unwrap();
}

#[test]
fn spsc() {
    const COUNT: usize = 100_000;

    let (s, r) = unbounded();

    scope(|scope| {
        scope.spawn(move |_| {
            for i in 0..COUNT {
                assert_eq!(r.recv(), Ok(i));
            }
            assert!(r.recv().is_err());
        });
        scope.spawn(move |_| {
            for i in 0..COUNT {
                s.send(i).unwrap();
            }
        });
    })
    .unwrap();
}

#[test]
fn mpmc() {
    const COUNT: usize = 25_000;
    const THREADS: usize = 4;

    let (s, r) = unbounded::<usize>();
    let v = (0..COUNT).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();

    scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..COUNT {
                    let n = r.recv().unwrap();
                    v[n].fetch_add(1, Ordering::SeqCst);
                }
            });
        }
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..COUNT {
                    s.send(i).unwrap();
                }
            });
        }
    })
    .unwrap();

    assert_eq!(r.try_recv(), Err(TryRecvError::Empty));

    for c in v {
        assert_eq!(c.load(Ordering::SeqCst), THREADS);
    }
}

#[test]
fn stress_oneshot() {
    const COUNT: usize = 10_000;

    for _ in 0..COUNT {
        let (s, r) = unbounded();

        scope(|scope| {
            scope.spawn(|_| r.recv().unwrap());
            scope.spawn(|_| s.send(0).unwrap());
        })
        .unwrap();
    }
}

#[test]
fn stress_iter() {
    const COUNT: usize = 100_000;

    let (request_s, request_r) = unbounded();
    let (response_s, response_r) = unbounded();

    scope(|scope| {
        scope.spawn(move |_| {
            let mut count = 0;
            loop {
                for x in response_r.try_iter() {
                    count += x;
                    if count == COUNT {
                        return;
                    }
                }
                request_s.send(()).unwrap();
            }
        });

        for _ in request_r.iter() {
            if response_s.send(1).is_err() {
                break;
            }
        }
    })
    .unwrap();
}

#[test]
fn stress_timeout_two_threads() {
    const COUNT: usize = 100;

    let (s, r) = unbounded();

    scope(|scope| {
        scope.spawn(|_| {
            for i in 0..COUNT {
                if i % 2 == 0 {
                    thread::sleep(ms(50));
                }
                s.send(i).unwrap();
            }
        });

        scope.spawn(|_| {
            for i in 0..COUNT {
                if i % 2 == 0 {
                    thread::sleep(ms(50));
                }
                loop {
                    if let Ok(x) = r.recv_timeout(ms(10)) {
                        assert_eq!(x, i);
                        break;
                    }
                }
            }
        });
    })
    .unwrap();
}

#[test]
fn drops() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, PartialEq)]
    struct DropCounter;

    impl Drop for DropCounter {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    let mut rng = thread_rng();

    for _ in 0..100 {
        let steps = rng.gen_range(0..10_000);
        let additional = rng.gen_range(0..1000);

        DROPS.store(0, Ordering::SeqCst);
        let (s, r) = unbounded::<DropCounter>();

        scope(|scope| {
            scope.spawn(|_| {
                for _ in 0..steps {
                    r.recv().unwrap();
                }
            });

            scope.spawn(|_| {
                for _ in 0..steps {
                    s.send(DropCounter).unwrap();
                }
            });
        })
        .unwrap();

        for _ in 0..additional {
            s.try_send(DropCounter).unwrap();
        }

        assert_eq!(DROPS.load(Ordering::SeqCst), steps);
        drop(s);
        drop(r);
        assert_eq!(DROPS.load(Ordering::SeqCst), steps + additional);
    }
}

#[test]
fn linearizable() {
    const COUNT: usize = 25_000;
    const THREADS: usize = 4;

    let (s, r) = unbounded();

    scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..COUNT {
                    s.send(0).unwrap();
                    r.try_recv().unwrap();
                }
            });
        }
    })
    .unwrap();
}

// #[test]
// fn fairness() {
//     const COUNT: usize = 10_000;

//     let (s1, r1) = unbounded::<()>();
//     let (s2, r2) = unbounded::<()>();

//     for _ in 0..COUNT {
//         s1.send(()).unwrap();
//         s2.send(()).unwrap();
//     }

//     let mut hits = [0usize; 2];
//     for _ in 0..COUNT {
//         select! {
//             recv(r1) -> _ => hits[0] += 1,
//             recv(r2) -> _ => hits[1] += 1,
//         }
//     }
//     assert!(hits.iter().all(|x| *x >= COUNT / hits.len() / 2));
// }

// #[test]
// fn fairness_duplicates() {
//     const COUNT: usize = 10_000;

//     let (s, r) = unbounded();

//     for _ in 0..COUNT {
//         s.send(()).unwrap();
//     }

//     let mut hits = [0usize; 5];
//     for _ in 0..COUNT {
//         select! {
//             recv(r) -> _ => hits[0] += 1,
//             recv(r) -> _ => hits[1] += 1,
//             recv(r) -> _ => hits[2] += 1,
//             recv(r) -> _ => hits[3] += 1,
//             recv(r) -> _ => hits[4] += 1,
//         }
//     }
//     assert!(hits.iter().all(|x| *x >= COUNT / hits.len() / 2));
// }

// #[test]
// fn recv_in_send() {
//     let (s, r) = unbounded();
//     s.send(()).unwrap();

//     select! {
//         send(s, assert_eq!(r.recv(), Ok(()))) -> _ => {}
//     }
// }

#[test]
fn channel_through_channel() {
    const COUNT: usize = 1000;

    type T = Box<dyn Any + Send>;

    let (s, r) = unbounded::<T>();

    scope(|scope| {
        scope.spawn(move |_| {
            let mut s = s;

            for _ in 0..COUNT {
                let (new_s, new_r) = unbounded();
                let new_r: T = Box::new(Some(new_r));

                s.send(new_r).unwrap();
                s = new_s;
            }
        });

        scope.spawn(move |_| {
            let mut r = r;

            for _ in 0..COUNT {
                r = r
                    .recv()
                    .unwrap()
                    .downcast_mut::<Option<Receiver<T>>>()
                    .unwrap()
                    .take()
                    .unwrap()
            }
        });
    })
    .unwrap();
}
