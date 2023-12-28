use smol::future::FutureExt;
use std::{future::Future, time::Duration};

pub fn post_inc(value: &mut usize) -> usize {
    let prev = *value;
    *value += 1;
    prev
}

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
impl<'a> std::fmt::Debug for CwdBacktrace<'a> {
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
                .any(|s| s.filename().map_or(false, |f| f.starts_with(&cwd)))
            {
                formatted_frame.backtrace_frame(frame)?;
            }
        }
        fmt.finish()
    }
}
