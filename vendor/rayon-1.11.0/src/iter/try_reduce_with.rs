use super::plumbing::*;
use super::ParallelIterator;
use super::Try;

use std::ops::ControlFlow::{self, Break, Continue};
use std::sync::atomic::{AtomicBool, Ordering};

pub(super) fn try_reduce_with<PI, R, T>(pi: PI, reduce_op: R) -> Option<T>
where
    PI: ParallelIterator<Item = T>,
    R: Fn(T::Output, T::Output) -> T + Sync,
    T: Try + Send,
{
    let full = AtomicBool::new(false);
    let consumer = TryReduceWithConsumer {
        reduce_op: &reduce_op,
        full: &full,
    };
    pi.drive_unindexed(consumer)
}

struct TryReduceWithConsumer<'r, R> {
    reduce_op: &'r R,
    full: &'r AtomicBool,
}

impl<'r, R> Copy for TryReduceWithConsumer<'r, R> {}

impl<'r, R> Clone for TryReduceWithConsumer<'r, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'r, R, T> Consumer<T> for TryReduceWithConsumer<'r, R>
where
    R: Fn(T::Output, T::Output) -> T + Sync,
    T: Try + Send,
{
    type Folder = TryReduceWithFolder<'r, R, T>;
    type Reducer = Self;
    type Result = Option<T>;

    fn split_at(self, _index: usize) -> (Self, Self, Self) {
        (self, self, self)
    }

    fn into_folder(self) -> Self::Folder {
        TryReduceWithFolder {
            reduce_op: self.reduce_op,
            opt_control: None,
            full: self.full,
        }
    }

    fn full(&self) -> bool {
        self.full.load(Ordering::Relaxed)
    }
}

impl<'r, R, T> UnindexedConsumer<T> for TryReduceWithConsumer<'r, R>
where
    R: Fn(T::Output, T::Output) -> T + Sync,
    T: Try + Send,
{
    fn split_off_left(&self) -> Self {
        *self
    }

    fn to_reducer(&self) -> Self::Reducer {
        *self
    }
}

impl<'r, R, T> Reducer<Option<T>> for TryReduceWithConsumer<'r, R>
where
    R: Fn(T::Output, T::Output) -> T + Sync,
    T: Try,
{
    fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T> {
        let reduce_op = self.reduce_op;
        match (left, right) {
            (Some(left), Some(right)) => match (left.branch(), right.branch()) {
                (Continue(left), Continue(right)) => Some(reduce_op(left, right)),
                (Break(r), _) | (_, Break(r)) => Some(T::from_residual(r)),
            },
            (None, x) | (x, None) => x,
        }
    }
}

struct TryReduceWithFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    opt_control: Option<ControlFlow<T::Residual, T::Output>>,
    full: &'r AtomicBool,
}

impl<'r, R, T> Folder<T> for TryReduceWithFolder<'r, R, T>
where
    R: Fn(T::Output, T::Output) -> T,
    T: Try,
{
    type Result = Option<T>;

    fn consume(mut self, item: T) -> Self {
        let reduce_op = self.reduce_op;
        let control = match (self.opt_control, item.branch()) {
            (Some(Continue(left)), Continue(right)) => reduce_op(left, right).branch(),
            (Some(control @ Break(_)), _) | (_, control) => control,
        };
        if let Break(_) = control {
            self.full.store(true, Ordering::Relaxed)
        }
        self.opt_control = Some(control);
        self
    }

    fn complete(self) -> Option<T> {
        match self.opt_control {
            Some(Continue(c)) => Some(T::from_output(c)),
            Some(Break(r)) => Some(T::from_residual(r)),
            None => None,
        }
    }

    fn full(&self) -> bool {
        match self.opt_control {
            Some(Break(_)) => true,
            _ => self.full.load(Ordering::Relaxed),
        }
    }
}
