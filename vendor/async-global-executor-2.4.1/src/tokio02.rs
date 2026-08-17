use once_cell::sync::Lazy;
use tokio02_crate as tokio;

pub(crate) fn enter<F: FnOnce() -> R, R>(f: F) -> R {
    RUNTIME.enter(f)
}

static RUNTIME: Lazy<tokio::runtime::Handle> = Lazy::new(|| {
    tokio::runtime::Handle::try_current().unwrap_or_else(|_| {
        let mut rt = tokio::runtime::Runtime::new().expect("failed to build tokio02 runtime");
        let handle = rt.handle().clone();
        std::thread::Builder::new()
            .name("async-global-executor/tokio02".to_string())
            .spawn(move || {
                rt.block_on(futures_lite::future::pending::<()>());
            })
            .expect("failed to spawn tokio02 driver thread");
        handle
    })
});

#[cfg(test)]
mod test {
    use super::*;

    async fn compute() -> u8 {
        tokio::spawn(async { 1 + 2 }).await.unwrap()
    }

    #[test]
    fn spawn_tokio() {
        crate::block_on(async {
            assert_eq!(
                crate::spawn(compute()).await
                    + crate::spawn_local(compute()).await
                    + tokio::spawn(compute()).await.unwrap(),
                9
            );
        });
    }
}
