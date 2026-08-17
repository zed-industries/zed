use wasm_bindgen::JsCast;
use web_sys::{AbortController, AbortSignal};

mod body;
mod client;
/// TODO
#[cfg(feature = "multipart")]
pub mod multipart;
mod request;
mod response;

pub use self::body::Body;
pub use self::client::{Client, ClientBuilder};
pub use self::request::{Request, RequestBuilder};
pub use self::response::Response;

async fn promise<T>(promise: js_sys::Promise) -> Result<T, crate::error::BoxError>
where
    T: JsCast,
{
    use wasm_bindgen_futures::JsFuture;

    let js_val = JsFuture::from(promise).await.map_err(crate::error::wasm)?;

    js_val
        .dyn_into::<T>()
        .map_err(|_js_val| "promise resolved to unexpected type".into())
}

/// A guard that cancels a fetch request when dropped.
struct AbortGuard {
    ctrl: AbortController,
}

impl AbortGuard {
    fn new() -> crate::Result<Self> {
        Ok(AbortGuard {
            ctrl: AbortController::new()
                .map_err(crate::error::wasm)
                .map_err(crate::error::builder)?,
        })
    }

    fn signal(&self) -> AbortSignal {
        self.ctrl.signal()
    }
}

impl Drop for AbortGuard {
    fn drop(&mut self) {
        self.ctrl.abort();
    }
}
