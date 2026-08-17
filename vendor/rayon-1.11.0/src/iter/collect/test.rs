#![cfg(test)]
#![allow(unused_assignments)]

// These tests are primarily targeting "abusive" producers that will
// try to drive the "collect consumer" incorrectly. These should
// result in panics.

use super::collect_with_consumer;
use crate::iter::plumbing::*;
use rayon_core::join;

use std::fmt;
use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::Result as ThreadResult;

/// Promises to produce 2 items, but then produces 3.  Does not do any
/// splits at all.
#[test]
#[should_panic(expected = "too many values")]
fn produce_too_many_items() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 2, |consumer| {
        let mut folder = consumer.into_folder();
        folder = folder.consume(22);
        folder = folder.consume(23);
        folder = folder.consume(24);
        unreachable!("folder does not complete")
    });
}

/// Produces fewer items than promised. Does not do any
/// splits at all.
#[test]
#[should_panic(expected = "expected 5 total writes, but got 2")]
fn produce_fewer_items() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 5, |consumer| {
        let mut folder = consumer.into_folder();
        folder = folder.consume(22);
        folder = folder.consume(23);
        folder.complete()
    });
}

// Complete is not called by the consumer. Hence,the collection vector is not fully initialized.
#[test]
#[should_panic(expected = "expected 4 total writes, but got 2")]
fn left_produces_items_with_no_complete() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2).consume(3);
        right_folder.complete()
    });
}

// Complete is not called by the right consumer. Hence,the
// collection vector is not fully initialized.
#[test]
#[should_panic(expected = "expected 4 total writes, but got 2")]
fn right_produces_items_with_no_complete() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2).consume(3);
        left_folder.complete()
    });
}

// Complete is not called by the consumer. Hence,the collection vector is not fully initialized.
#[test]
#[cfg_attr(not(panic = "unwind"), ignore)]
fn produces_items_with_no_complete() {
    let counter = DropCounter::default();
    let mut v = vec![];
    let panic_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        collect_with_consumer(&mut v, 2, |consumer| {
            let mut folder = consumer.into_folder();
            folder = folder.consume(counter.element());
            folder = folder.consume(counter.element());
            panic!("folder does not complete");
        });
    }));
    assert!(v.is_empty());
    assert_is_panic_with_message(&panic_result, "folder does not complete");
    counter.assert_drop_count();
}

// The left consumer produces too many items while the right
// consumer produces correct number.
#[test]
#[should_panic(expected = "too many values")]
fn left_produces_too_many_items() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1).consume(2);
        right_folder = right_folder.consume(2).consume(3);
        let _ = right_folder.complete();
        unreachable!("folder does not complete");
    });
}

// The right consumer produces too many items while the left
// consumer produces correct number.
#[test]
#[should_panic(expected = "too many values")]
fn right_produces_too_many_items() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2).consume(3).consume(4);
        let _ = left_folder.complete();
        unreachable!("folder does not complete");
    });
}

// The left consumer produces fewer items while the right
// consumer produces correct number.
#[test]
#[should_panic(expected = "expected 4 total writes, but got 1")]
fn left_produces_fewer_items() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let reducer = consumer.to_reducer();
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0);
        right_folder = right_folder.consume(2).consume(3);
        let left_result = left_folder.complete();
        let right_result = right_folder.complete();
        reducer.reduce(left_result, right_result)
    });
}

// The left and right consumer produce the correct number but
// only left result is returned
#[test]
#[should_panic(expected = "expected 4 total writes, but got 2")]
fn only_left_result() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2).consume(3);
        let left_result = left_folder.complete();
        let _ = right_folder.complete();
        left_result
    });
}

// The left and right consumer produce the correct number but
// only right result is returned
#[test]
#[should_panic(expected = "expected 4 total writes, but got 2")]
fn only_right_result() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2).consume(3);
        let _ = left_folder.complete();
        right_folder.complete()
    });
}

// The left and right consumer produce the correct number but reduce
// in the wrong order.
#[test]
#[should_panic(expected = "expected 4 total writes, but got 2")]
fn reducer_does_not_preserve_order() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let reducer = consumer.to_reducer();
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2).consume(3);
        let left_result = left_folder.complete();
        let right_result = right_folder.complete();
        reducer.reduce(right_result, left_result)
    });
}

// The right consumer produces fewer items while the left
// consumer produces correct number.
#[test]
#[should_panic(expected = "expected 4 total writes, but got 3")]
fn right_produces_fewer_items() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let reducer = consumer.to_reducer();
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let mut left_folder = left_consumer.into_folder();
        let mut right_folder = right_consumer.into_folder();
        left_folder = left_folder.consume(0).consume(1);
        right_folder = right_folder.consume(2);
        let left_result = left_folder.complete();
        let right_result = right_folder.complete();
        reducer.reduce(left_result, right_result)
    });
}

// The left consumer panics and the right stops short, like `panic_fuse()`.
// We should get the left panic without finishing `collect_with_consumer`.
#[test]
#[should_panic(expected = "left consumer panic")]
fn left_panics() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let reducer = consumer.to_reducer();
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let (left_result, right_result) = join(
            || {
                let mut left_folder = left_consumer.into_folder();
                left_folder = left_folder.consume(0);
                panic!("left consumer panic");
            },
            || {
                let mut right_folder = right_consumer.into_folder();
                right_folder = right_folder.consume(2);
                right_folder.complete() // early return
            },
        );
        reducer.reduce(left_result, right_result)
    });
    unreachable!();
}

// The right consumer panics and the left stops short, like `panic_fuse()`.
// We should get the right panic without finishing `collect_with_consumer`.
#[test]
#[should_panic(expected = "right consumer panic")]
fn right_panics() {
    let mut v = vec![];
    collect_with_consumer(&mut v, 4, |consumer| {
        let reducer = consumer.to_reducer();
        let (left_consumer, right_consumer, _) = consumer.split_at(2);
        let (left_result, right_result) = join(
            || {
                let mut left_folder = left_consumer.into_folder();
                left_folder = left_folder.consume(0);
                left_folder.complete() // early return
            },
            || {
                let mut right_folder = right_consumer.into_folder();
                right_folder = right_folder.consume(2);
                panic!("right consumer panic");
            },
        );
        reducer.reduce(left_result, right_result)
    });
    unreachable!();
}

// The left consumer produces fewer items while the right
// consumer produces correct number; check that created elements are dropped
#[test]
#[cfg_attr(not(panic = "unwind"), ignore)]
fn left_produces_fewer_items_drops() {
    let counter = DropCounter::default();
    let mut v = vec![];
    let panic_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        collect_with_consumer(&mut v, 4, |consumer| {
            let reducer = consumer.to_reducer();
            let (left_consumer, right_consumer, _) = consumer.split_at(2);
            let mut left_folder = left_consumer.into_folder();
            let mut right_folder = right_consumer.into_folder();
            left_folder = left_folder.consume(counter.element());
            right_folder = right_folder
                .consume(counter.element())
                .consume(counter.element());
            let left_result = left_folder.complete();
            let right_result = right_folder.complete();
            reducer.reduce(left_result, right_result)
        });
    }));
    assert!(v.is_empty());
    assert_is_panic_with_message(&panic_result, "expected 4 total writes, but got 1");
    counter.assert_drop_count();
}

/// This counter can create elements, and then count and verify
/// the number of which have actually been dropped again.
#[derive(Default)]
struct DropCounter {
    created: AtomicUsize,
    dropped: AtomicUsize,
}

struct Element<'a>(&'a AtomicUsize);

impl DropCounter {
    fn created(&self) -> usize {
        self.created.load(Ordering::SeqCst)
    }

    fn dropped(&self) -> usize {
        self.dropped.load(Ordering::SeqCst)
    }

    fn element(&self) -> Element<'_> {
        self.created.fetch_add(1, Ordering::SeqCst);
        Element(&self.dropped)
    }

    fn assert_drop_count(&self) {
        assert_eq!(
            self.created(),
            self.dropped(),
            "Expected {} dropped elements, but found {}",
            self.created(),
            self.dropped()
        );
    }
}

impl<'a> Drop for Element<'a> {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Assert that the result from catch_unwind is a panic that contains expected message
fn assert_is_panic_with_message<T>(result: &ThreadResult<T>, expected: &str)
where
    T: fmt::Debug,
{
    match result {
        Ok(value) => {
            panic!("assertion failure: Expected panic, got successful {value:?}");
        }
        Err(error) => {
            let message_str = error.downcast_ref::<&'static str>().cloned();
            let message_string = error.downcast_ref::<String>().map(String::as_str);
            if let Some(message) = message_str.or(message_string) {
                if !message.contains(expected) {
                    panic!(
                        "assertion failure: Expected {expected:?}, but found panic with {message:?}"
                    );
                }
            // assertion passes
            } else {
                panic!(
                    "assertion failure: Expected {expected:?}, but found panic with unknown value"
                );
            }
        }
    }
}
