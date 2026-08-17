use once_cell::sync::Lazy;
use tokio_crate as tokio;

pub(crate) fn enter() -> tokio::runtime::EnterGuard<'static> {
    RUNTIME.enter()
}

static RUNTIME: Lazy<tokio::runtime::Handle> = Lazy::new(|| {
    tokio::runtime::Handle::try_current().unwrap_or_else(|_| {
        let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
        let handle = rt.handle().clone();
        std::thread::Builder::new()
            .name("async-global-executor/tokio".to_string())
            .spawn(move || {
                rt.block_on(futures_lite::future::pending::<()>());
            })
            .expect("failed to spawn tokio driver thread");
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
