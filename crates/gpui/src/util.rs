#[cfg(any(test, feature = "test-support"))]
use std::time::Duration;

#[cfg(any(test, feature = "test-support"))]
use futures::Future;

#[cfg(any(test, feature = "test-support"))]
use smol::future::FutureExt;

pub use util::*;

/// A helper trait for building complex objects with imperative conditionals in a fluent style.
pub trait FluentBuilder {
    /// Imperatively modify self with the given closure.
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
    {
        f(self)
    }

    /// Conditionally modify self with the given closure.
    fn when(self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { this })
    }

    /// Conditionally unwrap and modify self with the given closure, if the given option is Some.
    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }
    /// Conditionally unwrap and modify self with the given closure, if the given option is None.
    fn when_none<T>(self, option: &Option<T>, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(_) = option {
                this
            } else {
                then(this)
            }
        })
    }
}

#[cfg(any(test, feature = "test-support"))]
pub async fn timeout<F, T>(timeout: Duration, f: F) -> Result<T, ()>
where
    F: Future<Output = T>,
{
    let timer = async {
        smol::Timer::after(timeout).await;
        Err(())
    };
    let future = async move { Ok(f.await) };
    timer.race(future).await
}

#[cfg(any(test, feature = "test-support"))]
pub struct CwdBacktrace<'a>(pub &'a backtrace::Backtrace);

#[cfg(any(test, feature = "test-support"))]
impl std::fmt::Debug for CwdBacktrace<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use backtrace::{BacktraceFmt, BytesOrWideString};

        let cwd = std::env::current_dir().unwrap();
        let cwd = cwd.parent().unwrap();
        let mut print_path = |fmt: &mut std::fmt::Formatter<'_>, path: BytesOrWideString<'_>| {
            std::fmt::Display::fmt(&path, fmt)
        };
        let mut fmt = BacktraceFmt::new(f, backtrace::PrintFmt::Full, &mut print_path);
        for frame in self.0.frames() {
            let mut formatted_frame = fmt.frame();
            if frame
                .symbols()
                .iter()
                .any(|s| s.filename().map_or(false, |f| f.starts_with(cwd)))
            {
                formatted_frame.backtrace_frame(frame)?;
            }
        }
        fmt.finish()
    }
}
