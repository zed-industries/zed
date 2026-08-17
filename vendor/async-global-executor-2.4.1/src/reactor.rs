pub(crate) fn block_on<F: std::future::Future<Output = T>, T>(future: F) -> T {
    #[cfg(feature = "async-io")]
    let run = || async_io::block_on(future);
    #[cfg(not(feature = "async-io"))]
    let run = || futures_lite::future::block_on(future);
    #[cfg(feature = "tokio")]
    let _tokio_enter = crate::tokio::enter();
    #[cfg(feature = "tokio02")]
    let run = || crate::tokio02::enter(run);
    #[cfg(feature = "tokio03")]
    let _tokio03_enter = crate::tokio03::enter();
    run()
}
