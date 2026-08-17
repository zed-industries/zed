use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::ops::Range;
use std::panic::{self, UnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};

const ITER: Range<i32> = 0..0x1_0000;
const PANIC: i32 = 0xC000;

fn check(&i: &i32) {
    if i == PANIC {
        panic!("boom")
    }
}

#[test]
#[should_panic(expected = "boom")]
fn iter_panic() {
    ITER.into_par_iter().for_each(|i| check(&i));
}

#[test]
#[cfg_attr(not(panic = "unwind"), ignore)]
fn iter_panic_fuse() {
    // We only use a single thread in order to make the behavior
    // of 'panic_fuse' deterministic
    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();

    pool.install(|| {
        fn count(iter: impl ParallelIterator + UnwindSafe) -> usize {
            let count = AtomicUsize::new(0);
            let result = panic::catch_unwind(|| {
                iter.for_each(|_| {
                    count.fetch_add(1, Ordering::Relaxed);
                });
            });
            assert!(result.is_err());
            count.into_inner()
        }

        // Without `panic_fuse()`, we'll reach every item except the panicking one.
        let expected = ITER.len() - 1;
        let iter = ITER.into_par_iter().with_max_len(1);
        assert_eq!(count(iter.clone().inspect(check)), expected);

        // With `panic_fuse()` anywhere in the chain, we'll reach fewer items.
        assert!(count(iter.clone().inspect(check).panic_fuse()) < expected);
        assert!(count(iter.clone().panic_fuse().inspect(check)) < expected);

        // Try in reverse to be sure we hit the producer case.
        assert!(count(iter.panic_fuse().inspect(check).rev()) < expected);
    });
}
