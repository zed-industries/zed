use super::plumbing::*;
use super::ParallelIterator;
use super::Try;

use std::ops::ControlFlow::{self, Break, Continue};
use std::sync::atomic::{AtomicBool, Ordering};

pub(super) fn try_reduce<PI, R, ID, T>(pi: PI, identity: ID, reduce_op: R) -> T
where
    PI: ParallelIterator<Item = T>,
    R: Fn(T::Output, T::Output) -> T + Sync,
    ID: Fn() -> T::Output + Sync,
    T: Try + Send,
{
    let full = AtomicBool::new(false);
    let consumer = TryReduceConsumer {
        identity: &identity,
        reduce_op: &reduce_op,
        full: &full,
    };
    pi.drive_unindexed(consumer)
}

struct TryReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
    full: &'r AtomicBool,
}

impl<'r, R, ID> Copy for TryReduceConsumer<'r, R, ID> {}

impl<'r, R, ID> Clone for TryReduceConsumer<'r, R, ID> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'r, R, ID, T> Consumer<T> for TryReduceConsumer<'r, R, ID>
where
    R: Fn(T::Output, T::Output) -> T + Sync,
    ID: Fn() -> T::Output + Sync,
    T: Try + Send,
{
    type Folder = TryReduceFolder<'r, R, T>;
    type Reducer = Self;
    type Result = T;

    fn split_at(self, _index: usize) -> (Self, Self, Self) {
        (self, self, self)
    }

    fn into_folder(self) -> Self::Folder {
        TryReduceFolder {
            reduce_op: self.reduce_op,
            control: Continue((self.identity)()),
            full: self.full,
        }
    }

    fn full(&self) -> bool {
        self.full.load(Ordering::Relaxed)
    }
}

impl<'r, R, ID, T> UnindexedConsumer<T> for TryReduceConsumer<'r, R, ID>
where
    R: Fn(T::Output, T::Output) -> T + Sync,
    ID: Fn() -> T::Output + Sync,
    T: Try + Send,
{
    fn split_off_left(&self) -> Self {
        *self
    }

    fn to_reducer(&self) -> Self::Reducer {
        *self
    }
}

impl<'r, R, ID, T> Reducer<T> for TryReduceConsumer<'r, R, ID>
where
    R: Fn(T::Output, T::Output) -> T + Sync,
    T: Try,
{
    fn reduce(self, left: T, right: T) -> T {
        match (left.branch(), right.branch()) {
            (Continue(left), Continue(right)) => (self.reduce_op)(left, right),
            (Break(r), _) | (_, Break(r)) => T::from_residual(r),
        }
    }
}

struct TryReduceFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    control: ControlFlow<T::Residual, T::Output>,
    full: &'r AtomicBool,
}

impl<'r, R, T> Folder<T> for TryReduceFolder<'r, R, T>
where
    R: Fn(T::Output, T::Output) -> T,
    T: Try,
{
    type Result = T;

    fn consume(mut self, item: T) -> Self {
        let reduce_op = self.reduce_op;
        self.control = match (self.control, item.branch()) {
            (Continue(left), Continue(right)) => reduce_op(left, right).branch(),
            (control @ Break(_), _) | (_, control @ Break(_)) => control,
        };
        if let Break(_) = self.control {
            self.full.store(true, Ordering::Relaxed);
        }
        self
    }

    fn complete(self) -> T {
        match self.control {
            Continue(c) => T::from_output(c),
            Break(r) => T::from_residual(r),
        }
    }

    fn full(&self) -> bool {
        match self.control {
            Break(_) => true,
            _ => self.full.load(Ordering::Relaxed),
        }
    }
}
