pub mod arc_cow;
pub mod fs;
pub mod github;
pub mod http;
pub mod paths;
mod semantic_version;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

pub use backtrace::Backtrace;
use futures::Future;
use lazy_static::lazy_static;
use rand::{seq::SliceRandom, Rng};
pub use semantic_version::SemanticVersion;
use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    env,
    ops::{AddAssign, Range, RangeInclusive},
    panic::Location,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use unicase::UniCase;

pub use take_until::*;

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

pub fn http_proxy_from_env() -> Option<isahc::http::Uri> {
    macro_rules! try_env {
        ($($env:literal),+) => {
            $(
                if let Ok(env) = std::env::var($env) {
                    return env.parse::<isahc::http::Uri>().ok();
                }
            )+
        };
    }

    try_env!(
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy"
    );
    None
}

/// Removes characters from the end of the string if its length is greater than `max_chars` and
/// appends "..." to the string. Returns string unchanged if its length is smaller than max_chars.
pub fn truncate_and_trailoff(s: &str, max_chars: usize) -> String {
    debug_assert!(max_chars >= 5);

    let truncation_ix = s.char_indices().map(|(i, _)| i).nth(max_chars);
    match truncation_ix {
        Some(length) => s[..length].to_string() + "…",
        None => s.to_string(),
    }
}

/// Removes characters from the front of the string if its length is greater than `max_chars` and
/// prepends the string with "...". Returns string unchanged if its length is smaller than max_chars.
pub fn truncate_and_remove_front(s: &str, max_chars: usize) -> String {
    debug_assert!(max_chars >= 5);

    let truncation_ix = s.char_indices().map(|(i, _)| i).nth_back(max_chars);
    match truncation_ix {
        Some(length) => "…".to_string() + &s[length..],
        None => s.to_string(),
    }
}

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

/// Extend a sorted vector with a sorted sequence of items, maintaining the vector's sort order and
/// enforcing a maximum length. This also de-duplicates items. Sort the items according to the given callback. Before calling this,
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

pub fn merge_json_value_into(source: serde_json::Value, target: &mut serde_json::Value) {
    use serde_json::Value;

    match (source, target) {
        (Value::Object(source), Value::Object(target)) => {
            for (key, value) in source {
                if let Some(target) = target.get_mut(&key) {
                    merge_json_value_into(value, target);
                } else {
                    target.insert(key.clone(), value);
                }
            }
        }

        (source, target) => *target = source,
    }
}

pub fn merge_non_null_json_value_into(source: serde_json::Value, target: &mut serde_json::Value) {
    use serde_json::Value;
    if let Value::Object(source_object) = source {
        let target_object = if let Value::Object(target) = target {
            target
        } else {
            *target = Value::Object(Default::default());
            target.as_object_mut().unwrap()
        };
        for (key, value) in source_object {
            if let Some(target) = target_object.get_mut(&key) {
                merge_non_null_json_value_into(value, target);
            } else if !value.is_null() {
                target_object.insert(key.clone(), value);
            }
        }
    } else if !source.is_null() {
        *target = source
    }
}

pub fn measure<R>(label: &str, f: impl FnOnce() -> R) -> R {
    lazy_static! {
        pub static ref ZED_MEASUREMENTS: bool = env::var("ZED_MEASUREMENTS")
            .map(|measurements| measurements == "1" || measurements == "true")
            .unwrap_or(false);
    }

    if *ZED_MEASUREMENTS {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        eprintln!("{}: {:?}", label, elapsed);
        result
    } else {
        f()
    }
}

pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    /// Assert that this result should never be an error in development or tests.
    fn debug_assert_ok(self, reason: &str) -> Self;
    fn warn_on_err(self) -> Option<Self::Ok>;
    fn inspect_error(self, func: impl FnOnce(&E)) -> Self;
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    #[track_caller]
    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                let caller = Location::caller();
                log::error!("{}:{}: {:?}", caller.file(), caller.line(), error);
                None
            }
        }
    }

    #[track_caller]
    fn debug_assert_ok(self, reason: &str) -> Self {
        if let Err(error) = &self {
            debug_panic!("{reason} - {error:?}");
        }
        self
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

    /// https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect_err
    fn inspect_error(self, func: impl FnOnce(&E)) -> Self {
        if let Err(err) = &self {
            func(err);
        }

        self
    }
}

pub trait TryFutureExt {
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized;
}

impl<F, T, E> TryFutureExt for F
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    #[track_caller]
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, log::Level::Error, *location)
    }

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, log::Level::Error, location)
    }

    #[track_caller]
    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, log::Level::Warn, *location)
    }

    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized,
    {
        UnwrapFuture(self)
    }
}

#[must_use]
pub struct LogErrorFuture<F>(F, log::Level, core::panic::Location<'static>);

impl<F, T, E> Future for LogErrorFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let level = self.1;
        let location = self.2;
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(output) => Poll::Ready(match output {
                Ok(output) => Some(output),
                Err(error) => {
                    log::log!(
                        level,
                        "{}:{}: {:?}",
                        location.file(),
                        location.line(),
                        error
                    );
                    None
                }
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct UnwrapFuture<F>(F);

impl<F, T, E> Future for UnwrapFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.unwrap()),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Deferred<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Deferred<F> {
    /// Drop without running the deferred function.
    pub fn abort(mut self) {
        self.0.take();
    }
}

impl<F: FnOnce()> Drop for Deferred<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

/// Run the given function when the returned value is dropped (unless it's cancelled).
pub fn defer<F: FnOnce()>(f: F) -> Deferred<F> {
    Deferred(Some(f))
}

pub struct RandomCharIter<T: Rng> {
    rng: T,
    simple_text: bool,
}

impl<T: Rng> RandomCharIter<T> {
    pub fn new(rng: T) -> Self {
        Self {
            rng,
            simple_text: std::env::var("SIMPLE_TEXT").map_or(false, |v| !v.is_empty()),
        }
    }

    pub fn with_simple_text(mut self) -> Self {
        self.simple_text = true;
        self
    }
}

impl<T: Rng> Iterator for RandomCharIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.simple_text {
            return if self.rng.gen_range(0..100) < 5 {
                Some('\n')
            } else {
                Some(self.rng.gen_range(b'a'..b'z' + 1).into())
            };
        }

        match self.rng.gen_range(0..100) {
            // whitespace
            0..=19 => [' ', '\n', '\r', '\t'].choose(&mut self.rng).copied(),
            // two-byte greek letters
            20..=32 => char::from_u32(self.rng.gen_range(('α' as u32)..('ω' as u32 + 1))),
            // // three-byte characters
            33..=45 => ['✋', '✅', '❌', '❎', '⭐']
                .choose(&mut self.rng)
                .copied(),
            // // four-byte characters
            46..=58 => ['🍐', '🏀', '🍗', '🎉'].choose(&mut self.rng).copied(),
            // ascii letters
            _ => Some(self.rng.gen_range(b'a'..b'z' + 1).into()),
        }
    }
}

/// Get an embedded file as a string.
pub fn asset_str<A: rust_embed::RustEmbed>(path: &str) -> Cow<'static, str> {
    match A::get(path).unwrap().data {
        Cow::Borrowed(bytes) => Cow::Borrowed(std::str::from_utf8(bytes).unwrap()),
        Cow::Owned(bytes) => Cow::Owned(String::from_utf8(bytes).unwrap()),
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

/// Evaluates to an immediately invoked function expression. Good for using the ? operator
/// in functions which do not return an Option or Result
#[macro_export]
macro_rules! maybe {
    ($block:block) => {
        (|| $block)()
    };
}

/// Evaluates to an immediately invoked function expression. Good for using the ? operator
/// in functions which do not return an Option or Result, but async.
#[macro_export]
macro_rules! async_maybe {
    ($block:block) => {
        (|| async move { $block })()
    };
}

pub trait RangeExt<T> {
    fn sorted(&self) -> Self;
    fn to_inclusive(&self) -> RangeInclusive<T>;
    fn overlaps(&self, other: &Range<T>) -> bool;
    fn contains_inclusive(&self, other: &Range<T>) -> bool;
}

impl<T: Ord + Clone> RangeExt<T> for Range<T> {
    fn sorted(&self) -> Self {
        cmp::min(&self.start, &self.end).clone()..cmp::max(&self.start, &self.end).clone()
    }

    fn to_inclusive(&self) -> RangeInclusive<T> {
        self.start.clone()..=self.end.clone()
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn contains_inclusive(&self, other: &Range<T>) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

impl<T: Ord + Clone> RangeExt<T> for RangeInclusive<T> {
    fn sorted(&self) -> Self {
        cmp::min(self.start(), self.end()).clone()..=cmp::max(self.start(), self.end()).clone()
    }

    fn to_inclusive(&self) -> RangeInclusive<T> {
        self.clone()
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        self.start() < &other.end && &other.start <= self.end()
    }

    fn contains_inclusive(&self, other: &Range<T>) -> bool {
        self.start() <= &other.start && &other.end <= self.end()
    }
}

/// A way to sort strings with starting numbers numerically first, falling back to alphanumeric one,
/// case-insensitive.
///
/// This is useful for turning regular alphanumerically sorted sequences as `1-abc, 10, 11-def, .., 2, 21-abc`
/// into `1-abc, 2, 10, 11-def, .., 21-abc`
#[derive(Debug, PartialEq, Eq)]
pub struct NumericPrefixWithSuffix<'a>(i32, &'a str);

impl<'a> NumericPrefixWithSuffix<'a> {
    pub fn from_numeric_prefixed_str(str: &'a str) -> Option<Self> {
        let mut chars = str.chars();
        let prefix: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
        let remainder = chars.as_str();

        match prefix.parse::<i32>() {
            Ok(prefix) => Some(NumericPrefixWithSuffix(prefix, remainder)),
            Err(_) => None,
        }
    }
}

impl Ord for NumericPrefixWithSuffix<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let NumericPrefixWithSuffix(num_a, remainder_a) = self;
        let NumericPrefixWithSuffix(num_b, remainder_b) = other;
        num_a
            .cmp(num_b)
            .then_with(|| UniCase::new(remainder_a).cmp(&UniCase::new(remainder_b)))
    }
}

impl<'a> PartialOrd for NumericPrefixWithSuffix<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

        let foo = maybe!({
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

    #[test]
    fn test_numeric_prefix_with_suffix() {
        let mut sorted = vec!["1-abc", "10", "11def", "2", "21-abc"];
        sorted.sort_by_key(|s| {
            NumericPrefixWithSuffix::from_numeric_prefixed_str(s).unwrap_or_else(|| {
                panic!("Cannot convert string `{s}` into NumericPrefixWithSuffix")
            })
        });
        assert_eq!(sorted, ["1-abc", "2", "10", "11def", "21-abc"]);

        for numeric_prefix_less in ["numeric_prefix_less", "aaa", "~™£"] {
            assert_eq!(
                NumericPrefixWithSuffix::from_numeric_prefixed_str(numeric_prefix_less),
                None,
                "String without numeric prefix `{numeric_prefix_less}` should not be converted into NumericPrefixWithSuffix"
            )
        }
    }
}
