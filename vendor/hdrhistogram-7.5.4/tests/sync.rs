#[cfg(all(feature = "sync", test))]
mod sync {
    use hdrhistogram::{sync::SyncHistogram, Histogram};
    use std::sync::{atomic, Arc};
    use std::{thread, time};

    const TRACKABLE_MAX: u64 = 3600 * 1000 * 1000;
    // Store up to 2 * 10^3 in single-unit precision. Can be 5 at most.
    const SIGFIG: u8 = 3;
    const TEST_VALUE_LEVEL: u64 = 4;

    #[test]
    fn record_through() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();
        h.record(TEST_VALUE_LEVEL).unwrap();
        assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn recorder_drop() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();
        let mut r = h.recorder();
        let jh = thread::spawn(move || {
            r += TEST_VALUE_LEVEL;
        });
        h.refresh();
        assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
        assert_eq!(h.len(), 1);
        jh.join().unwrap();
    }

    #[test]
    fn record_nodrop() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let mut r = h.recorder();
        let b = Arc::clone(&barrier);
        let jh = thread::spawn(move || {
            r += TEST_VALUE_LEVEL;
            b.wait();
        });
        h.refresh();
        assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
        assert_eq!(h.len(), 1);
        barrier.wait();
        jh.join().unwrap();
    }

    #[test]
    fn phase_timeout() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();
        h.record(TEST_VALUE_LEVEL).unwrap();
        let mut r = h.recorder();
        r += TEST_VALUE_LEVEL;
        h.refresh_timeout(time::Duration::from_millis(100));

        // second TEST_VALUE_LEVEL should not be visible
        // since no record happened after phase()
        assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn recorder_drop_staged() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        let barrier = Arc::new(std::sync::Barrier::new(2));
        let mut r = h.recorder();
        let b = Arc::clone(&barrier);
        let jh = thread::spawn(move || {
            let n = 10_000;
            for _ in 0..n {
                r += TEST_VALUE_LEVEL;
            }
            // one of the writes above will unblock the reader's first phase
            // the 1st barrier below ensures that the reader's second phase isn't passed by a write too
            // the 2nd barrier below ensures that there is at least one write to send on drop,
            // and that that write doesn't wake up the 2nd phase
            b.wait();
            r += TEST_VALUE_LEVEL;
            b.wait();
            drop(r);
            n + 1
        });
        h.refresh(); // this should be unblocked by one of the writes
        barrier.wait();
        barrier.wait();
        h.refresh(); // this will be unblocked by the recorder drop
        let n = jh.join().unwrap();
        h.refresh(); // no recorders, so we should be fine

        assert_eq!(h.count_at(TEST_VALUE_LEVEL), n);
        assert_eq!(h.len(), n);
    }

    #[test]
    fn phase_no_wait_after_drop() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        {
            let _ = h.recorder();
        }
        h.refresh(); // this shouldn't block since the recorder went away

        assert_eq!(h.len(), 0);
    }

    #[test]
    fn mt_record_static() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        let n = 16;
        let barrier = Arc::new(std::sync::Barrier::new(n + 1));
        let jhs: Vec<_> = (0..n)
            .map(|_| {
                let mut r = h.recorder();
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let n = 100_000;
                    for _ in 0..n {
                        r += TEST_VALUE_LEVEL;
                    }
                    barrier.wait();
                    n
                })
            })
            .collect();

        barrier.wait();
        h.refresh();

        assert_eq!(h.len(), jhs.into_iter().map(|r| r.join().unwrap()).sum());
    }

    #[test]
    fn refresh_times_out() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        let _r = h.recorder();
        h.refresh_timeout(time::Duration::from_millis(100));
    }

    #[test]
    fn mt_record_dynamic() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        let n = 16;
        let barrier = Arc::new(std::sync::Barrier::new(n + 1));
        let jhs: Vec<_> = (0..n)
            .map(|_| {
                let mut r = h.recorder();
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let n = 30_000;
                    for i in 0..n {
                        if i % 1_000 == 0 {
                            r = r.clone();
                        }
                        r += TEST_VALUE_LEVEL;
                    }
                    barrier.wait();
                    n as u64
                })
            })
            .collect();

        barrier.wait();
        h.refresh();

        assert_eq!(h.len(), jhs.into_iter().map(|r| r.join().unwrap()).sum());
    }

    #[test]
    fn idle_recorder() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        let barrier = Arc::new(std::sync::Barrier::new(2));
        let mut r = h.recorder();
        let i = r.idle();
        h.refresh(); // this should not block
        h.refresh(); // nor should this
        drop(i);
        let b = Arc::clone(&barrier);
        let jh = thread::spawn(move || {
            r += TEST_VALUE_LEVEL;
            b.wait();
        });
        barrier.wait();
        h.refresh(); // this will block!

        assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
        assert_eq!(h.len(), 1);
        jh.join().unwrap();
    }

    #[test]
    fn clone_idle_recorder() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();

        let done = Arc::new(atomic::AtomicBool::new(false));
        let r = h.recorder().into_idle();
        h.refresh(); // this should not block
        h.refresh(); // nor should this
        let mut r2 = r.recorder();
        let d = Arc::clone(&done);
        let jh = thread::spawn(move || {
            let mut i = 0;
            while !d.load(atomic::Ordering::SeqCst) {
                r2 += TEST_VALUE_LEVEL;
                i += 1;
            }
            i
        });
        h.refresh(); // this is released by the r2 += above
        let mut r = r.activate();
        // a call to refresh would block here now
        let d = Arc::clone(&done);
        let jh2 = thread::spawn(move || {
            let mut i = 0;
            while !d.load(atomic::Ordering::SeqCst) {
                r += TEST_VALUE_LEVEL;
                i += 1;
            }
            i
        });

        h.refresh(); // this is released by the second r2 _and_ the r += above

        // tell recorders to exit
        done.store(true, atomic::Ordering::SeqCst);
        h.refresh(); // shouldn't block for long
        let n = jh.join().unwrap() + jh2.join().unwrap();
        h.refresh(); // no more recorders, so shouldn't block

        assert_eq!(h.count_at(TEST_VALUE_LEVEL), n);
        assert_eq!(h.len(), n);
    }

    #[test]
    fn concurrent_writes() {
        let mut h: SyncHistogram<_> = Histogram::<u64>::new_with_max(TRACKABLE_MAX, SIGFIG)
            .unwrap()
            .into();
        h.record(TEST_VALUE_LEVEL).unwrap();
        let mut r = h.recorder();
        r += TEST_VALUE_LEVEL;
        h.refresh_timeout(time::Duration::from_millis(100));

        // second TEST_VALUE_LEVEL should not be visible
        // since no record happened after phase()
        assert_eq!(h.count_at(TEST_VALUE_LEVEL), 1);
        assert_eq!(h.len(), 1);
    }
}
