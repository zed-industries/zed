use std::ops::Deref;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::rc::Rc;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Barrier};
use std::thread;

use crate::collector::Collector;
use crate::{
    AtomicOwned, AtomicShared, Bag, Guard, Owned, Ptr, Queue, Shared, Stack, Tag, bag, suspend,
};

static_assertions::assert_eq_size!(Guard, usize);
static_assertions::assert_eq_size!(Option<Guard>, usize);
static_assertions::assert_impl_all!(AtomicShared<String>: Send, Sync, RefUnwindSafe, UnwindSafe);
static_assertions::assert_impl_all!(Guard: RefUnwindSafe, UnwindSafe);
static_assertions::assert_impl_all!(Ptr<String>: RefUnwindSafe, UnwindSafe);
static_assertions::assert_impl_all!(Shared<String>: Send, Sync, RefUnwindSafe, UnwindSafe);
static_assertions::assert_not_impl_all!(AtomicShared<*const u8>: Send, Sync, RefUnwindSafe, UnwindSafe);
static_assertions::assert_not_impl_all!(Collector: Send, Sync);
static_assertions::assert_not_impl_all!(Guard: Send, Sync);
static_assertions::assert_not_impl_all!(Ptr<String>: Send, Sync);
static_assertions::assert_not_impl_all!(Ptr<*const u8>: Send, Sync, RefUnwindSafe, UnwindSafe);
static_assertions::assert_not_impl_all!(Shared<*const u8>: Send, Sync, RefUnwindSafe, UnwindSafe);
static_assertions::assert_not_impl_any!(Bag<Rc<String>>: Send, Sync);
static_assertions::assert_impl_all!(Bag<String>: Send, Sync, UnwindSafe);
static_assertions::assert_impl_all!(bag::IterMut<'static, String>: Send, Sync, UnwindSafe);
static_assertions::assert_not_impl_any!(Bag<*const String>: Send, Sync);
static_assertions::assert_not_impl_any!(bag::IterMut<'static, *const String>: Send, Sync);
static_assertions::assert_not_impl_any!(Queue<Rc<String>>: Send, Sync);
static_assertions::assert_impl_all!(Queue<String>: Send, Sync, UnwindSafe);
static_assertions::assert_not_impl_any!(Queue<*const String>: Send, Sync);
static_assertions::assert_not_impl_any!(Stack<Rc<String>>: Send, Sync);
static_assertions::assert_impl_all!(Stack<String>: Send, Sync, UnwindSafe);
static_assertions::assert_not_impl_any!(Stack<*const String>: Send, Sync);

struct A(AtomicUsize, usize, &'static AtomicBool);
impl Drop for A {
    fn drop(&mut self) {
        self.2.swap(true, Relaxed);
    }
}

struct B(&'static AtomicUsize);
impl Drop for B {
    fn drop(&mut self) {
        self.0.fetch_add(1, Relaxed);
    }
}

struct C<T>(Owned<T>);
impl<T> Drop for C<T> {
    fn drop(&mut self) {
        let guard = Guard::new();
        let guarded_ptr = self.0.get_guarded_ptr(&guard);
        assert!(!guarded_ptr.is_null());
    }
}

struct R(&'static AtomicUsize, usize, usize);
impl R {
    fn new(cnt: &'static AtomicUsize, task_id: usize, seq: usize) -> R {
        cnt.fetch_add(1, Relaxed);
        R(cnt, task_id, seq)
    }
}
impl Drop for R {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Relaxed);
    }
}

#[test]
fn deferred() {
    static EXECUTED: AtomicBool = AtomicBool::new(false);

    let guard = Guard::new();
    guard.defer_execute(|| EXECUTED.store(true, Relaxed));
    drop(guard);

    while !EXECUTED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn shared() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let mut shared = Shared::new(A(AtomicUsize::new(10), 10, &DESTROYED));
    if let Some(mut_ref) = unsafe { shared.get_mut() } {
        mut_ref.1 += 1;
    }
    shared.0.fetch_add(1, Relaxed);
    assert_eq!(shared.deref().0.load(Relaxed), 11);
    assert_eq!(shared.deref().1, 11);

    let mut shared_clone = shared.clone();
    assert!(unsafe { shared_clone.get_mut().is_none() });
    shared_clone.0.fetch_add(1, Relaxed);
    assert_eq!(shared_clone.deref().0.load(Relaxed), 12);
    assert_eq!(shared_clone.deref().1, 11);

    let mut shared_clone_again = shared_clone.clone();
    assert!(unsafe { shared_clone_again.get_mut().is_none() });
    assert_eq!(shared_clone_again.deref().0.load(Relaxed), 12);
    assert_eq!(shared_clone_again.deref().1, 11);

    drop(shared);
    assert!(!DESTROYED.load(Relaxed));
    assert!(unsafe { shared_clone_again.get_mut().is_none() });

    drop(shared_clone);
    assert!(!DESTROYED.load(Relaxed));
    assert!(unsafe { shared_clone_again.get_mut().is_some() });

    drop(shared_clone_again);
    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn owned() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let mut owned = Owned::new(A(AtomicUsize::new(10), 10, &DESTROYED));
    unsafe {
        *owned.get_mut().0.get_mut() += 2;
        owned.get_mut().1 += 2;
    }
    assert_eq!(owned.deref().0.load(Relaxed), 12);
    assert_eq!(owned.deref().1, 12);

    let guard = Guard::new();
    let ptr = owned.get_guarded_ptr(&guard);
    assert!(ptr.get_shared().is_none());

    drop(owned);
    assert!(!DESTROYED.load(Relaxed));

    drop(guard);

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn sendable() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let shared = Shared::new(A(AtomicUsize::new(14), 14, &DESTROYED));
    let owned = Owned::new(A(AtomicUsize::new(15), 15, &DESTROYED));
    let shared_clone = shared.clone();
    let thread = std::thread::spawn(move || {
        assert_eq!(shared_clone.0.load(Relaxed), shared_clone.1);
        assert_eq!(owned.1, 15);
    });
    assert!(thread.join().is_ok());
    assert_eq!(shared.0.load(Relaxed), shared.1);
}

#[test]
fn accelerate() {
    let current_epoch = Guard::new().epoch();
    let target_epoch = current_epoch.next().next().next().next().next();

    let thread = std::thread::spawn(move || {
        loop {
            let guard = Guard::new();
            if guard.epoch() == target_epoch {
                break;
            }
            guard.accelerate();
            thread::yield_now();
        }
    });
    loop {
        let guard = Guard::new();
        if guard.epoch() == target_epoch {
            break;
        }
        guard.accelerate();
    }
    assert!(thread.join().is_ok());
}

#[test]
fn shared_send() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let shared = Shared::new(A(AtomicUsize::new(14), 14, &DESTROYED));
    let shared_clone = shared.clone();
    let thread = std::thread::spawn(move || {
        assert_eq!(shared_clone.0.load(Relaxed), 14);
        unsafe {
            assert!(!shared_clone.drop_in_place());
        }
    });
    assert!(thread.join().is_ok());
    assert_eq!(shared.0.load(Relaxed), 14);

    unsafe {
        assert!(shared.drop_in_place());
    }

    assert!(DESTROYED.load(Relaxed));
}

#[test]
fn shared_nested() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let nested_shared = Shared::new(Shared::new(A(AtomicUsize::new(10), 10, &DESTROYED)));
    assert!(!DESTROYED.load(Relaxed));
    drop(nested_shared);

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn shared_nested_thread() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let thread = std::thread::spawn(move || {
        let nested_shared = Shared::new(Shared::new(A(AtomicUsize::new(10), 10, &DESTROYED)));
        assert!(!DESTROYED.load(Relaxed));
        drop(nested_shared);
    });
    assert!(thread.join().is_ok());

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn owned_nested_unchecked() {
    let nested_owned = Owned::new(C(Owned::new(C(Owned::new(11)))));
    assert_eq!(*(nested_owned.0.0), 11);
}

#[test]
fn atomic_shared() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let atomic_shared = AtomicShared::new(A(AtomicUsize::new(10), 10, &DESTROYED));
    assert!(!DESTROYED.load(Relaxed));

    let guard = Guard::new();
    let atomic_shared_clone = atomic_shared.clone(Relaxed, &guard);
    assert_eq!(
        atomic_shared_clone
            .load(Relaxed, &guard)
            .as_ref()
            .unwrap()
            .1,
        10
    );

    drop(atomic_shared);
    assert!(!DESTROYED.load(Relaxed));

    atomic_shared_clone.update_tag_if(Tag::Second, |_| true, Relaxed, Relaxed);

    drop(atomic_shared_clone);
    drop(guard);

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn atomic_owned() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let atomic_owned = AtomicOwned::new(A(AtomicUsize::new(10), 10, &DESTROYED));
    assert!(!DESTROYED.load(Relaxed));

    let guard = Guard::new();
    let ptr = atomic_owned.load(Relaxed, &guard);
    assert_eq!(ptr.as_ref().map(|a| a.1), Some(10));

    atomic_owned.update_tag_if(Tag::Second, |_| true, Relaxed, Relaxed);

    drop(atomic_owned);
    assert_eq!(ptr.as_ref().map(|a| a.1), Some(10));

    drop(guard);

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn atomic_shared_send() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let atomic_shared = AtomicShared::new(A(AtomicUsize::new(17), 17, &DESTROYED));
    assert!(!DESTROYED.load(Relaxed));

    let thread = std::thread::spawn(move || {
        let guard = Guard::new();
        let ptr = atomic_shared.load(Relaxed, &guard);
        assert_eq!(ptr.as_ref().unwrap().0.load(Relaxed), 17);
    });
    assert!(thread.join().is_ok());

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn atomic_shared_creation() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let atomic_shared = AtomicShared::new(A(AtomicUsize::new(11), 11, &DESTROYED));
    assert!(!DESTROYED.load(Relaxed));

    let guard = Guard::new();

    let shared = atomic_shared.get_shared(Relaxed, &guard);

    drop(atomic_shared);
    assert!(!DESTROYED.load(Relaxed));

    if let Some(shared) = shared {
        assert_eq!(shared.1, 11);
        assert!(!DESTROYED.load(Relaxed));
    }
    drop(guard);

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn atomic_shared_conversion() {
    static DESTROYED: AtomicBool = AtomicBool::new(false);

    let atomic_shared = AtomicShared::new(A(AtomicUsize::new(11), 11, &DESTROYED));
    assert!(!DESTROYED.load(Relaxed));

    let guard = Guard::new();

    let shared = atomic_shared.into_shared(Relaxed);
    assert!(!DESTROYED.load(Relaxed));

    if let Some(shared) = shared {
        assert_eq!(shared.1, 11);
        assert!(!DESTROYED.load(Relaxed));
    }
    drop(guard);

    while !DESTROYED.load(Relaxed) {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn reclaim_collector() {
    static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

    let num_threads = 16;
    let num_iter = 32;

    for _ in 0..num_iter {
        assert!(suspend());

        thread::scope(|s| {
            for _ in 0..num_threads {
                assert!(
                    s.spawn(|| {
                        let owned = Owned::new(B(&DEALLOCATED));
                        assert_ne!(owned.0.load(Relaxed), usize::MAX);
                    })
                    .join()
                    .is_ok()
                );
            }
        });

        while DEALLOCATED.load(Relaxed) != num_threads {
            Guard::new().accelerate();
            thread::yield_now();
        }
        DEALLOCATED.store(0, Relaxed);
    }
}

#[test]
fn reclaim_collector_nested() {
    static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

    let num_threads = if cfg!(miri) { 4 } else { 16 };
    let num_iter = if cfg!(miri) { 4 } else { 16 };

    for _ in 0..num_iter {
        assert!(suspend());

        thread::scope(|s| {
            let threads: Vec<_> = (0..num_threads)
                .map(|_| {
                    s.spawn(|| {
                        let guard = Guard::new();
                        let owned_shared = Owned::new(Shared::new(B(&DEALLOCATED)));
                        assert_ne!(
                            owned_shared
                                .get_guarded_ptr(&guard)
                                .as_ref()
                                .unwrap()
                                .0
                                .load(Relaxed),
                            usize::MAX
                        );
                        let owned = Owned::new(B(&DEALLOCATED));
                        assert_ne!(
                            owned
                                .get_guarded_ptr(&guard)
                                .as_ref()
                                .unwrap()
                                .0
                                .load(Relaxed),
                            usize::MAX
                        );
                    })
                })
                .collect();
            for t in threads {
                assert!(t.join().is_ok());
            }
        });

        while DEALLOCATED.load(Relaxed) != num_threads * 2 {
            Guard::new().accelerate();
            thread::yield_now();
        }
        DEALLOCATED.store(0, Relaxed);
    }
}

#[test]
fn atomic_shared_parallel() {
    let atomic_shared: Shared<AtomicShared<String>> =
        Shared::new(AtomicShared::new(String::from("How are you?")));
    let mut threads = Vec::new();
    let concurrency = if cfg!(miri) { 4 } else { 16 };
    for _ in 0..concurrency {
        let atomic_shared = atomic_shared.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..concurrency {
                let guard = Guard::new();
                let mut ptr = (*atomic_shared).load(Acquire, &guard);
                assert!(ptr.tag() == Tag::None || ptr.tag() == Tag::Second);
                if let Some(str_ref) = ptr.as_ref() {
                    assert!(str_ref == "How are you?" || str_ref == "How can I help you?");
                }
                let converted: Result<Shared<String>, _> = Shared::try_from(ptr);
                if let Ok(shared) = converted {
                    assert!(*shared == "How are you?" || *shared == "How can I help you?");
                }
                while let Err((passed, current)) = atomic_shared.compare_exchange(
                    ptr,
                    (
                        Some(Shared::new(String::from("How can I help you?"))),
                        Tag::Second,
                    ),
                    AcqRel,
                    Acquire,
                    &guard,
                ) {
                    if let Some(shared) = passed {
                        assert!(*shared == "How can I help you?");
                    }
                    ptr = current;
                    if let Some(str_ref) = ptr.as_ref() {
                        assert!(str_ref == "How are you?" || str_ref == "How can I help you?");
                    }
                    assert!(ptr.tag() == Tag::None || ptr.tag() == Tag::Second);
                }
                assert!(!suspend());
                drop(guard);

                assert!(suspend());

                atomic_shared.update_tag_if(Tag::None, |_| true, Relaxed, Relaxed);

                let guard = Guard::new();
                ptr = (*atomic_shared).load(Acquire, &guard);
                assert!(ptr.tag() == Tag::None || ptr.tag() == Tag::Second);
                if let Some(str_ref) = ptr.as_ref() {
                    assert!(str_ref == "How are you?" || str_ref == "How can I help you?");
                }
                drop(guard);

                let (old, _) = atomic_shared.swap(
                    (Some(Shared::new(String::from("How are you?"))), Tag::Second),
                    AcqRel,
                );
                if let Some(shared) = old {
                    assert!(*shared == "How are you?" || *shared == "How can I help you?");
                }
            }
        }));
    }
    for t in threads {
        assert!(t.join().is_ok());
    }
}

#[test]
fn atomic_shared_clone() {
    let atomic_shared: Shared<AtomicShared<String>> =
        Shared::new(AtomicShared::new(String::from("How are you?")));
    let mut threads = Vec::new();
    for t in 0..4 {
        let atomic_shared = atomic_shared.clone();
        threads.push(thread::spawn(move || {
            let num_iter = if cfg!(miri) { 16 } else { 256 };
            for i in 0..num_iter {
                if t == 0 {
                    let tag = if i % 3 == 0 {
                        Tag::First
                    } else if i % 2 == 0 {
                        Tag::Second
                    } else {
                        Tag::None
                    };
                    let (old, _) = atomic_shared.swap(
                        (Some(Shared::new(String::from("How are you?"))), tag),
                        Release,
                    );
                    assert!(old.is_some());
                    if let Some(shared) = old {
                        assert!(*shared == "How are you?");
                    }
                } else {
                    let (shared_clone, _) = (*atomic_shared)
                        .clone(Acquire, &Guard::new())
                        .swap((None, Tag::First), Release);
                    assert!(shared_clone.is_some());
                    if let Some(shared) = shared_clone {
                        assert!(*shared == "How are you?");
                    }
                    let shared_clone = atomic_shared.get_shared(Acquire, &Guard::new());
                    assert!(shared_clone.is_some());
                    if let Some(shared) = shared_clone {
                        assert!(*shared == "How are you?");
                    }
                }
            }
        }));
    }
    for t in threads {
        assert!(t.join().is_ok());
    }
}

#[test]
fn bag_reclaim() {
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    for workload_size in [2, 18, 32, 40, 120] {
        let mut bag: Bag<R> = Bag::default();
        for _ in 0..workload_size {
            bag.push(R::new(&INST_CNT, 0, 0));
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size);
        assert_eq!(bag.iter_mut().count(), workload_size);
        bag.iter_mut().for_each(|e| {
            *e = R::new(&INST_CNT, 0, 0);
        });

        for _ in 0..workload_size / 2 {
            bag.pop();
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size / 2);
        drop(bag);
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }
}

#[test]
fn bag_from_iter() {
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);

    let workload_size = 16;
    let bag = (0..workload_size)
        .map(|_| R::new(&INST_CNT, 0, 0))
        .collect::<Bag<R>>();
    assert_eq!(bag.len(), workload_size);
    drop(bag);
    assert_eq!(INST_CNT.load(Relaxed), 0);
}

#[test]
fn bag_into_iter() {
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    for workload_size in [2, 18, 32, 40, 120] {
        let mut bag: Bag<R> = Bag::default();
        for _ in 0..workload_size {
            bag.push(R::new(&INST_CNT, 0, 0));
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size);
        assert_eq!(bag.len(), workload_size);
        assert_eq!(bag.iter_mut().count(), workload_size);

        for v in &mut bag {
            assert_eq!(v.0.load(Relaxed), INST_CNT.load(Relaxed));
        }
        assert_eq!(INST_CNT.load(Relaxed), workload_size);

        for v in bag {
            assert_eq!(v.0.load(Relaxed), INST_CNT.load(Relaxed));
        }
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }
}

#[test]
fn bag_mpmc() {
    const NUM_THREADS: usize = if cfg!(miri) { 2 } else { 6 };
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 8 } else { 64 };
    for _ in 0..4 {
        let bag_default: Arc<Bag<R>> = Arc::new(Bag::default());
        let bag_half: Arc<Bag<R, 15>> = Arc::new(Bag::new());
        for _ in 0..workload_size {
            let mut threads = Vec::with_capacity(NUM_THREADS);
            let barrier = Arc::new(Barrier::new(NUM_THREADS));
            for _ in 0..NUM_THREADS {
                let barrier = barrier.clone();
                let bag32 = bag_default.clone();
                let bag_half = bag_half.clone();
                threads.push(thread::spawn(move || {
                    barrier.wait();
                    for _ in 0..4 {
                        for _ in 0..workload_size {
                            bag32.push(R::new(&INST_CNT, 0, 0));
                            bag_half.push(R::new(&INST_CNT, 0, 0));
                        }
                        for _ in 0..workload_size {
                            while bag32.pop().is_none() {
                                Guard::new().accelerate();
                                thread::yield_now();
                            }
                            while bag_half.pop().is_none() {
                                Guard::new().accelerate();
                                thread::yield_now();
                            }
                        }
                    }
                }));
            }

            for thread in threads {
                assert!(thread.join().is_ok());
            }
            assert!(bag_default.pop().is_none());
            assert!(bag_default.is_empty());
            assert!(bag_half.pop().is_none());
            assert!(bag_half.is_empty());
        }
        assert_eq!(INST_CNT.load(Relaxed), 0);
    }
}

#[test]
fn bag_mpsc() {
    const NUM_THREADS: usize = if cfg!(miri) { 2 } else { 6 };
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 16 } else { 256 };
    let bag32: Arc<Bag<R>> = Arc::new(Bag::default());
    let bag7: Arc<Bag<R, 7>> = Arc::new(Bag::new());
    for _ in 0..16 {
        let mut threads = Vec::with_capacity(NUM_THREADS);
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        for thread_id in 0..NUM_THREADS {
            let barrier = barrier.clone();
            let bag32 = bag32.clone();
            let bag7 = bag7.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                let mut cnt = 0;
                while thread_id == 0 && cnt < workload_size * (NUM_THREADS - 1) * 2 {
                    cnt += bag32.pop_all(0, |a, _| a + 1);
                    cnt += bag7.pop_all(0, |a, _| a + 1);
                    thread::yield_now();
                }
                if thread_id != 0 {
                    for _ in 0..workload_size {
                        bag32.push(R::new(&INST_CNT, 0, 0));
                        bag7.push(R::new(&INST_CNT, 0, 0));
                    }
                    for _ in 0..workload_size / 16 {
                        if bag32.pop().is_some() {
                            bag32.push(R::new(&INST_CNT, 0, 0));
                        }
                        if bag7.pop().is_some() {
                            bag7.push(R::new(&INST_CNT, 0, 0));
                        }
                    }
                }
            }));
        }

        for thread in threads {
            assert!(thread.join().is_ok());
        }
        assert!(bag32.pop().is_none());
        assert!(bag32.is_empty());
        assert!(bag7.pop().is_none());
        assert!(bag7.is_empty());
    }
    assert_eq!(INST_CNT.load(Relaxed), 0);
}

#[test]
fn queue_clone() {
    let queue = Queue::default();
    queue.push(37);
    queue.push(3);
    queue.push(1);

    let queue_clone = queue.clone();

    assert_eq!(queue.pop().map(|e| **e), Some(37));
    assert_eq!(queue.pop().map(|e| **e), Some(3));
    assert_eq!(queue.pop().map(|e| **e), Some(1));
    assert!(queue.pop().is_none());

    assert_eq!(queue_clone.pop().map(|e| **e), Some(37));
    assert_eq!(queue_clone.pop().map(|e| **e), Some(3));
    assert_eq!(queue_clone.pop().map(|e| **e), Some(1));
    assert!(queue_clone.pop().is_none());
}

#[test]
fn queue_from_iter() {
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);

    let workload_size = 16;
    let queue = (0..workload_size)
        .map(|i| R::new(&INST_CNT, i, i))
        .collect::<Queue<R>>();
    assert_eq!(queue.len(), workload_size);
    drop(queue);

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn queue_pop_all() {
    const NUM_ENTRIES: usize = 256;
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);

    let queue = Queue::default();

    for i in 0..NUM_ENTRIES {
        queue.push(R::new(&INST_CNT, i, i));
    }

    let mut expected = 0;
    while let Some(e) = queue.pop() {
        assert_eq!(e.1, expected);
        expected += 1;
    }
    assert_eq!(expected, NUM_ENTRIES);
    assert!(queue.is_empty());

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn queue_iter_push_pop() {
    const NUM_THREADS: usize = if cfg!(miri) { 2 } else { 4 };
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 16 } else { 256 };
    let queue: Arc<Queue<R>> = Arc::new(Queue::default());
    for _ in 0..4 {
        let mut threads = Vec::with_capacity(NUM_THREADS);
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        for task_id in 0..NUM_THREADS {
            let barrier = barrier.clone();
            let queue = queue.clone();
            threads.push(thread::spawn(move || {
                if task_id == 0 {
                    for seq in 0..workload_size {
                        if seq == workload_size / 2 {
                            barrier.wait();
                        }
                        assert_eq!(queue.push(R::new(&INST_CNT, task_id, seq)).2, seq);
                    }
                    let mut last = 0;
                    while let Some(popped) = queue.pop() {
                        let current = popped.1;
                        assert!(last == 0 || last + 1 == current);
                        last = current;
                    }
                } else {
                    let mut last = 0;

                    barrier.wait();
                    let guard = Guard::new();
                    let iter = queue.iter(&guard);
                    for current in iter {
                        let current = current.1;
                        assert!(current == 0 || last + 1 == current);
                        last = current;
                    }
                }
            }));
        }

        for thread in threads {
            assert!(thread.join().is_ok());
        }
    }
    assert!(queue.is_empty());

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn queue_mpmc() {
    const NUM_THREADS: usize = if cfg!(miri) { 3 } else { 6 };
    const NUM_PRODUCERS: usize = NUM_THREADS / 2;
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 16 } else { 256 };
    let queue: Arc<Queue<R>> = Arc::new(Queue::default());
    for _ in 0..4 {
        let num_popped: Arc<AtomicUsize> = Arc::new(AtomicUsize::default());
        let mut threads = Vec::with_capacity(NUM_THREADS);
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        for thread_id in 0..NUM_THREADS {
            let barrier = barrier.clone();
            let queue = queue.clone();
            let num_popped = num_popped.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                if thread_id < NUM_PRODUCERS {
                    for seq in 1..=workload_size {
                        assert_eq!(queue.push(R::new(&INST_CNT, thread_id, seq)).2, seq);
                    }
                } else {
                    let mut popped_acc: [usize; NUM_PRODUCERS] = Default::default();
                    loop {
                        let mut cnt = 0;
                        while let Some(popped) = queue.pop() {
                            cnt += 1;
                            assert!(popped_acc[popped.1] < popped.2);
                            popped_acc[popped.1] = popped.2;
                        }
                        if num_popped.fetch_add(cnt, Relaxed) + cnt == workload_size * NUM_PRODUCERS
                        {
                            break;
                        }
                        thread::yield_now();
                    }
                }
            }));
        }

        for thread in threads {
            assert!(thread.join().is_ok());
        }
    }
    assert!(queue.is_empty());

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn stack_clone() {
    let stack = Stack::default();
    stack.push(37);
    stack.push(3);
    stack.push(1);

    let stack_clone = stack.clone();

    assert_eq!(stack.pop().map(|e| **e), Some(1));
    assert_eq!(stack.pop().map(|e| **e), Some(3));
    assert_eq!(stack.pop().map(|e| **e), Some(37));
    assert!(stack.pop().is_none());

    assert_eq!(stack_clone.pop().map(|e| **e), Some(1));
    assert_eq!(stack_clone.pop().map(|e| **e), Some(3));
    assert_eq!(stack_clone.pop().map(|e| **e), Some(37));
    assert!(stack_clone.pop().is_none());
}

#[test]
fn stack_from_iter() {
    let workload_size = 16;
    let stack = (0..workload_size).collect::<Stack<usize>>();
    assert_eq!(stack.len(), workload_size);
    for i in (0..workload_size).rev() {
        assert_eq!(stack.pop().map(|e| **e), Some(i));
    }
}

#[test]
fn stack_iterator() {
    const NUM_THREADS: usize = if cfg!(miri) { 2 } else { 12 };
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 16 } else { 256 };
    let stack: Arc<Stack<R>> = Arc::new(Stack::default());
    for _ in 0..4 {
        let mut threads = Vec::with_capacity(NUM_THREADS);
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        for task_id in 0..NUM_THREADS {
            let barrier = barrier.clone();
            let stack = stack.clone();
            threads.push(thread::spawn(move || {
                if task_id == 0 {
                    for seq in 0..workload_size {
                        if seq == workload_size / 2 {
                            barrier.wait();
                        }
                        assert_eq!(stack.push(R::new(&INST_CNT, task_id, seq)).2, seq);
                    }
                    let mut last = workload_size;
                    while let Some(popped) = stack.pop() {
                        let current = popped.2;
                        assert_eq!(current + 1, last);
                        last = current;
                    }
                } else {
                    let mut last = workload_size;

                    barrier.wait();
                    let guard = Guard::new();
                    let iter = stack.iter(&guard);
                    for current in iter {
                        let current = current.2;
                        assert!(last == workload_size || last > current);
                        last = current;
                    }
                }
            }));
        }

        for t in threads {
            assert!(t.join().is_ok());
        }
    }
    assert!(stack.is_empty());

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn stack_mpmc() {
    const NUM_THREADS: usize = if cfg!(miri) { 2 } else { 12 };
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 16 } else { 256 };
    let stack: Arc<Stack<R>> = Arc::new(Stack::default());
    for _ in 0..4 {
        let mut threads = Vec::with_capacity(NUM_THREADS);
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        for thread_id in 0..NUM_THREADS {
            let barrier = barrier.clone();
            let stack = stack.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                for seq in 0..workload_size {
                    assert_eq!(stack.push(R::new(&INST_CNT, thread_id, seq)).2, seq);
                }
                let mut last_popped = usize::MAX;
                let mut cnt = 0;
                while cnt < workload_size {
                    while let Ok(Some(popped)) = stack.pop_if(|e| e.1 == thread_id) {
                        assert_eq!(popped.1, thread_id);
                        assert!(last_popped > popped.2);
                        last_popped = popped.2;
                        cnt += 1;
                    }
                    thread::yield_now();
                }
            }));
        }

        for t in threads {
            assert!(t.join().is_ok());
        }
    }
    assert!(stack.is_empty());

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}

#[test]
fn stack_mpsc() {
    const NUM_THREADS: usize = if cfg!(miri) { 2 } else { 12 };
    static INST_CNT: AtomicUsize = AtomicUsize::new(0);
    let workload_size = if cfg!(miri) { 16 } else { 256 };
    let stack: Arc<Stack<R>> = Arc::new(Stack::default());
    for _ in 0..4 {
        let mut threads = Vec::with_capacity(NUM_THREADS);
        let barrier = Arc::new(Barrier::new(NUM_THREADS));
        for thread_id in 0..NUM_THREADS {
            let barrier = barrier.clone();
            let stack = stack.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                let mut cnt = 0;
                while thread_id == 0 && cnt < workload_size * (NUM_THREADS - 1) {
                    // Consumer.
                    let popped = stack.pop_all();
                    while let Some(e) = popped.pop() {
                        assert_ne!(e.1, 0);
                        cnt += 1;
                    }
                    thread::yield_now();
                }
                if thread_id != 0 {
                    for seq in 0..workload_size {
                        assert_eq!(stack.push(R::new(&INST_CNT, thread_id, seq)).2, seq);
                    }
                    for seq in 0..workload_size / 16 {
                        if stack.pop().is_some() {
                            assert_eq!(stack.push(R::new(&INST_CNT, thread_id, seq)).2, seq);
                        }
                    }
                }
            }));
        }

        for t in threads {
            assert!(t.join().is_ok());
        }
    }
    assert!(stack.is_empty());

    while INST_CNT.load(Relaxed) != 0 {
        Guard::new().accelerate();
        thread::yield_now();
    }
}
