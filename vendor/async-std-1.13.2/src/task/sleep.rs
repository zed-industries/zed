use std::time::Duration;

use crate::future;
use crate::io;

/// Sleeps for the specified amount of time.
///
/// This function might sleep for slightly longer than the specified duration but never less.
///
/// This function is an async version of [`std::thread::sleep`].
///
/// [`std::thread::sleep`]: https://doc.rust-lang.org/std/thread/fn.sleep.html
///
/// See also: [`stream::interval`].
///
/// [`stream::interval`]: ../stream/fn.interval.html
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use std::time::Duration;
///
/// use async_std::task;
///
/// task::sleep(Duration::from_secs(1)).await;
/// #
/// # })
/// ```
pub async fn sleep(dur: Duration) {
    let _: io::Result<()> = io::timeout(dur, future::pending()).await;
}
