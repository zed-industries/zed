// Helper functions to be runtime agnostic

#[cfg(feature = "tokio-runtime")]
extern crate tokio;
#[allow(unused_imports)]
#[cfg(feature = "tokio-runtime")]
pub use tokio::{main, test};

#[cfg(feature = "async-std-runtime")]
extern crate async_std;
#[allow(unused_imports)]
#[cfg(feature = "async-std-runtime")]
pub use async_std::{main, test};

#[allow(unused)]
#[cfg(feature = "tokio-runtime")]
pub async fn sleep(duration: std::time::Duration) {
    tokio::time::sleep(duration).await;
}
#[allow(unused)]
#[cfg(feature = "async-std-runtime")]
pub async fn sleep(duration: std::time::Duration) {
    async_std::task::sleep(duration).await;
}

#[allow(unused_imports)]
#[cfg(feature = "async-dispatcher-runtime")]
pub use async_dispatcher::{main, test};

#[allow(unused)]
#[cfg(feature = "async-dispatcher-runtime")]
pub use async_dispatcher::sleep;
