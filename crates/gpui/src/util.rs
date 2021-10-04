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
