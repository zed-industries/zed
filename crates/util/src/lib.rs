pub mod channel;
pub mod paths;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

pub use backtrace::Backtrace;
use futures::Future;
use rand::{seq::SliceRandom, Rng};
use std::{
    cmp::Ordering,
    ops::AddAssign,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug, Default)]
pub struct StaffMode(pub bool);

impl std::ops::Deref for StaffMode {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! debug_panic {
    ( $($fmt_arg:tt)* ) => {
        if cfg!(debug_assertions) {
            panic!( $($fmt_arg)* );
        } else {
            let backtrace = $crate::Backtrace::new();
            log::error!("{}\n{:?}", format_args!($($fmt_arg)*), backtrace);
        }
    };
}

pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn truncate_and_trailoff(s: &str, max_chars: usize) -> String {
    debug_assert!(max_chars >= 5);

    let truncation_ix = s.char_indices().map(|(i, _)| i).nth(max_chars);
    match truncation_ix {
        Some(length) => s[..length].to_string() + "…",
        None => s.to_string(),
    }
}

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

/// Extend a sorted vector with a sorted sequence of items, maintaining the vector's sort order and
/// enforcing a maximum length. Sort the items according to the given callback. Before calling this,
/// both `vec` and `new_items` should already be sorted according to the `cmp` comparator.
pub fn extend_sorted<T, I, F>(vec: &mut Vec<T>, new_items: I, limit: usize, mut cmp: F)
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut start_index = 0;
    for new_item in new_items {
        if let Err(i) = vec[start_index..].binary_search_by(|m| cmp(m, &new_item)) {
            let index = start_index + i;
            if vec.len() < limit {
                vec.insert(index, new_item);
            } else if index < vec.len() {
                vec.pop();
                vec.insert(index, new_item);
            }
            start_index = index;
        }
    }
}

pub trait ResultExt {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    fn warn_on_err(self) -> Option<Self::Ok>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log::error!("{:?}", error);
                None
            }
        }
    }

    fn warn_on_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log::warn!("{:?}", error);
                None
            }
        }
    }
}

pub trait TryFutureExt {
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
}

impl<F, T> TryFutureExt for F
where
    F: Future<Output = anyhow::Result<T>>,
{
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, log::Level::Error)
    }

    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, log::Level::Warn)
    }
}

pub struct LogErrorFuture<F>(F, log::Level);

impl<F, T> Future for LogErrorFuture<F>
where
    F: Future<Output = anyhow::Result<T>>,
{
    type Output = Option<T>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let level = self.1;
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(output) => Poll::Ready(match output {
                Ok(output) => Some(output),
                Err(error) => {
                    log::log!(level, "{:?}", error);
                    None
                }
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

pub fn defer<F: FnOnce()>(f: F) -> impl Drop {
    Defer(Some(f))
}

pub struct RandomCharIter<T: Rng>(T);

impl<T: Rng> RandomCharIter<T> {
    pub fn new(rng: T) -> Self {
        Self(rng)
    }
}

impl<T: Rng> Iterator for RandomCharIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if std::env::var("SIMPLE_TEXT").map_or(false, |v| !v.is_empty()) {
            return if self.0.gen_range(0..100) < 5 {
                Some('\n')
            } else {
                Some(self.0.gen_range(b'a'..b'z' + 1).into())
            };
        }

        match self.0.gen_range(0..100) {
            // whitespace
            0..=19 => [' ', '\n', '\r', '\t'].choose(&mut self.0).copied(),
            // two-byte greek letters
            20..=32 => char::from_u32(self.0.gen_range(('α' as u32)..('ω' as u32 + 1))),
            // // three-byte characters
            33..=45 => ['✋', '✅', '❌', '❎', '⭐'].choose(&mut self.0).copied(),
            // // four-byte characters
            46..=58 => ['🍐', '🏀', '🍗', '🎉'].choose(&mut self.0).copied(),
            // ascii letters
            _ => Some(self.0.gen_range(b'a'..b'z' + 1).into()),
        }
    }
}

// copy unstable standard feature option unzip
// https://github.com/rust-lang/rust/issues/87800
// Remove when this ship in Rust 1.66 or 1.67
pub fn unzip_option<T, U>(option: Option<(T, U)>) -> (Option<T>, Option<U>) {
    match option {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    }
}

/// Immediately invoked function expression. Good for using the ? operator
/// in functions which do not return an Option or Result
#[macro_export]
macro_rules! iife {
    ($block:block) => {
        (|| $block)()
    };
}

/// Async lImmediately invoked function expression. Good for using the ? operator
/// in functions which do not return an Option or Result. Async version of above
#[macro_export]
macro_rules! async_iife {
    ($block:block) => {
        (|| async move { $block })()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_sorted() {
        let mut vec = vec![];

        extend_sorted(&mut vec, vec![21, 17, 13, 8, 1, 0], 5, |a, b| b.cmp(a));
        assert_eq!(vec, &[21, 17, 13, 8, 1]);

        extend_sorted(&mut vec, vec![101, 19, 17, 8, 2], 8, |a, b| b.cmp(a));
        assert_eq!(vec, &[101, 21, 19, 17, 13, 8, 2, 1]);

        extend_sorted(&mut vec, vec![1000, 19, 17, 9, 5], 8, |a, b| b.cmp(a));
        assert_eq!(vec, &[1000, 101, 21, 19, 17, 13, 9, 8]);
    }

    #[test]
    fn test_iife() {
        fn option_returning_function() -> Option<()> {
            None
        }

        let foo = iife!({
            option_returning_function()?;
            Some(())
        });

        assert_eq!(foo, None);
    }

    #[test]
    fn test_trancate_and_trailoff() {
        assert_eq!(truncate_and_trailoff("", 5), "");
        assert_eq!(truncate_and_trailoff("èèèèèè", 7), "èèèèèè");
        assert_eq!(truncate_and_trailoff("èèèèèè", 6), "èèèèèè");
        assert_eq!(truncate_and_trailoff("èèèèèè", 5), "èèèèè…");
    }
}
