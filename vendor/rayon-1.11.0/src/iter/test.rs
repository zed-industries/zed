use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;
use crate::prelude::*;
use rayon_core::*;

use rand::distr::StandardUniform;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::collections::{BinaryHeap, VecDeque};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::sync::mpsc;

fn is_indexed<T: IndexedParallelIterator>(_: T) {}

fn seeded_rng() -> XorShiftRng {
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}

#[test]
fn execute() {
    let a: Vec<i32> = (0..1024).collect();
    let mut b = vec![];
    a.par_iter().map(|&i| i + 1).collect_into_vec(&mut b);
    let c: Vec<i32> = (0..1024).map(|i| i + 1).collect();
    assert_eq!(b, c);
}

#[test]
fn execute_cloned() {
    let a: Vec<i32> = (0..1024).collect();
    let mut b: Vec<i32> = vec![];
    a.par_iter().cloned().collect_into_vec(&mut b);
    let c: Vec<i32> = (0..1024).collect();
    assert_eq!(b, c);
}

#[test]
fn execute_range() {
    let a = 0i32..1024;
    let mut b = vec![];
    a.into_par_iter().map(|i| i + 1).collect_into_vec(&mut b);
    let c: Vec<i32> = (0..1024).map(|i| i + 1).collect();
    assert_eq!(b, c);
}

#[test]
fn execute_unindexed_range() {
    let a = 0i64..1024;
    let b: LinkedList<i64> = a.into_par_iter().map(|i| i + 1).collect();
    let c: LinkedList<i64> = (0..1024).map(|i| i + 1).collect();
    assert_eq!(b, c);
}

#[test]
fn execute_pseudo_indexed_range() {
    let range = i128::MAX - 1024..i128::MAX;

    // Given `Some` length, collecting `Vec` will try to act indexed.
    let a = range.clone().into_par_iter();
    assert_eq!(a.opt_len(), Some(1024));

    let b: Vec<i128> = a.map(|i| i + 1).collect();
    let c: Vec<i128> = range.map(|i| i + 1).collect();
    assert_eq!(b, c);
}

#[test]
fn check_map_indexed() {
    let a = [1, 2, 3];
    is_indexed(a.par_iter().map(|x| x));
}

#[test]
fn map_sum() {
    let a: Vec<i32> = (0..1024).collect();
    let r1: i32 = a.par_iter().map(|&i| i + 1).sum();
    let r2 = a.iter().map(|&i| i + 1).sum();
    assert_eq!(r1, r2);
}

#[test]
fn map_reduce() {
    let a: Vec<i32> = (0..1024).collect();
    let r1 = a.par_iter().map(|&i| i + 1).reduce(|| 0, |i, j| i + j);
    let r2 = a.iter().map(|&i| i + 1).sum();
    assert_eq!(r1, r2);
}

#[test]
fn map_reduce_with() {
    let a: Vec<i32> = (0..1024).collect();
    let r1 = a.par_iter().map(|&i| i + 1).reduce_with(|i, j| i + j);
    let r2 = a.iter().map(|&i| i + 1).sum();
    assert_eq!(r1, Some(r2));
}

#[test]
fn fold_map_reduce() {
    // Kind of a weird test, but it demonstrates various
    // transformations that are taking place. Relies on
    // `with_max_len(1).fold()` being equivalent to `map()`.
    //
    // Take each number from 0 to 32 and fold them by appending to a
    // vector.  Because of `with_max_len(1)`, this will produce 32 vectors,
    // each with one item.  We then collect all of these into an
    // individual vector by mapping each into their own vector (so we
    // have Vec<Vec<i32>>) and then reducing those into a single
    // vector.
    let r1 = (0_i32..32)
        .into_par_iter()
        .with_max_len(1)
        .fold(Vec::new, |mut v, e| {
            v.push(e);
            v
        })
        .map(|v| vec![v])
        .reduce_with(|mut v_a, v_b| {
            v_a.extend(v_b);
            v_a
        });
    assert_eq!(
        r1,
        Some(vec![
            vec![0],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![7],
            vec![8],
            vec![9],
            vec![10],
            vec![11],
            vec![12],
            vec![13],
            vec![14],
            vec![15],
            vec![16],
            vec![17],
            vec![18],
            vec![19],
            vec![20],
            vec![21],
            vec![22],
            vec![23],
            vec![24],
            vec![25],
            vec![26],
            vec![27],
            vec![28],
            vec![29],
            vec![30],
            vec![31]
        ])
    );
}

#[test]
fn fold_is_full() {
    let counter = AtomicUsize::new(0);
    let a = (0_i32..2048)
        .into_par_iter()
        .inspect(|_| {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .fold(|| 0, |a, b| a + b)
        .find_any(|_| true);
    assert!(a.is_some());
    assert!(counter.load(Ordering::SeqCst) < 2048); // should not have visited every single one
}

#[test]
fn check_step_by() {
    let a: Vec<i32> = (0..1024).step_by(2).collect();
    let b: Vec<i32> = (0..1024).into_par_iter().step_by(2).collect();

    assert_eq!(a, b);
}

#[test]
fn check_step_by_unaligned() {
    let a: Vec<i32> = (0..1029).step_by(10).collect();
    let b: Vec<i32> = (0..1029).into_par_iter().step_by(10).collect();

    assert_eq!(a, b)
}

#[test]
fn check_step_by_rev() {
    let a: Vec<i32> = (0..1024).step_by(2).rev().collect();
    let b: Vec<i32> = (0..1024).into_par_iter().step_by(2).rev().collect();

    assert_eq!(a, b);
}

#[test]
fn check_enumerate() {
    let a: Vec<usize> = (0..1024).rev().collect();

    let mut b = vec![];
    a.par_iter()
        .enumerate()
        .map(|(i, &x)| i + x)
        .collect_into_vec(&mut b);
    assert!(b.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_enumerate_rev() {
    let a: Vec<usize> = (0..1024).rev().collect();

    let mut b = vec![];
    a.par_iter()
        .enumerate()
        .rev()
        .map(|(i, &x)| i + x)
        .collect_into_vec(&mut b);
    assert!(b.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_indices_after_enumerate_split() {
    let a: Vec<i32> = (0..1024).collect();
    a.par_iter().enumerate().with_producer(WithProducer);

    struct WithProducer;
    impl<'a> ProducerCallback<(usize, &'a i32)> for WithProducer {
        type Output = ();
        fn callback<P>(self, producer: P)
        where
            P: Producer<Item = (usize, &'a i32)>,
        {
            let (a, b) = producer.split_at(512);
            for ((index, value), trusted_index) in a.into_iter().zip(0..) {
                assert_eq!(index, trusted_index);
                assert_eq!(index, *value as usize);
            }
            for ((index, value), trusted_index) in b.into_iter().zip(512..) {
                assert_eq!(index, trusted_index);
                assert_eq!(index, *value as usize);
            }
        }
    }
}

#[test]
fn check_increment() {
    let mut a: Vec<usize> = (0..1024).rev().collect();

    a.par_iter_mut().enumerate().for_each(|(i, v)| *v += i);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_skip() {
    let a: Vec<usize> = (0..1024).collect();

    let mut v1 = Vec::new();
    a.par_iter().skip(16).collect_into_vec(&mut v1);
    let v2 = a.iter().skip(16).collect::<Vec<_>>();
    assert_eq!(v1, v2);

    let mut v1 = Vec::new();
    a.par_iter().skip(2048).collect_into_vec(&mut v1);
    let v2 = a.iter().skip(2048).collect::<Vec<_>>();
    assert_eq!(v1, v2);

    let mut v1 = Vec::new();
    a.par_iter().skip(0).collect_into_vec(&mut v1);
    #[allow(clippy::iter_skip_zero)]
    let v2 = a.iter().skip(0).collect::<Vec<_>>();
    assert_eq!(v1, v2);

    // Check that the skipped elements side effects are executed
    use std::sync::atomic::{AtomicUsize, Ordering};
    let num = AtomicUsize::new(0);
    a.par_iter()
        .map(|&n| num.fetch_add(n, Ordering::Relaxed))
        .skip(512)
        .count();
    assert_eq!(num.load(Ordering::Relaxed), a.iter().sum::<usize>());
}

#[test]
fn check_take() {
    let a: Vec<usize> = (0..1024).collect();

    let mut v1 = Vec::new();
    a.par_iter().take(16).collect_into_vec(&mut v1);
    let v2 = a.iter().take(16).collect::<Vec<_>>();
    assert_eq!(v1, v2);

    let mut v1 = Vec::new();
    a.par_iter().take(2048).collect_into_vec(&mut v1);
    let v2 = a.iter().take(2048).collect::<Vec<_>>();
    assert_eq!(v1, v2);

    let mut v1 = Vec::new();
    a.par_iter().take(0).collect_into_vec(&mut v1);
    let v2 = a.iter().take(0).collect::<Vec<_>>();
    assert_eq!(v1, v2);
}

#[test]
fn check_inspect() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let a = AtomicUsize::new(0);
    let b: usize = (0_usize..1024)
        .into_par_iter()
        .inspect(|&i| {
            a.fetch_add(i, Ordering::Relaxed);
        })
        .sum();

    assert_eq!(a.load(Ordering::Relaxed), b);
}

#[test]
fn check_move() {
    let a = vec![vec![1, 2, 3]];
    let ptr = a[0].as_ptr();

    let mut b = vec![];
    a.into_par_iter().collect_into_vec(&mut b);

    // a simple move means the inner vec will be completely unchanged
    assert_eq!(ptr, b[0].as_ptr());
}

#[test]
fn check_drops() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let c = AtomicUsize::new(0);
    let a = vec![DropCounter(&c); 10];

    let mut b = vec![];
    a.clone().into_par_iter().collect_into_vec(&mut b);
    assert_eq!(c.load(Ordering::Relaxed), 0);

    b.into_par_iter();
    assert_eq!(c.load(Ordering::Relaxed), 10);

    a.into_par_iter().with_producer(Partial);
    assert_eq!(c.load(Ordering::Relaxed), 20);

    #[derive(Clone)]
    struct DropCounter<'a>(&'a AtomicUsize);
    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    struct Partial;
    impl<'a> ProducerCallback<DropCounter<'a>> for Partial {
        type Output = ();
        fn callback<P>(self, producer: P)
        where
            P: Producer<Item = DropCounter<'a>>,
        {
            let (a, _) = producer.split_at(5);
            a.into_iter().next();
        }
    }
}

#[test]
fn check_slice_indexed() {
    let a = vec![1, 2, 3];
    is_indexed(a.par_iter());
}

#[test]
fn check_slice_mut_indexed() {
    let mut a = vec![1, 2, 3];
    is_indexed(a.par_iter_mut());
}

#[test]
fn check_vec_indexed() {
    let a = vec![1, 2, 3];
    is_indexed(a.into_par_iter());
}

#[test]
fn check_range_indexed() {
    is_indexed((1..5).into_par_iter());
}

#[test]
fn check_cmp_direct() {
    let a = (0..1024).into_par_iter();
    let b = (0..1024).into_par_iter();

    let result = a.cmp(b);

    assert!(result == ::std::cmp::Ordering::Equal);
}

#[test]
fn check_cmp_to_seq() {
    assert_eq!(
        (0..1024).into_par_iter().cmp(0..1024),
        (0..1024).cmp(0..1024)
    );
}

#[test]
fn check_cmp_rng_to_seq() {
    let mut rng = seeded_rng();
    let rng = &mut rng;
    let a: Vec<i32> = rng.sample_iter(&StandardUniform).take(1024).collect();
    let b: Vec<i32> = rng.sample_iter(&StandardUniform).take(1024).collect();
    for i in 0..a.len() {
        let par_result = a[i..].par_iter().cmp(b[i..].par_iter());
        let seq_result = a[i..].iter().cmp(b[i..].iter());

        assert_eq!(par_result, seq_result);
    }
}

#[test]
fn check_cmp_lt_direct() {
    let a = (0..1024).into_par_iter();
    let b = (1..1024).into_par_iter();

    let result = a.cmp(b);

    assert!(result == ::std::cmp::Ordering::Less);
}

#[test]
fn check_cmp_lt_to_seq() {
    assert_eq!(
        (0..1024).into_par_iter().cmp(1..1024),
        (0..1024).cmp(1..1024)
    )
}

#[test]
fn check_cmp_gt_direct() {
    let a = (1..1024).into_par_iter();
    let b = (0..1024).into_par_iter();

    let result = a.cmp(b);

    assert!(result == ::std::cmp::Ordering::Greater);
}

#[test]
fn check_cmp_gt_to_seq() {
    assert_eq!(
        (1..1024).into_par_iter().cmp(0..1024),
        (1..1024).cmp(0..1024)
    )
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn check_cmp_short_circuit() {
    // We only use a single thread in order to make the short-circuit behavior deterministic.
    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();

    let a = vec![0; 1024];
    let mut b = a.clone();
    b[42] = 1;

    pool.install(|| {
        let expected = ::std::cmp::Ordering::Less;
        assert_eq!(a.par_iter().cmp(&b), expected);

        for len in 1..10 {
            let counter = AtomicUsize::new(0);
            let result = a
                .par_iter()
                .with_max_len(len)
                .inspect(|_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                })
                .cmp(&b);
            assert_eq!(result, expected);
            // should not have visited every single one
            assert!(counter.into_inner() < a.len());
        }
    });
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn check_partial_cmp_short_circuit() {
    // We only use a single thread to make the short-circuit behavior deterministic.
    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();

    let a = vec![0; 1024];
    let mut b = a.clone();
    b[42] = 1;

    pool.install(|| {
        let expected = Some(::std::cmp::Ordering::Less);
        assert_eq!(a.par_iter().partial_cmp(&b), expected);

        for len in 1..10 {
            let counter = AtomicUsize::new(0);
            let result = a
                .par_iter()
                .with_max_len(len)
                .inspect(|_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                })
                .partial_cmp(&b);
            assert_eq!(result, expected);
            // should not have visited every single one
            assert!(counter.into_inner() < a.len());
        }
    });
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn check_partial_cmp_nan_short_circuit() {
    // We only use a single thread to make the short-circuit behavior deterministic.
    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();

    let a = vec![0.0; 1024];
    let mut b = a.clone();
    b[42] = f64::NAN;

    pool.install(|| {
        let expected = None;
        assert_eq!(a.par_iter().partial_cmp(&b), expected);

        for len in 1..10 {
            let counter = AtomicUsize::new(0);
            let result = a
                .par_iter()
                .with_max_len(len)
                .inspect(|_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                })
                .partial_cmp(&b);
            assert_eq!(result, expected);
            // should not have visited every single one
            assert!(counter.into_inner() < a.len());
        }
    });
}

#[test]
fn check_partial_cmp_direct() {
    let a = (0..1024).into_par_iter();
    let b = (0..1024).into_par_iter();

    let result = a.partial_cmp(b);

    assert!(result == Some(::std::cmp::Ordering::Equal));
}

#[test]
fn check_partial_cmp_to_seq() {
    let par_result = (0..1024).into_par_iter().partial_cmp(0..1024);
    let seq_result = (0..1024).partial_cmp(0..1024);
    assert_eq!(par_result, seq_result);
}

#[test]
fn check_partial_cmp_rng_to_seq() {
    let mut rng = seeded_rng();
    let rng = &mut rng;
    let a: Vec<i32> = rng.sample_iter(&StandardUniform).take(1024).collect();
    let b: Vec<i32> = rng.sample_iter(&StandardUniform).take(1024).collect();
    for i in 0..a.len() {
        let par_result = a[i..].par_iter().partial_cmp(b[i..].par_iter());
        let seq_result = a[i..].iter().partial_cmp(b[i..].iter());

        assert_eq!(par_result, seq_result);
    }
}

#[test]
fn check_partial_cmp_lt_direct() {
    let a = (0..1024).into_par_iter();
    let b = (1..1024).into_par_iter();

    let result = a.partial_cmp(b);

    assert!(result == Some(::std::cmp::Ordering::Less));
}

#[test]
fn check_partial_cmp_lt_to_seq() {
    let par_result = (0..1024).into_par_iter().partial_cmp(1..1024);
    let seq_result = (0..1024).partial_cmp(1..1024);
    assert_eq!(par_result, seq_result);
}

#[test]
fn check_partial_cmp_gt_direct() {
    let a = (1..1024).into_par_iter();
    let b = (0..1024).into_par_iter();

    let result = a.partial_cmp(b);

    assert!(result == Some(::std::cmp::Ordering::Greater));
}

#[test]
fn check_partial_cmp_gt_to_seq() {
    let par_result = (1..1024).into_par_iter().partial_cmp(0..1024);
    let seq_result = (1..1024).partial_cmp(0..1024);
    assert_eq!(par_result, seq_result);
}

#[test]
fn check_partial_cmp_none_direct() {
    let a = vec![f64::NAN, 0.0];
    let b = vec![0.0, 1.0];

    let result = a.par_iter().partial_cmp(b.par_iter());

    assert!(result.is_none());
}

#[test]
fn check_partial_cmp_none_to_seq() {
    let a = vec![f64::NAN, 0.0];
    let b = vec![0.0, 1.0];

    let par_result = a.par_iter().partial_cmp(b.par_iter());
    let seq_result = a.iter().partial_cmp(b.iter());

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_partial_cmp_late_nan_direct() {
    let a = vec![0.0, f64::NAN];
    let b = vec![1.0, 1.0];

    let result = a.par_iter().partial_cmp(b.par_iter());

    assert!(result == Some(::std::cmp::Ordering::Less));
}

#[test]
fn check_partial_cmp_late_nan_to_seq() {
    let a = vec![0.0, f64::NAN];
    let b = vec![1.0, 1.0];

    let par_result = a.par_iter().partial_cmp(b.par_iter());
    let seq_result = a.iter().partial_cmp(b.iter());

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_cmp_lengths() {
    // comparisons should consider length if they are otherwise equal
    let a = vec![0; 1024];
    let b = vec![0; 1025];

    assert_eq!(a.par_iter().cmp(&b), a.iter().cmp(&b));
    assert_eq!(a.par_iter().partial_cmp(&b), a.iter().partial_cmp(&b));
}

#[test]
fn check_eq_direct() {
    let a = (0..1024).into_par_iter();
    let b = (0..1024).into_par_iter();

    let result = a.eq(b);

    assert!(result);
}

#[test]
fn check_eq_to_seq() {
    let par_result = (0..1024).into_par_iter().eq((0..1024).into_par_iter());
    let seq_result = (0..1024).eq(0..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_ne_direct() {
    let a = (0..1024).into_par_iter();
    let b = (1..1024).into_par_iter();

    let result = a.ne(b);

    assert!(result);
}

#[test]
fn check_ne_to_seq() {
    let par_result = (0..1024).into_par_iter().ne((1..1025).into_par_iter());
    let seq_result = (0..1024).ne(1..1025);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_ne_lengths() {
    // equality should consider length too
    let a = vec![0; 1024];
    let b = vec![0; 1025];

    assert_eq!(a.par_iter().eq(&b), a.iter().eq(&b));
    assert_eq!(a.par_iter().ne(&b), a.iter().ne(&b));
}

#[test]
fn check_lt_direct() {
    assert!((0..1024).into_par_iter().lt(1..1024));
    assert!(!(1..1024).into_par_iter().lt(0..1024));
}

#[test]
fn check_lt_to_seq() {
    let par_result = (0..1024).into_par_iter().lt((1..1024).into_par_iter());
    let seq_result = (0..1024).lt(1..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_le_equal_direct() {
    assert!((0..1024).into_par_iter().le((0..1024).into_par_iter()));
}

#[test]
fn check_le_equal_to_seq() {
    let par_result = (0..1024).into_par_iter().le((0..1024).into_par_iter());
    let seq_result = (0..1024).le(0..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_le_less_direct() {
    assert!((0..1024).into_par_iter().le((1..1024).into_par_iter()));
}

#[test]
fn check_le_less_to_seq() {
    let par_result = (0..1024).into_par_iter().le((1..1024).into_par_iter());
    let seq_result = (0..1024).le(1..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_gt_direct() {
    assert!((1..1024).into_par_iter().gt((0..1024).into_par_iter()));
}

#[test]
fn check_gt_to_seq() {
    let par_result = (1..1024).into_par_iter().gt((0..1024).into_par_iter());
    let seq_result = (1..1024).gt(0..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_ge_equal_direct() {
    assert!((0..1024).into_par_iter().ge((0..1024).into_par_iter()));
}

#[test]
fn check_ge_equal_to_seq() {
    let par_result = (0..1024).into_par_iter().ge((0..1024).into_par_iter());
    let seq_result = (0..1024).ge(0..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_ge_greater_direct() {
    assert!((1..1024).into_par_iter().ge((0..1024).into_par_iter()));
}

#[test]
fn check_ge_greater_to_seq() {
    let par_result = (1..1024).into_par_iter().ge((0..1024).into_par_iter());
    let seq_result = (1..1024).ge(0..1024);

    assert_eq!(par_result, seq_result);
}

#[test]
fn check_zip() {
    let mut a: Vec<usize> = (0..1024).rev().collect();
    let b: Vec<usize> = (0..1024).collect();

    a.par_iter_mut().zip(&b[..]).for_each(|(a, &b)| *a += b);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_zip_into_par_iter() {
    let mut a: Vec<usize> = (0..1024).rev().collect();
    let b: Vec<usize> = (0..1024).collect();

    a.par_iter_mut()
        .zip(&b) // here we rely on &b iterating over &usize
        .for_each(|(a, &b)| *a += b);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_zip_into_mut_par_iter() {
    let a: Vec<usize> = (0..1024).rev().collect();
    let mut b: Vec<usize> = (0..1024).collect();

    a.par_iter().zip(&mut b).for_each(|(&a, b)| *b += a);

    assert!(b.iter().all(|&x| x == b.len() - 1));
}

#[test]
fn check_zip_range() {
    let mut a: Vec<usize> = (0..1024).rev().collect();

    a.par_iter_mut()
        .zip(0usize..1024)
        .for_each(|(a, b)| *a += b);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_zip_eq() {
    let mut a: Vec<usize> = (0..1024).rev().collect();
    let b: Vec<usize> = (0..1024).collect();

    a.par_iter_mut().zip_eq(&b[..]).for_each(|(a, &b)| *a += b);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_zip_eq_into_par_iter() {
    let mut a: Vec<usize> = (0..1024).rev().collect();
    let b: Vec<usize> = (0..1024).collect();

    a.par_iter_mut()
        .zip_eq(&b) // here we rely on &b iterating over &usize
        .for_each(|(a, &b)| *a += b);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_zip_eq_into_mut_par_iter() {
    let a: Vec<usize> = (0..1024).rev().collect();
    let mut b: Vec<usize> = (0..1024).collect();

    a.par_iter().zip_eq(&mut b).for_each(|(&a, b)| *b += a);

    assert!(b.iter().all(|&x| x == b.len() - 1));
}

#[test]
fn check_zip_eq_range() {
    let mut a: Vec<usize> = (0..1024).rev().collect();

    a.par_iter_mut()
        .zip_eq(0usize..1024)
        .for_each(|(a, b)| *a += b);

    assert!(a.iter().all(|&x| x == a.len() - 1));
}

#[test]
fn check_sum_filtered_ints() {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let par_sum_evens: i32 = a.par_iter().filter(|&x| (x & 1) == 0).sum();
    let seq_sum_evens = a.iter().filter(|&x| (x & 1) == 0).sum();
    assert_eq!(par_sum_evens, seq_sum_evens);
}

#[test]
fn check_sum_filtermap_ints() {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let par_sum_evens: u32 = a
        .par_iter()
        .filter_map(|&x| if (x & 1) == 0 { Some(x as u32) } else { None })
        .sum();
    let seq_sum_evens = a
        .iter()
        .filter_map(|&x| if (x & 1) == 0 { Some(x as u32) } else { None })
        .sum();
    assert_eq!(par_sum_evens, seq_sum_evens);
}

#[test]
fn check_flat_map_nested_ranges() {
    // FIXME -- why are precise type hints required on the integers here?

    let v: i32 = (0_i32..10)
        .into_par_iter()
        .flat_map(|i| (0_i32..10).into_par_iter().map(move |j| (i, j)))
        .map(|(i, j)| i * j)
        .sum();

    let w = (0_i32..10)
        .flat_map(|i| (0_i32..10).map(move |j| (i, j)))
        .map(|(i, j)| i * j)
        .sum();

    assert_eq!(v, w);
}

#[test]
fn check_empty_flat_map_sum() {
    let a: Vec<i32> = (0..1024).collect();
    let empty = &a[..0];

    // empty on the inside
    let b: i32 = a.par_iter().flat_map(|_| empty).sum();
    assert_eq!(b, 0);

    // empty on the outside
    let c: i32 = empty.par_iter().flat_map(|_| a.par_iter()).sum();
    assert_eq!(c, 0);
}

#[test]
fn check_flatten_vec() {
    let a: Vec<i32> = (0..1024).collect();
    let b: Vec<Vec<i32>> = vec![a.clone(), a.clone(), a.clone(), a.clone()];
    let c: Vec<i32> = b.par_iter().flatten().cloned().collect();
    let mut d = a.clone();
    d.extend(&a);
    d.extend(&a);
    d.extend(&a);

    assert_eq!(d, c);
}

#[test]
fn check_flatten_vec_empty() {
    let a: Vec<Vec<i32>> = vec![vec![]];
    let b: Vec<i32> = a.par_iter().flatten().cloned().collect();

    assert_eq!(vec![] as Vec<i32>, b);
}

#[test]
fn check_slice_split() {
    let v: Vec<_> = (0..1000).collect();
    for m in 1..100 {
        let a: Vec<_> = v.split(|x| x % m == 0).collect();
        let b: Vec<_> = v.par_split(|x| x % m == 0).collect();
        assert_eq!(a, b);
    }

    // same as std::slice::split() examples
    let slice = [10, 40, 33, 20];
    let v: Vec<_> = slice.par_split(|num| num % 3 == 0).collect();
    assert_eq!(v, &[&slice[..2], &slice[3..]]);

    let slice = [10, 40, 33];
    let v: Vec<_> = slice.par_split(|num| num % 3 == 0).collect();
    assert_eq!(v, &[&slice[..2], &slice[..0]]);

    let slice = [10, 6, 33, 20];
    let v: Vec<_> = slice.par_split(|num| num % 3 == 0).collect();
    assert_eq!(v, &[&slice[..1], &slice[..0], &slice[3..]]);
}

#[test]
fn check_slice_split_inclusive() {
    let v: Vec<_> = (0..1000).collect();
    for m in 1..100 {
        let a: Vec<_> = v.split_inclusive(|x| x % m == 0).collect();
        let b: Vec<_> = v.par_split_inclusive(|x| x % m == 0).collect();
        assert_eq!(a, b);
    }

    // same as std::slice::split_inclusive() examples
    let slice = [10, 40, 33, 20];
    let v: Vec<_> = slice.par_split_inclusive(|num| num % 3 == 0).collect();
    assert_eq!(v, &[&slice[..3], &slice[3..]]);

    let slice = [3, 10, 40, 33];
    let v: Vec<_> = slice.par_split_inclusive(|num| num % 3 == 0).collect();
    assert_eq!(v, &[&slice[..1], &slice[1..]]);
}

#[test]
fn check_slice_split_mut() {
    let mut v1: Vec<_> = (0..1000).collect();
    let mut v2 = v1.clone();
    for m in 1..100 {
        let a: Vec<_> = v1.split_mut(|x| x % m == 0).collect();
        let b: Vec<_> = v2.par_split_mut(|x| x % m == 0).collect();
        assert_eq!(a, b);
    }

    // same as std::slice::split_mut() example
    let mut v = [10, 40, 30, 20, 60, 50];
    v.par_split_mut(|num| num % 3 == 0).for_each(|group| {
        group[0] = 1;
    });
    assert_eq!(v, [1, 40, 30, 1, 60, 1]);
}

#[test]
fn check_slice_split_inclusive_mut() {
    let mut v1: Vec<_> = (0..1000).collect();
    let mut v2 = v1.clone();
    for m in 1..100 {
        let a: Vec<_> = v1.split_inclusive_mut(|x| x % m == 0).collect();
        let b: Vec<_> = v2.par_split_inclusive_mut(|x| x % m == 0).collect();
        assert_eq!(a, b);
    }

    // same as std::slice::split_inclusive_mut() example
    let mut v = [10, 40, 30, 20, 60, 50];
    v.par_split_inclusive_mut(|num| num % 3 == 0)
        .for_each(|group| {
            let terminator_idx = group.len() - 1;
            group[terminator_idx] = 1;
        });
    assert_eq!(v, [10, 40, 1, 20, 1, 1]);
}

#[test]
fn check_chunks() {
    let a: Vec<i32> = vec![1, 5, 10, 4, 100, 3, 1000, 2, 10000, 1];
    let par_sum_product_pairs: i32 = a.par_chunks(2).map(|c| c.iter().product::<i32>()).sum();
    let seq_sum_product_pairs = a.chunks(2).map(|c| c.iter().product::<i32>()).sum();
    assert_eq!(par_sum_product_pairs, 12345);
    assert_eq!(par_sum_product_pairs, seq_sum_product_pairs);

    let par_sum_product_triples: i32 = a.par_chunks(3).map(|c| c.iter().product::<i32>()).sum();
    let seq_sum_product_triples = a.chunks(3).map(|c| c.iter().product::<i32>()).sum();
    assert_eq!(par_sum_product_triples, 5_0 + 12_00 + 20_000_000 + 1);
    assert_eq!(par_sum_product_triples, seq_sum_product_triples);
}

#[test]
fn check_chunks_mut() {
    let mut a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut b: Vec<i32> = a.clone();
    a.par_chunks_mut(2).for_each(|c| c[0] = c.iter().sum());
    b.chunks_mut(2).for_each(|c| c[0] = c.iter().sum());
    assert_eq!(a, &[3, 2, 7, 4, 11, 6, 15, 8, 19, 10]);
    assert_eq!(a, b);

    let mut a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut b: Vec<i32> = a.clone();
    a.par_chunks_mut(3).for_each(|c| c[0] = c.iter().sum());
    b.chunks_mut(3).for_each(|c| c[0] = c.iter().sum());
    assert_eq!(a, &[6, 2, 3, 15, 5, 6, 24, 8, 9, 10]);
    assert_eq!(a, b);
}

#[test]
fn check_windows() {
    let a: Vec<i32> = (0..1024).collect();
    let par: Vec<_> = a.par_windows(2).collect();
    let seq: Vec<_> = a.windows(2).collect();
    assert_eq!(par, seq);

    let par: Vec<_> = a.par_windows(100).collect();
    let seq: Vec<_> = a.windows(100).collect();
    assert_eq!(par, seq);

    let par: Vec<_> = a.par_windows(1_000_000).collect();
    let seq: Vec<_> = a.windows(1_000_000).collect();
    assert_eq!(par, seq);

    let par: Vec<_> = a
        .par_windows(2)
        .chain(a.par_windows(1_000_000))
        .zip(a.par_windows(2))
        .collect();
    let seq: Vec<_> = a
        .windows(2)
        .chain(a.windows(1_000_000))
        .zip(a.windows(2))
        .collect();
    assert_eq!(par, seq);
}

#[test]
fn check_options() {
    let mut a = vec![None, Some(1), None, None, Some(2), Some(4)];

    assert_eq!(7, a.par_iter().flat_map(|opt| opt).sum::<i32>());
    assert_eq!(7, a.par_iter().flat_map(|opt| opt).sum::<i32>());

    a.par_iter_mut()
        .flat_map(|opt| opt)
        .for_each(|x| *x = *x * *x);

    assert_eq!(21, a.into_par_iter().flat_map(|opt| opt).sum::<i32>());
}

#[test]
fn check_results() {
    let mut a = vec![Err(()), Ok(1i32), Err(()), Err(()), Ok(2), Ok(4)];

    assert_eq!(7, a.par_iter().flat_map(|res| res).sum::<i32>());

    assert_eq!(Err::<i32, ()>(()), a.par_iter().cloned().sum());
    assert_eq!(Ok(7), a.par_iter().cloned().filter(Result::is_ok).sum());

    assert_eq!(Err::<i32, ()>(()), a.par_iter().cloned().product());
    assert_eq!(Ok(8), a.par_iter().cloned().filter(Result::is_ok).product());

    a.par_iter_mut()
        .flat_map(|res| res)
        .for_each(|x| *x = *x * *x);

    assert_eq!(21, a.into_par_iter().flat_map(|res| res).sum::<i32>());
}

#[test]
fn check_binary_heap() {
    use std::collections::BinaryHeap;

    let a: BinaryHeap<i32> = (0..10).collect();

    assert_eq!(45, a.par_iter().sum::<i32>());
    assert_eq!(45, a.into_par_iter().sum::<i32>());
}

#[test]
fn check_btree_map() {
    use std::collections::BTreeMap;

    let mut a: BTreeMap<i32, i32> = (0..10).map(|i| (i, -i)).collect();

    assert_eq!(45, a.par_iter().map(|(&k, _)| k).sum::<i32>());
    assert_eq!(-45, a.par_iter().map(|(_, &v)| v).sum::<i32>());

    a.par_iter_mut().for_each(|(k, v)| *v += *k);

    assert_eq!(0, a.into_par_iter().map(|(_, v)| v).sum::<i32>());
}

#[test]
fn check_btree_set() {
    use std::collections::BTreeSet;

    let a: BTreeSet<i32> = (0..10).collect();

    assert_eq!(45, a.par_iter().sum::<i32>());
    assert_eq!(45, a.into_par_iter().sum::<i32>());
}

#[test]
fn check_hash_map() {
    use std::collections::HashMap;

    let mut a: HashMap<i32, i32> = (0..10).map(|i| (i, -i)).collect();

    assert_eq!(45, a.par_iter().map(|(&k, _)| k).sum::<i32>());
    assert_eq!(-45, a.par_iter().map(|(_, &v)| v).sum::<i32>());

    a.par_iter_mut().for_each(|(k, v)| *v += *k);

    assert_eq!(0, a.into_par_iter().map(|(_, v)| v).sum::<i32>());
}

#[test]
fn check_hash_set() {
    use std::collections::HashSet;

    let a: HashSet<i32> = (0..10).collect();

    assert_eq!(45, a.par_iter().sum::<i32>());
    assert_eq!(45, a.into_par_iter().sum::<i32>());
}

#[test]
fn check_linked_list() {
    use std::collections::LinkedList;

    let mut a: LinkedList<i32> = (0..10).collect();

    assert_eq!(45, a.par_iter().sum::<i32>());

    a.par_iter_mut().for_each(|x| *x = -*x);

    assert_eq!(-45, a.into_par_iter().sum::<i32>());
}

#[test]
fn check_vec_deque() {
    use std::collections::VecDeque;

    let mut a: VecDeque<i32> = (0..10).collect();

    // try to get it to wrap around
    a.drain(..5);
    a.extend(0..5);

    assert_eq!(45, a.par_iter().sum::<i32>());

    a.par_iter_mut().for_each(|x| *x = -*x);

    assert_eq!(-45, a.into_par_iter().sum::<i32>());
}

#[test]
fn check_chain() {
    let mut res = vec![];

    // stays indexed in the face of madness
    Some(0)
        .into_par_iter()
        .chain(Ok::<_, ()>(1))
        .chain(1..4)
        .chain(Err("huh?"))
        .chain(None)
        .chain(vec![5, 8, 13])
        .map(|x| (x as u8 + b'a') as char)
        .chain(vec!['x', 'y', 'z'])
        .zip((0i32..1000).into_par_iter().map(|x| -x))
        .enumerate()
        .map(|(a, (b, c))| (a, b, c))
        .chain(None)
        .collect_into_vec(&mut res);

    assert_eq!(
        res,
        vec![
            (0, 'a', 0),
            (1, 'b', -1),
            (2, 'b', -2),
            (3, 'c', -3),
            (4, 'd', -4),
            (5, 'f', -5),
            (6, 'i', -6),
            (7, 'n', -7),
            (8, 'x', -8),
            (9, 'y', -9),
            (10, 'z', -10)
        ]
    );

    // unindexed is ok too
    let res: Vec<i32> = Some(1i32)
        .into_par_iter()
        .chain(
            (2i32..4)
                .into_par_iter()
                .chain(vec![5, 6, 7, 8, 9])
                .chain(Some((10, 100)).into_par_iter().flat_map(|(a, b)| a..b))
                .filter(|x| x & 1 == 1),
        )
        .collect();
    let other: Vec<i32> = (0..100).filter(|x| x & 1 == 1).collect();
    assert_eq!(res, other);

    // chain collect is ok with the "fake" specialization
    let res: Vec<i32> = Some(1i32).into_par_iter().chain(None).collect();
    assert_eq!(res, &[1]);
}

#[test]
fn check_count() {
    let c0 = (0_u32..24 * 1024).filter(|i| i % 2 == 0).count();
    let c1 = (0_u32..24 * 1024)
        .into_par_iter()
        .filter(|i| i % 2 == 0)
        .count();
    assert_eq!(c0, c1);
}

#[test]
fn find_any() {
    let a: Vec<i32> = (0..1024).collect();

    assert!(a.par_iter().find_any(|&&x| x % 42 == 41).is_some());
    assert_eq!(
        a.par_iter().find_any(|&&x| x % 19 == 1 && x % 53 == 0),
        Some(&742_i32)
    );
    assert_eq!(a.par_iter().find_any(|&&x| x < 0), None);

    assert!(a.par_iter().position_any(|&x| x % 42 == 41).is_some());
    assert_eq!(
        a.par_iter().position_any(|&x| x % 19 == 1 && x % 53 == 0),
        Some(742_usize)
    );
    assert_eq!(a.par_iter().position_any(|&x| x < 0), None);

    assert!(a.par_iter().any(|&x| x > 1000));
    assert!(!a.par_iter().any(|&x| x < 0));

    assert!(!a.par_iter().all(|&x| x > 1000));
    assert!(a.par_iter().all(|&x| x >= 0));
}

#[test]
fn find_first_or_last() {
    let a: Vec<i32> = (0..1024).collect();

    assert_eq!(a.par_iter().find_first(|&&x| x % 42 == 41), Some(&41_i32));
    assert_eq!(
        a.par_iter().find_first(|&&x| x % 19 == 1 && x % 53 == 0),
        Some(&742_i32)
    );
    assert_eq!(a.par_iter().find_first(|&&x| x < 0), None);

    assert_eq!(
        a.par_iter().position_first(|&x| x % 42 == 41),
        Some(41_usize)
    );
    assert_eq!(
        a.par_iter().position_first(|&x| x % 19 == 1 && x % 53 == 0),
        Some(742_usize)
    );
    assert_eq!(a.par_iter().position_first(|&x| x < 0), None);

    assert_eq!(a.par_iter().find_last(|&&x| x % 42 == 41), Some(&1007_i32));
    assert_eq!(
        a.par_iter().find_last(|&&x| x % 19 == 1 && x % 53 == 0),
        Some(&742_i32)
    );
    assert_eq!(a.par_iter().find_last(|&&x| x < 0), None);

    assert_eq!(
        a.par_iter().position_last(|&x| x % 42 == 41),
        Some(1007_usize)
    );
    assert_eq!(
        a.par_iter().position_last(|&x| x % 19 == 1 && x % 53 == 0),
        Some(742_usize)
    );
    assert_eq!(a.par_iter().position_last(|&x| x < 0), None);
}

#[test]
fn find_map_first_or_last_or_any() {
    let mut a: Vec<i32> = vec![];

    assert!(a.par_iter().find_map_any(half_if_positive).is_none());
    assert!(a.par_iter().find_map_first(half_if_positive).is_none());
    assert!(a.par_iter().find_map_last(half_if_positive).is_none());

    a = (-1024..-3).collect();

    assert!(a.par_iter().find_map_any(half_if_positive).is_none());
    assert!(a.par_iter().find_map_first(half_if_positive).is_none());
    assert!(a.par_iter().find_map_last(half_if_positive).is_none());

    assert!(a.par_iter().find_map_any(half_if_negative).is_some());
    assert_eq!(
        a.par_iter().find_map_first(half_if_negative),
        Some(-512_i32)
    );
    assert_eq!(a.par_iter().find_map_last(half_if_negative), Some(-2_i32));

    a.append(&mut (2..1025).collect());

    assert!(a.par_iter().find_map_any(half_if_positive).is_some());
    assert_eq!(a.par_iter().find_map_first(half_if_positive), Some(1_i32));
    assert_eq!(a.par_iter().find_map_last(half_if_positive), Some(512_i32));

    fn half_if_positive(x: &i32) -> Option<i32> {
        if *x > 0 {
            Some(x / 2)
        } else {
            None
        }
    }

    fn half_if_negative(x: &i32) -> Option<i32> {
        if *x < 0 {
            Some(x / 2)
        } else {
            None
        }
    }
}

#[test]
fn check_find_not_present() {
    let counter = AtomicUsize::new(0);
    let value: Option<i32> = (0_i32..2048).into_par_iter().find_any(|&p| {
        counter.fetch_add(1, Ordering::SeqCst);
        p >= 2048
    });
    assert!(value.is_none());
    assert!(counter.load(Ordering::SeqCst) == 2048); // should have visited every single one
}

#[test]
fn check_find_is_present() {
    let counter = AtomicUsize::new(0);
    let value: Option<i32> = (0_i32..2048).into_par_iter().find_any(|&p| {
        counter.fetch_add(1, Ordering::SeqCst);
        (1024..1096).contains(&p)
    });
    let q = value.unwrap();
    assert!((1024..1096).contains(&q));
    assert!(counter.load(Ordering::SeqCst) < 2048); // should not have visited every single one
}

#[test]
fn check_while_some() {
    let value = (0_i32..2048).into_par_iter().map(Some).while_some().max();
    assert_eq!(value, Some(2047));

    let counter = AtomicUsize::new(0);
    let value = (0_i32..2048)
        .into_par_iter()
        .map(|x| {
            counter.fetch_add(1, Ordering::SeqCst);
            if x < 1024 {
                Some(x)
            } else {
                None
            }
        })
        .while_some()
        .max();
    assert!(value < Some(1024));
    assert!(counter.load(Ordering::SeqCst) < 2048); // should not have visited every single one
}

#[test]
fn par_iter_collect_option() {
    let a: Option<Vec<_>> = (0_i32..2048).map(Some).collect();
    let b: Option<Vec<_>> = (0_i32..2048).into_par_iter().map(Some).collect();
    assert_eq!(a, b);

    let c: Option<Vec<_>> = (0_i32..2048)
        .into_par_iter()
        .map(|x| if x == 1234 { None } else { Some(x) })
        .collect();
    assert_eq!(c, None);
}

#[test]
fn par_iter_collect_result() {
    let a: Result<Vec<_>, ()> = (0_i32..2048).map(Ok).collect();
    let b: Result<Vec<_>, ()> = (0_i32..2048).into_par_iter().map(Ok).collect();
    assert_eq!(a, b);

    let c: Result<Vec<_>, _> = (0_i32..2048)
        .into_par_iter()
        .map(|x| if x == 1234 { Err(x) } else { Ok(x) })
        .collect();
    assert_eq!(c, Err(1234));

    let d: Result<Vec<_>, _> = (0_i32..2048)
        .into_par_iter()
        .map(|x| if x % 100 == 99 { Err(x) } else { Ok(x) })
        .collect();
    assert_eq!(d.map_err(|x| x % 100), Err(99));
}

#[test]
fn par_iter_collect() {
    let a: Vec<i32> = (0..1024).collect();
    let b: Vec<i32> = a.par_iter().map(|&i| i + 1).collect();
    let c: Vec<i32> = (0..1024).map(|i| i + 1).collect();
    assert_eq!(b, c);
}

#[test]
fn par_iter_collect_vecdeque() {
    let a: Vec<i32> = (0..1024).collect();
    let b: VecDeque<i32> = a.par_iter().cloned().collect();
    let c: VecDeque<i32> = a.iter().cloned().collect();
    assert_eq!(b, c);
}

#[test]
fn par_iter_collect_binaryheap() {
    let a: Vec<i32> = (0..1024).collect();
    let mut b: BinaryHeap<i32> = a.par_iter().cloned().collect();
    assert_eq!(b.peek(), Some(&1023));
    assert_eq!(b.len(), 1024);
    for n in (0..1024).rev() {
        assert_eq!(b.pop(), Some(n));
        assert_eq!(b.len() as i32, n);
    }
}

#[test]
fn par_iter_collect_hashmap() {
    let a: Vec<i32> = (0..1024).collect();
    let b: HashMap<i32, String> = a.par_iter().map(|&i| (i, format!("{i}"))).collect();
    assert_eq!(&b[&3], "3");
    assert_eq!(b.len(), 1024);
}

#[test]
fn par_iter_collect_hashset() {
    let a: Vec<i32> = (0..1024).collect();
    let b: HashSet<i32> = a.par_iter().cloned().collect();
    assert_eq!(b.len(), 1024);
}

#[test]
fn par_iter_collect_btreemap() {
    let a: Vec<i32> = (0..1024).collect();
    let b: BTreeMap<i32, String> = a.par_iter().map(|&i| (i, format!("{i}"))).collect();
    assert_eq!(&b[&3], "3");
    assert_eq!(b.len(), 1024);
}

#[test]
fn par_iter_collect_btreeset() {
    let a: Vec<i32> = (0..1024).collect();
    let b: BTreeSet<i32> = a.par_iter().cloned().collect();
    assert_eq!(b.len(), 1024);
}

#[test]
fn par_iter_collect_linked_list() {
    let a: Vec<i32> = (0..1024).collect();
    let b: LinkedList<_> = a.par_iter().map(|&i| (i, format!("{i}"))).collect();
    let c: LinkedList<_> = a.iter().map(|&i| (i, format!("{i}"))).collect();
    assert_eq!(b, c);
}

#[test]
fn par_iter_collect_linked_list_flat_map_filter() {
    let b: LinkedList<i32> = (0_i32..1024)
        .into_par_iter()
        .flat_map(|i| 0..i)
        .filter(|&i| i % 2 == 0)
        .collect();
    let c: LinkedList<i32> = (0_i32..1024)
        .flat_map(|i| 0..i)
        .filter(|&i| i % 2 == 0)
        .collect();
    assert_eq!(b, c);
}

#[test]
fn par_iter_collect_cows() {
    use std::borrow::Cow;

    let s = "Fearless Concurrency with Rust";

    // Collects `i32` into a `Vec`
    let a: Cow<'_, [i32]> = (0..1024).collect();
    let b: Cow<'_, [i32]> = a.par_iter().cloned().collect();
    assert_eq!(a, b);

    // Collects `char` into a `String`
    let a: Cow<'_, str> = s.chars().collect();
    let b: Cow<'_, str> = s.par_chars().collect();
    assert_eq!(a, b);

    // Collects `str` into a `String`
    let sw = s.split_whitespace();
    let psw = s.par_split_whitespace();
    let a: Cow<'_, str> = sw.clone().collect();
    let b: Cow<'_, str> = psw.clone().collect();
    assert_eq!(a, b);

    // Collects `String` into a `String`
    let a: Cow<'_, str> = sw.map(str::to_owned).collect();
    let b: Cow<'_, str> = psw.map(str::to_owned).collect();
    assert_eq!(a, b);

    // Collects `OsStr` into a `OsString`
    let sw = s.split_whitespace().map(OsStr::new);
    let psw = s.par_split_whitespace().map(OsStr::new);
    let a: Cow<'_, OsStr> = Cow::Owned(sw.clone().collect());
    let b: Cow<'_, OsStr> = psw.clone().collect();
    assert_eq!(a, b);

    // Collects `OsString` into a `OsString`
    let a: Cow<'_, OsStr> = Cow::Owned(sw.map(OsStr::to_owned).collect());
    let b: Cow<'_, OsStr> = psw.map(OsStr::to_owned).collect();
    assert_eq!(a, b);
}

#[test]
fn par_iter_unindexed_flat_map() {
    let b: Vec<i64> = (0_i64..1024).into_par_iter().flat_map(Some).collect();
    let c: Vec<i64> = (0_i64..1024).flat_map(Some).collect();
    assert_eq!(b, c);
}

#[test]
fn min_max() {
    let rng = seeded_rng();
    let a: Vec<i32> = rng.sample_iter(&StandardUniform).take(1024).collect();
    for i in 0..=a.len() {
        let slice = &a[..i];
        assert_eq!(slice.par_iter().min(), slice.iter().min());
        assert_eq!(slice.par_iter().max(), slice.iter().max());
    }
}

#[test]
fn min_max_by() {
    let rng = seeded_rng();
    // Make sure there are duplicate keys, for testing sort stability
    let r: Vec<i32> = rng.sample_iter(&StandardUniform).take(512).collect();
    let a: Vec<(i32, u16)> = r.iter().chain(&r).cloned().zip(0..).collect();
    for i in 0..=a.len() {
        let slice = &a[..i];
        assert_eq!(
            slice.par_iter().min_by(|x, y| x.0.cmp(&y.0)),
            slice.iter().min_by(|x, y| x.0.cmp(&y.0))
        );
        assert_eq!(
            slice.par_iter().max_by(|x, y| x.0.cmp(&y.0)),
            slice.iter().max_by(|x, y| x.0.cmp(&y.0))
        );
    }
}

#[test]
fn min_max_by_key() {
    let rng = seeded_rng();
    // Make sure there are duplicate keys, for testing sort stability
    let r: Vec<i32> = rng.sample_iter(&StandardUniform).take(512).collect();
    let a: Vec<(i32, u16)> = r.iter().chain(&r).cloned().zip(0..).collect();
    for i in 0..=a.len() {
        let slice = &a[..i];
        assert_eq!(
            slice.par_iter().min_by_key(|x| x.0),
            slice.iter().min_by_key(|x| x.0)
        );
        assert_eq!(
            slice.par_iter().max_by_key(|x| x.0),
            slice.iter().max_by_key(|x| x.0)
        );
    }
}

#[test]
fn check_rev() {
    let a: Vec<usize> = (0..1024).rev().collect();
    let b: Vec<usize> = (0..1024).collect();

    assert!(a.par_iter().rev().zip(b).all(|(&a, b)| a == b));
}

#[test]
fn scope_mix() {
    let counter_p = &AtomicUsize::new(0);
    scope(|s| {
        s.spawn(move |s| {
            divide_and_conquer(s, counter_p, 1024);
        });
        s.spawn(move |_| {
            let a: Vec<i32> = (0..1024).collect();
            let r1 = a.par_iter().map(|&i| i + 1).reduce_with(|i, j| i + j);
            let r2 = a.iter().map(|&i| i + 1).sum();
            assert_eq!(r1.unwrap(), r2);
        });
    });
}

fn divide_and_conquer<'scope>(scope: &Scope<'scope>, counter: &'scope AtomicUsize, size: usize) {
    if size > 1 {
        scope.spawn(move |scope| divide_and_conquer(scope, counter, size / 2));
        scope.spawn(move |scope| divide_and_conquer(scope, counter, size / 2));
    } else {
        // count the leaves
        counter.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn check_split() {
    use std::ops::Range;

    let a = (0..1024).into_par_iter();

    let b = split(0..1024, |Range { start, end }| {
        let mid = (end - start) / 2;
        if mid > start {
            (start..mid, Some(mid..end))
        } else {
            (start..end, None)
        }
    })
    .flat_map(|range| range);

    assert_eq!(a.collect::<Vec<_>>(), b.collect::<Vec<_>>());
}

#[test]
fn check_lengths() {
    fn check(min: usize, max: usize) {
        let range = 0..1024 * 1024;

        // Check against normalized values.
        let min_check = Ord::min(Ord::max(min, 1), range.len());
        let max_check = Ord::max(max, min_check.saturating_add(min_check - 1));

        assert!(
            range
                .into_par_iter()
                .with_min_len(min)
                .with_max_len(max)
                .fold(|| 0, |count, _| count + 1)
                .all(|c| c >= min_check && c <= max_check),
            "check_lengths failed {:?} -> {:?} ",
            (min, max),
            (min_check, max_check)
        );
    }

    let lengths = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, usize::MAX];
    for &min in &lengths {
        for &max in &lengths {
            check(min, max);
        }
    }
}

#[test]
fn check_map_with() {
    let (sender, receiver) = mpsc::channel();
    let a: HashSet<_> = (0..1024).collect();

    a.par_iter()
        .cloned()
        .map_with(sender, |s, i| s.send(i).unwrap())
        .count();

    let b: HashSet<_> = receiver.iter().collect();
    assert_eq!(a, b);
}

#[test]
fn check_fold_with() {
    let (sender, receiver) = mpsc::channel();
    let a: HashSet<_> = (0..1024).collect();

    a.par_iter()
        .cloned()
        .fold_with(sender, |s, i| {
            s.send(i).unwrap();
            s
        })
        .count();

    let b: HashSet<_> = receiver.iter().collect();
    assert_eq!(a, b);
}

#[test]
fn check_for_each_with() {
    let (sender, receiver) = mpsc::channel();
    let a: HashSet<_> = (0..1024).collect();

    a.par_iter()
        .cloned()
        .for_each_with(sender, |s, i| s.send(i).unwrap());

    let b: HashSet<_> = receiver.iter().collect();
    assert_eq!(a, b);
}

#[test]
fn check_extend_items() {
    fn check<C>()
    where
        C: Default
            + Eq
            + Debug
            + Extend<i32>
            + for<'a> Extend<&'a i32>
            + ParallelExtend<i32>
            + for<'a> ParallelExtend<&'a i32>,
    {
        let mut serial = C::default();
        let mut parallel = C::default();

        // extend with references
        let v: Vec<_> = (0..128).collect();
        serial.extend(&v);
        parallel.par_extend(&v);
        assert_eq!(serial, parallel);

        // extend with values
        serial.extend(-128..0);
        parallel.par_extend(-128..0);
        assert_eq!(serial, parallel);
    }

    check::<BTreeSet<_>>();
    check::<HashSet<_>>();
    check::<LinkedList<_>>();
    check::<Vec<_>>();
    check::<VecDeque<_>>();
}

#[test]
fn check_extend_heap() {
    let mut serial: BinaryHeap<_> = Default::default();
    let mut parallel: BinaryHeap<_> = Default::default();

    // extend with references
    let v: Vec<_> = (0..128).collect();
    serial.extend(&v);
    parallel.par_extend(&v);
    assert_eq!(
        serial.clone().into_sorted_vec(),
        parallel.clone().into_sorted_vec()
    );

    // extend with values
    serial.extend(-128..0);
    parallel.par_extend(-128..0);
    assert_eq!(serial.into_sorted_vec(), parallel.into_sorted_vec());
}

#[test]
fn check_extend_pairs() {
    fn check<C>()
    where
        C: Default
            + Eq
            + Debug
            + Extend<(usize, i32)>
            + for<'a> Extend<(&'a usize, &'a i32)>
            + ParallelExtend<(usize, i32)>
            + for<'a> ParallelExtend<(&'a usize, &'a i32)>,
    {
        let mut serial = C::default();
        let mut parallel = C::default();

        // extend with references
        let m: HashMap<_, _> = (0..128).enumerate().collect();
        serial.extend(&m);
        parallel.par_extend(&m);
        assert_eq!(serial, parallel);

        // extend with values
        let v: Vec<(_, _)> = (-128..0).enumerate().collect();
        serial.extend(v.clone());
        parallel.par_extend(v);
        assert_eq!(serial, parallel);
    }

    check::<BTreeMap<usize, i32>>();
    check::<HashMap<usize, i32>>();
}

#[test]
fn check_unzip_into_vecs() {
    let mut a = vec![];
    let mut b = vec![];
    (0..1024)
        .into_par_iter()
        .map(|i| i * i)
        .enumerate()
        .unzip_into_vecs(&mut a, &mut b);

    let (c, d): (Vec<_>, Vec<_>) = (0..1024).map(|i| i * i).enumerate().unzip();
    assert_eq!(a, c);
    assert_eq!(b, d);
}

#[test]
fn check_unzip() {
    // indexed, unindexed
    let (a, b): (Vec<_>, HashSet<_>) = (0..1024).into_par_iter().map(|i| i * i).enumerate().unzip();
    let (c, d): (Vec<_>, HashSet<_>) = (0..1024).map(|i| i * i).enumerate().unzip();
    assert_eq!(a, c);
    assert_eq!(b, d);

    // unindexed, indexed
    let (a, b): (HashSet<_>, Vec<_>) = (0..1024).into_par_iter().map(|i| i * i).enumerate().unzip();
    let (c, d): (HashSet<_>, Vec<_>) = (0..1024).map(|i| i * i).enumerate().unzip();
    assert_eq!(a, c);
    assert_eq!(b, d);

    // indexed, indexed
    let (a, b): (Vec<_>, Vec<_>) = (0..1024).into_par_iter().map(|i| i * i).enumerate().unzip();
    let (c, d): (Vec<_>, Vec<_>) = (0..1024).map(|i| i * i).enumerate().unzip();
    assert_eq!(a, c);
    assert_eq!(b, d);

    // unindexed producer
    let (a, b): (Vec<_>, Vec<_>) = (0..1024)
        .into_par_iter()
        .filter_map(|i| Some((i, i * i)))
        .unzip();
    let (c, d): (Vec<_>, Vec<_>) = (0..1024).map(|i| (i, i * i)).unzip();
    assert_eq!(a, c);
    assert_eq!(b, d);
}

#[test]
fn check_partition() {
    let (a, b): (Vec<_>, Vec<_>) = (0..1024).into_par_iter().partition(|&i| i % 3 == 0);
    let (c, d): (Vec<_>, Vec<_>) = (0..1024).partition(|&i| i % 3 == 0);
    assert_eq!(a, c);
    assert_eq!(b, d);
}

#[test]
fn check_partition_map() {
    let input = "a b c 1 2 3 x y z";
    let (a, b): (Vec<_>, String) =
        input
            .par_split_whitespace()
            .partition_map(|s| match s.parse::<i32>() {
                Ok(n) => Either::Left(n),
                Err(_) => Either::Right(s),
            });
    assert_eq!(a, vec![1, 2, 3]);
    assert_eq!(b, "abcxyz");
}

#[test]
fn check_either() {
    type I = crate::vec::IntoIter<i32>;
    type E = Either<I, I>;

    let v: Vec<i32> = (0..1024).collect();

    // try iterating the left side
    let left: E = Either::Left(v.clone().into_par_iter());
    assert!(left.eq(v.clone()));

    // try iterating the right side
    let right: E = Either::Right(v.clone().into_par_iter());
    assert!(right.eq(v.clone()));

    // try an indexed iterator
    let left: E = Either::Left(v.clone().into_par_iter());
    assert!(left.enumerate().eq(v.into_par_iter().enumerate()));
}

#[test]
fn check_either_extend() {
    type E = Either<Vec<i32>, HashSet<i32>>;

    let v: Vec<i32> = (0..1024).collect();

    // try extending the left side
    let mut left: E = Either::Left(vec![]);
    left.par_extend(v.clone());
    assert_eq!(left.as_ref(), Either::Left(&v));

    // try extending the right side
    let mut right: E = Either::Right(HashSet::default());
    right.par_extend(v.clone());
    assert_eq!(right, Either::Right(v.iter().cloned().collect()));
}

#[test]
fn check_interleave_eq() {
    let xs: Vec<usize> = (0..10).collect();
    let ys: Vec<usize> = (10..20).collect();

    let mut actual = vec![];
    xs.par_iter()
        .interleave(&ys)
        .map(|&i| i)
        .collect_into_vec(&mut actual);

    let expected: Vec<usize> = (0..10)
        .zip(10..20)
        .flat_map(|(i, j)| vec![i, j].into_iter())
        .collect();
    assert_eq!(expected, actual);
}

#[test]
fn check_interleave_uneven() {
    let cases: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> = vec![
        (
            (0..9).collect(),
            vec![10],
            vec![0, 10, 1, 2, 3, 4, 5, 6, 7, 8],
        ),
        (
            vec![10],
            (0..9).collect(),
            vec![10, 0, 1, 2, 3, 4, 5, 6, 7, 8],
        ),
        (
            (0..5).collect(),
            (5..10).collect(),
            (0..5)
                .zip(5..10)
                .flat_map(|(i, j)| vec![i, j].into_iter())
                .collect(),
        ),
        (vec![], (0..9).collect(), (0..9).collect()),
        ((0..9).collect(), vec![], (0..9).collect()),
        (
            (0..50).collect(),
            (50..100).collect(),
            (0..50)
                .zip(50..100)
                .flat_map(|(i, j)| vec![i, j].into_iter())
                .collect(),
        ),
    ];

    for (i, (xs, ys, expected)) in cases.into_iter().enumerate() {
        let mut res = vec![];
        xs.par_iter()
            .interleave(&ys)
            .map(|&i| i)
            .collect_into_vec(&mut res);
        assert_eq!(expected, res, "Case {i} failed");

        res.truncate(0);
        xs.par_iter()
            .interleave(&ys)
            .rev()
            .map(|&i| i)
            .collect_into_vec(&mut res);
        assert_eq!(
            expected.into_iter().rev().collect::<Vec<usize>>(),
            res,
            "Case {i} reversed failed"
        );
    }
}

#[test]
fn check_interleave_shortest() {
    let cases: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> = vec![
        ((0..9).collect(), vec![10], vec![0, 10, 1]),
        (vec![10], (0..9).collect(), vec![10, 0]),
        (
            (0..5).collect(),
            (5..10).collect(),
            (0..5)
                .zip(5..10)
                .flat_map(|(i, j)| vec![i, j].into_iter())
                .collect(),
        ),
        (vec![], (0..9).collect(), vec![]),
        ((0..9).collect(), vec![], vec![0]),
        (
            (0..50).collect(),
            (50..100).collect(),
            (0..50)
                .zip(50..100)
                .flat_map(|(i, j)| vec![i, j].into_iter())
                .collect(),
        ),
    ];

    for (i, (xs, ys, expected)) in cases.into_iter().enumerate() {
        let mut res = vec![];
        xs.par_iter()
            .interleave_shortest(&ys)
            .map(|&i| i)
            .collect_into_vec(&mut res);
        assert_eq!(expected, res, "Case {i} failed");

        res.truncate(0);
        xs.par_iter()
            .interleave_shortest(&ys)
            .rev()
            .map(|&i| i)
            .collect_into_vec(&mut res);
        assert_eq!(
            expected.into_iter().rev().collect::<Vec<usize>>(),
            res,
            "Case {i} reversed failed"
        );
    }
}

#[test]
#[should_panic(expected = "chunk_size must not be zero")]
fn check_chunks_zero_size() {
    let _: Vec<Vec<i32>> = vec![1, 2, 3].into_par_iter().chunks(0).collect();
}

#[test]
fn check_chunks_even_size() {
    assert_eq!(
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        (1..10).into_par_iter().chunks(3).collect::<Vec<Vec<i32>>>()
    );
}

#[test]
fn check_chunks_empty() {
    let v: Vec<i32> = vec![];
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(
        expected,
        v.into_par_iter().chunks(2).collect::<Vec<Vec<i32>>>()
    );
}

#[test]
fn check_chunks_len() {
    assert_eq!(4, (0..8).into_par_iter().chunks(2).len());
    assert_eq!(3, (0..9).into_par_iter().chunks(3).len());
    assert_eq!(3, (0..8).into_par_iter().chunks(3).len());
    assert_eq!(1, [1].par_iter().chunks(3).len());
    assert_eq!(0, (0..0).into_par_iter().chunks(3).len());
}

#[test]
fn check_chunks_uneven() {
    let cases: Vec<(Vec<u32>, usize, Vec<Vec<u32>>)> = vec![
        ((0..5).collect(), 3, vec![vec![0, 1, 2], vec![3, 4]]),
        (vec![1], 5, vec![vec![1]]),
        ((0..4).collect(), 3, vec![vec![0, 1, 2], vec![3]]),
    ];

    for (i, (v, n, expected)) in cases.into_iter().enumerate() {
        let mut res: Vec<Vec<u32>> = vec![];
        v.par_iter()
            .chunks(n)
            .map(|v| v.into_iter().cloned().collect())
            .collect_into_vec(&mut res);
        assert_eq!(expected, res, "Case {i} failed");

        res.truncate(0);
        v.into_par_iter().chunks(n).rev().collect_into_vec(&mut res);
        assert_eq!(
            expected.into_iter().rev().collect::<Vec<Vec<u32>>>(),
            res,
            "Case {i} reversed failed"
        );
    }
}

#[test]
#[ignore] // it's quick enough on optimized 32-bit platforms, but otherwise... ... ...
#[should_panic(expected = "overflow")]
#[cfg(debug_assertions)]
fn check_repeat_unbounded() {
    // use just one thread, so we don't get infinite adaptive splitting
    // (forever stealing and re-splitting jobs that will panic on overflow)
    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    pool.install(|| {
        println!("counted {} repeats", repeat(()).count());
    });
}

#[test]
fn check_repeat_find_any() {
    let even = repeat(4).find_any(|&x| x % 2 == 0);
    assert_eq!(even, Some(4));
}

#[test]
fn check_repeat_take() {
    let v: Vec<_> = repeat(4).take(4).collect();
    assert_eq!(v, [4, 4, 4, 4]);
}

#[test]
fn check_repeat_zip() {
    let v = vec![4, 4, 4, 4];
    let mut fours: Vec<_> = repeat(4).zip(v).collect();
    assert_eq!(fours.len(), 4);
    while let Some(item) = fours.pop() {
        assert_eq!(item, (4, 4));
    }
}

#[test]
fn check_repeat_n_zip_left() {
    let v = vec![4, 4, 4, 4];
    let mut fours: Vec<_> = repeat_n(4, usize::MAX).zip(v).collect();
    assert_eq!(fours.len(), 4);
    while let Some(item) = fours.pop() {
        assert_eq!(item, (4, 4));
    }
}

#[test]
fn check_repeat_n_zip_right() {
    let v = vec![4, 4, 4, 4];
    let mut fours: Vec<_> = v.into_par_iter().zip(repeat_n(4, usize::MAX)).collect();
    assert_eq!(fours.len(), 4);
    while let Some(item) = fours.pop() {
        assert_eq!(item, (4, 4));
    }
}

#[test]
fn count_repeat_n_clones() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CLONES: AtomicUsize = AtomicUsize::new(0);
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    struct Counter;

    impl Clone for Counter {
        fn clone(&self) -> Self {
            CLONES.fetch_add(1, Ordering::Relaxed);
            Counter
        }
    }

    impl Drop for Counter {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[track_caller]
    fn check(clones: usize, drops: usize) {
        assert_eq!(CLONES.swap(0, Ordering::Relaxed), clones, "clones");
        assert_eq!(DROPS.swap(0, Ordering::Relaxed), drops, "drops");
    }

    drop(repeat_n(Counter, 100));
    check(0, 1);

    let empty = repeat_n(Counter, 0);
    check(0, 1);
    let empty2 = empty.clone();
    check(0, 0);
    assert_eq!(empty.count(), 0);
    assert_eq!(empty2.count(), 0);
    check(0, 0);

    let par_iter = repeat_n(Counter, 100);
    let par_iter2 = par_iter.clone();
    check(1, 0);
    assert_eq!(par_iter.count(), 100);
    check(99, 100);
    assert_eq!(par_iter2.map(std::mem::forget).count(), 100);
    check(99, 0);

    // Clone once in `split_at` and again for the first item, leaving its unused tail.
    // The other split doesn't have a tail, so it can avoid a clone.
    let step99 = repeat_n(Counter, 100).step_by(99);
    assert_eq!(step99.count(), 2);
    check(2, 3);

    // Same without any parallel splitting
    let step99 = repeat_n(Counter, 100).step_by(99).with_min_len(2);
    assert_eq!(step99.count(), 2);
    check(1, 2);

    // Clone once in `split_at` and again for both items, leaving both unused tails.
    let step50 = repeat_n(Counter, 100).step_by(50);
    assert_eq!(step50.count(), 2);
    check(3, 4);
}

#[test]
fn check_empty() {
    // drive_unindexed
    let mut v: Vec<i32> = empty().filter(|_| unreachable!()).collect();
    assert!(v.is_empty());

    // drive (indexed)
    empty().collect_into_vec(&mut v);
    assert!(v.is_empty());

    // with_producer
    let v: Vec<(i32, i32)> = empty().zip(1..10).collect();
    assert!(v.is_empty());
}

#[test]
fn check_once() {
    // drive_unindexed
    let mut v: Vec<i32> = once(42).filter(|_| true).collect();
    assert_eq!(v, &[42]);

    // drive (indexed)
    once(42).collect_into_vec(&mut v);
    assert_eq!(v, &[42]);

    // with_producer
    let v: Vec<(i32, i32)> = once(42).zip(1..10).collect();
    assert_eq!(v, &[(42, 1)]);
}

#[test]
fn check_update() {
    let mut v: Vec<Vec<_>> = vec![vec![1], vec![3, 2, 1]];
    v.par_iter_mut().update(|v| v.push(0)).for_each(|_| ());

    assert_eq!(v, vec![vec![1, 0], vec![3, 2, 1, 0]]);
}

#[test]
fn walk_tree_prefix() {
    let v: Vec<u32> = crate::iter::walk_tree_prefix(0u32..100, |r| {
        // root is smallest
        let mid = (r.start + 1 + r.end) / 2;
        // small indices to the left, large to the right
        std::iter::once((r.start + 1)..mid)
            .chain(std::iter::once(mid..r.end))
            .filter(|r| !r.is_empty())
    })
    .map(|r| r.start)
    .collect();
    assert!(v.into_iter().eq(0..100));
}

#[test]
fn walk_tree_postfix() {
    let v: Vec<_> = crate::iter::walk_tree_postfix(0u64..100, |r| {
        // root is largest
        let mid = (r.start + r.end - 1) / 2;
        // small indices to the left, large to the right
        std::iter::once(r.start..mid)
            .chain(std::iter::once(mid..(r.end - 1)))
            .filter(|r| !r.is_empty())
    })
    .map(|r| r.end - 1)
    .collect();
    assert!(v.into_iter().eq(0..100));
}

#[test]
fn walk_flat_tree_prefix() {
    let v: Vec<_> =
        crate::iter::walk_tree_prefix(0, |&e| if e < 99 { Some(e + 1) } else { None }).collect();
    assert!(v.into_iter().eq(0..100));
}

#[test]
fn walk_flat_tree_postfix() {
    let v: Vec<_> =
        crate::iter::walk_tree_postfix(99, |&e| if e > 0 { Some(e - 1) } else { None }).collect();
    assert!(v.into_iter().eq(0..100));
}

#[test]
fn walk_tree_prefix_degree5() {
    let depth = 5;
    let nodes_number = (1 - 5i32.pow(depth)) / (1 - 5);
    let nodes = (0..nodes_number).collect::<Vec<_>>();
    let v: Vec<i32> = crate::iter::walk_tree_prefix(nodes.as_slice(), |&r| {
        r.split_first()
            .into_iter()
            .filter_map(|(_, r)| if r.is_empty() { None } else { Some(r) })
            .flat_map(|r| r.chunks(r.len() / 5))
    })
    .filter_map(|r| r.first().copied())
    .collect();
    assert_eq!(v, nodes);
}

#[test]
fn walk_tree_postfix_degree5() {
    let depth = 5;
    let nodes_number = (1 - 5i32.pow(depth)) / (1 - 5);
    let nodes = (0..nodes_number).collect::<Vec<_>>();
    let v: Vec<i32> = crate::iter::walk_tree_postfix(nodes.as_slice(), |&r| {
        r.split_last()
            .into_iter()
            .filter_map(|(_, r)| if r.is_empty() { None } else { Some(r) })
            .flat_map(|r| r.chunks(r.len() / 5))
    })
    .filter_map(|r| r.last().copied())
    .collect();
    assert_eq!(v, nodes)
}

#[test]
fn blocks() {
    let count = AtomicUsize::new(0);
    let v: Vec<usize> = (0..1000)
        .into_par_iter()
        .map(|_| count.fetch_add(1, Ordering::Relaxed))
        .by_uniform_blocks(100)
        .collect();
    let m = v
        .chunks(100)
        .map(|c| c.iter().max().copied().unwrap())
        .collect::<Vec<usize>>();
    assert!(m.windows(2).all(|w| w[0].lt(&w[1])));
    assert_eq!(v.len(), 1000);
}
