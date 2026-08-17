#![deny(warnings)]
// This example demonstrates how to delegate the connect calls, which contain TLS handshakes,
// to a secondary tokio runtime of lower OS thread priority using a custom tower layer.
// This helps to ensure that long-running futures during handshake crypto operations don't block other I/O futures.
//
// This does introduce overhead of additional threads, channels, extra vtables, etc,
// so it is best suited to services with large numbers of incoming connections or that
// are otherwise very sensitive to any blocking futures.  Or, you might want fewer threads
// and/or to use the current_thread runtime.
//
// This is using the `tokio` runtime and certain other dependencies:
//
// `tokio = { version = "1", features = ["full"] }`
// `num_cpus = "1.0"`
// `libc = "0"`
// `pin-project-lite = "0.2"`
// `tower = { version = "0.5", default-features = false}`

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    background_threadpool::init_background_runtime();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let client = reqwest::Client::builder()
        .connector_layer(background_threadpool::BackgroundProcessorLayer::new())
        .build()
        .expect("should be able to build reqwest client");

    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        "https://hyper.rs".into()
    };

    eprintln!("Fetching {url:?}...");

    let res = client.get(url).send().await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    Ok(())
}

// separating out for convenience to avoid a million #[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
mod background_threadpool {
    use std::{
        future::Future,
        pin::Pin,
        sync::OnceLock,
        task::{Context, Poll},
    };

    use futures_util::TryFutureExt;
    use pin_project_lite::pin_project;
    use tokio::{runtime::Handle, select, sync::mpsc::error::TrySendError};
    use tower::{BoxError, Layer, Service};

    static CPU_HEAVY_THREAD_POOL: OnceLock<
        tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    > = OnceLock::new();

    pub(crate) fn init_background_runtime() {
        std::thread::Builder::new()
            .name("cpu-heavy-background-threadpool".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .thread_name("cpu-heavy-background-pool-thread")
                    .worker_threads(num_cpus::get() as usize)
                    // ref: https://github.com/tokio-rs/tokio/issues/4941
                    // consider uncommenting if seeing heavy task contention
                    // .disable_lifo_slot()
                    .on_thread_start(move || {
                        #[cfg(target_os = "linux")]
                        unsafe {
                            // Increase thread pool thread niceness, so they are lower priority
                            // than the foreground executor and don't interfere with I/O tasks
                            {
                                *libc::__errno_location() = 0;
                                if libc::nice(10) == -1 && *libc::__errno_location() != 0 {
                                    let error = std::io::Error::last_os_error();
                                    log::error!("failed to set threadpool niceness: {}", error);
                                }
                            }
                        }
                    })
                    .enable_all()
                    .build()
                    .unwrap_or_else(|e| panic!("cpu heavy runtime failed_to_initialize: {}", e));
                rt.block_on(async {
                    log::debug!("starting background cpu-heavy work");
                    process_cpu_work().await;
                });
            })
            .unwrap_or_else(|e| panic!("cpu heavy thread failed_to_initialize: {}", e));
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn process_cpu_work() {
        // we only use this channel for routing work, it should move pretty quick, it can be small
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        // share the handle to the background channel globally
        CPU_HEAVY_THREAD_POOL.set(tx).unwrap();

        while let Some(work) = rx.recv().await {
            tokio::task::spawn(work);
        }
    }

    // retrieve the sender to the background channel, and send the future over to it for execution
    fn send_to_background_runtime(future: impl Future<Output = ()> + Send + 'static) {
        let tx = CPU_HEAVY_THREAD_POOL.get().expect(
            "start up the secondary tokio runtime before sending to `CPU_HEAVY_THREAD_POOL`",
        );

        match tx.try_send(Box::pin(future)) {
            Ok(_) => (),
            Err(TrySendError::Closed(_)) => {
                panic!("background cpu heavy runtime channel is closed")
            }
            Err(TrySendError::Full(msg)) => {
                log::warn!(
                    "background cpu heavy runtime channel is full, task spawning loop delayed"
                );
                let tx = tx.clone();
                Handle::current().spawn(async move {
                    tx.send(msg)
                        .await
                        .expect("background cpu heavy runtime channel is closed")
                });
            }
        }
    }

    // This tower layer injects futures with a oneshot channel, and then sends them to the background runtime for processing.
    // We don't use the Buffer service because that is intended to process sequentially on a single task, whereas we want to
    // spawn a new task per call.
    #[derive(Copy, Clone)]
    pub struct BackgroundProcessorLayer {}
    impl BackgroundProcessorLayer {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl<S> Layer<S> for BackgroundProcessorLayer {
        type Service = BackgroundProcessor<S>;
        fn layer(&self, service: S) -> Self::Service {
            BackgroundProcessor::new(service)
        }
    }

    impl std::fmt::Debug for BackgroundProcessorLayer {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("BackgroundProcessorLayer").finish()
        }
    }

    // This tower service injects futures with a oneshot channel, and then sends them to the background runtime for processing.
    #[derive(Debug, Clone)]
    pub struct BackgroundProcessor<S> {
        inner: S,
    }

    impl<S> BackgroundProcessor<S> {
        pub fn new(inner: S) -> Self {
            BackgroundProcessor { inner }
        }
    }

    impl<S, Request> Service<Request> for BackgroundProcessor<S>
    where
        S: Service<Request>,
        S::Response: Send + 'static,
        S::Error: Into<BoxError> + Send,
        S::Future: Send + 'static,
    {
        type Response = S::Response;

        type Error = BoxError;

        type Future = BackgroundResponseFuture<S::Response>;

        fn poll_ready(
            &mut self,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            match self.inner.poll_ready(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
            }
        }

        fn call(&mut self, req: Request) -> Self::Future {
            let response = self.inner.call(req);

            // wrap our inner service's future with a future that writes to this oneshot channel
            let (mut tx, rx) = tokio::sync::oneshot::channel();
            let future = async move {
                select!(
                    _ = tx.closed() => {
                        // receiver already dropped, don't need to do anything
                    }
                    result = response.map_err(|err| Into::<BoxError>::into(err)) => {
                        // if this fails, the receiver already dropped, so we don't need to do anything
                        let _ = tx.send(result);
                    }
                )
            };
            // send the wrapped future to the background
            send_to_background_runtime(future);

            BackgroundResponseFuture::new(rx)
        }
    }

    // `BackgroundProcessor` response future
    pin_project! {
        #[derive(Debug)]
        pub struct BackgroundResponseFuture<S> {
            #[pin]
            rx: tokio::sync::oneshot::Receiver<Result<S, BoxError>>,
        }
    }

    impl<S> BackgroundResponseFuture<S> {
        pub(crate) fn new(rx: tokio::sync::oneshot::Receiver<Result<S, BoxError>>) -> Self {
            BackgroundResponseFuture { rx }
        }
    }

    impl<S> Future for BackgroundResponseFuture<S>
    where
        S: Send + 'static,
    {
        type Output = Result<S, BoxError>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.project();

            // now poll on the receiver end of the oneshot to get the result
            match this.rx.poll(cx) {
                Poll::Ready(v) => match v {
                    Ok(v) => Poll::Ready(v.map_err(Into::into)),
                    Err(err) => Poll::Ready(Err(Box::new(err) as BoxError)),
                },
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

// The [cfg(not(target_arch = "wasm32"))] above prevent building the tokio::main function
// for wasm32 target, because tokio isn't compatible with wasm32.
// If you aren't building for wasm32, you don't need that line.
// The two lines below avoid the "'main' function not found" error when building for wasm32 target.
#[cfg(any(target_arch = "wasm32"))]
fn main() {}
