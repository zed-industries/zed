//! These tests are converted from the [C11 memory ordering page][spec].
//!
//!
//! [spec]: https://en.cppreference.com/w/cpp/atomic/memory_order

/// https://en.cppreference.com/w/cpp/atomic/memory_order#Relaxed_ordering
///
/// This test is ignored because loom cannot fully model `Ordering::Relaxed`.
#[test]
#[should_panic]
#[ignore]
fn relaxed() {
    use loom::sync::atomic::AtomicUsize;
    use loom::thread;
    use std::sync::atomic::Ordering::Relaxed;

    loom::model(|| {
        let x1: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let x2 = x1;
        let y1: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y2 = y1;

        let t1 = thread::spawn(move || {
            let r1 = y1.load(Relaxed);
            x1.store(r1, Relaxed);
            r1
        });
        let t2 = thread::spawn(move || {
            let r2 = x2.load(Relaxed);
            y2.store(42, Relaxed);
            r2
        });

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        if r1 == 42 && r2 == 42 {
            panic!("This case is possible with Relaxed, so we should hit this panic.");
        }
    });
}

/// https://en.cppreference.com/w/cpp/atomic/memory_order#Sequentially-consistent_ordering
///
/// This is the SeqCst example modified to use AcqRel to see that we indeed exercise all the
/// possible executions.
#[test]
fn acq_rel() {
    use loom::sync::atomic::AtomicBool;
    use loom::thread;
    use std::sync::atomic::Ordering;

    let mut builder = loom::model::Builder::new();
    // The yield loop makes loom really sad without this:
    builder.preemption_bound = Some(1);

    let seen: &'static _ = Box::leak(Box::new(std::sync::Mutex::new(
        std::collections::HashSet::new(),
    )));

    builder.check(move || {
        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(std::sync::atomic::AtomicUsize::new(0)));

        // NOTE: done in this thread after spawning
        // thread::spawn(move || {
        //     x.store(true, Ordering::Release);
        // });
        thread::spawn(move || {
            y.store(true, Ordering::Release);
        });
        let t1 = thread::spawn(move || {
            while !x.load(Ordering::Acquire) {
                loom::thread::yield_now();
            }
            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        let t2 = thread::spawn(move || {
            while !y.load(Ordering::Acquire) {
                loom::thread::yield_now();
            }
            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        x.store(true, Ordering::Release);

        t1.join().unwrap();
        t2.join().unwrap();
        // Read z but not while holding the lock, since the read goes into loom innards.
        let z = z.load(Ordering::SeqCst);
        seen.lock().unwrap().insert(z);
    });
    let seen = seen.lock().unwrap();
    assert!(seen.contains(&0));
    assert!(seen.contains(&1));
    assert!(seen.contains(&2));
    assert_eq!(seen.len(), 3);
}

/// https://en.cppreference.com/w/cpp/atomic/memory_order#Sequentially-consistent_ordering
///
/// This test currently fails because loom executes a permutation that isn't legal under `SeqCst`
/// according to the spec in which `z == 0`.
#[test]
#[ignore]
fn test_seq_cst() {
    use loom::sync::atomic::AtomicBool;
    use loom::thread;
    use std::sync::atomic::Ordering;

    let mut builder = loom::model::Builder::new();
    // The yield loop makes loom really sad without this:
    builder.preemption_bound = Some(1);

    let seen: &'static _ = Box::leak(Box::new(std::sync::Mutex::new(
        std::collections::HashSet::new(),
    )));

    builder.check(move || {
        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(std::sync::atomic::AtomicUsize::new(0)));

        // NOTE: done in this thread after spawning
        // thread::spawn(move || {
        //     x.store(true, Ordering::SeqCst);
        // });
        thread::spawn(move || {
            y.store(true, Ordering::SeqCst);
        });
        let t1 = thread::spawn(move || {
            while !x.load(Ordering::SeqCst) {
                loom::thread::yield_now();
            }
            if y.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        let t2 = thread::spawn(move || {
            while !y.load(Ordering::SeqCst) {
                loom::thread::yield_now();
            }
            if x.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        x.store(true, Ordering::SeqCst);

        t1.join().unwrap();
        t2.join().unwrap();
        // Read z but not while holding the lock, since the read goes into loom innards.
        let z = z.load(Ordering::SeqCst);
        assert_ne!(z, 0, "z == 0 is not possible with SeqCst");
        seen.lock().unwrap().insert(z);
    });
    let seen = seen.lock().unwrap();
    assert!(seen.contains(&1));
    assert!(seen.contains(&2));
    assert_eq!(seen.len(), 2);
}
