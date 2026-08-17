use loom::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    thread,
};

#[test]
fn rwlock_two_writers() {
    loom::model(|| {
        let lock = Arc::new(RwLock::new(1));
        let c_lock = lock.clone();
        let c_lock2 = lock;

        let atomic = Arc::new(AtomicUsize::new(0));
        let c_atomic = atomic.clone();
        let c_atomic2 = atomic;

        thread::spawn(move || {
            let mut w = c_lock.write().unwrap();
            *w += 1;
            c_atomic.fetch_add(1, Ordering::Relaxed);
        });

        thread::spawn(move || {
            let mut w = c_lock2.write().unwrap();
            *w += 1;
            c_atomic2.fetch_add(1, Ordering::Relaxed);
        });
    });
}
