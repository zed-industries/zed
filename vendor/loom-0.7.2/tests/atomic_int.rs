#![deny(warnings, rust_2018_idioms)]

macro_rules! test_int {
    ($name:ident, $int:ty, $atomic:ty) => {
        mod $name {
            use loom::sync::atomic::*;
            use std::sync::atomic::Ordering::SeqCst;

            const NUM_A: u64 = 11641914933775430211;
            const NUM_B: u64 = 13209405719799650717;

            #[test]
            fn xor() {
                loom::model(|| {
                    let a: $int = NUM_A as $int;
                    let b: $int = NUM_B as $int;

                    let atomic = <$atomic>::new(a);
                    let prev = atomic.fetch_xor(b, SeqCst);

                    assert_eq!(a, prev, "prev did not match");
                    assert_eq!(a ^ b, atomic.load(SeqCst), "load failed");
                });
            }

            #[test]
            fn max() {
                loom::model(|| {
                    let a: $int = NUM_A as $int;
                    let b: $int = NUM_B as $int;

                    let atomic = <$atomic>::new(a);
                    let prev = atomic.fetch_max(b, SeqCst);

                    assert_eq!(a, prev, "prev did not match");
                    assert_eq!(a.max(b), atomic.load(SeqCst), "load failed");
                });
            }

            #[test]
            fn min() {
                loom::model(|| {
                    let a: $int = NUM_A as $int;
                    let b: $int = NUM_B as $int;

                    let atomic = <$atomic>::new(a);
                    let prev = atomic.fetch_min(b, SeqCst);

                    assert_eq!(a, prev, "prev did not match");
                    assert_eq!(a.min(b), atomic.load(SeqCst), "load failed");
                });
            }

            #[test]
            fn compare_exchange() {
                loom::model(|| {
                    let a: $int = NUM_A as $int;
                    let b: $int = NUM_B as $int;

                    let atomic = <$atomic>::new(a);
                    assert_eq!(Err(a), atomic.compare_exchange(b, a, SeqCst, SeqCst));
                    assert_eq!(Ok(a), atomic.compare_exchange(a, b, SeqCst, SeqCst));

                    assert_eq!(b, atomic.load(SeqCst));
                });
            }

            #[test]
            #[ignore]
            fn compare_exchange_weak() {
                loom::model(|| {
                    let a: $int = NUM_A as $int;
                    let b: $int = NUM_B as $int;

                    let atomic = <$atomic>::new(a);
                    assert_eq!(Err(a), atomic.compare_exchange_weak(b, a, SeqCst, SeqCst));
                    assert_eq!(Ok(a), atomic.compare_exchange_weak(a, b, SeqCst, SeqCst));

                    assert_eq!(b, atomic.load(SeqCst));
                });
            }

            #[test]
            fn fetch_update() {
                loom::model(|| {
                    let a: $int = NUM_A as $int;
                    let b: $int = NUM_B as $int;

                    let atomic = <$atomic>::new(a);
                    assert_eq!(Ok(a), atomic.fetch_update(SeqCst, SeqCst, |_| Some(b)));
                    assert_eq!(Err(b), atomic.fetch_update(SeqCst, SeqCst, |_| None));
                    assert_eq!(b, atomic.load(SeqCst));
                });
            }
        }
    };
}

test_int!(atomic_u8, u8, AtomicU8);
test_int!(atomic_u16, u16, AtomicU16);
test_int!(atomic_u32, u32, AtomicU32);
test_int!(atomic_usize, usize, AtomicUsize);

test_int!(atomic_i8, i8, AtomicI8);
test_int!(atomic_i16, i16, AtomicI16);
test_int!(atomic_i32, i32, AtomicI32);
test_int!(atomic_isize, isize, AtomicIsize);

#[cfg(target_pointer_width = "64")]
test_int!(atomic_u64, u64, AtomicU64);

#[cfg(target_pointer_width = "64")]
test_int!(atomic_i64, i64, AtomicI64);
