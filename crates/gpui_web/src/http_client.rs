use anyhow::anyhow;
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, RedirectPolicy};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use wasm_bindgen::JsCast as _;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    fn global_fetch(input: &web_sys::Request) -> Result<js_sys::Promise, JsValue>;
}

pub struct FetchHttpClient {
    user_agent: Option<http_client::http::header::HeaderValue>,
}

// When the `multithreaded` feature is enabled, safe code must not be able to
// obtain a `FetchHttpClient` without going through the `unsafe` constructors,
// whose contract is what makes the `AssertSend` wrapper in `send` sound. A safe
// `Default` impl would be a hole in that contract, so it is only provided in
// single-threaded builds.
#[cfg(not(feature = "multithreaded"))]
impl Default for FetchHttpClient {
    fn default() -> Self {
        Self { user_agent: None }
    }
}

#[cfg(feature = "multithreaded")]
impl FetchHttpClient {
    /// # Safety
    ///
    /// The caller must ensure that the created `FetchHttpClient` is only used in a single thread environment.
    pub unsafe fn new() -> Self {
        Self { user_agent: None }
    }

    /// # Safety
    ///
    /// The caller must ensure that the created `FetchHttpClient` is only used in a single thread environment.
    pub unsafe fn with_user_agent(user_agent: &str) -> anyhow::Result<Self> {
        Ok(Self {
            user_agent: Some(http_client::http::header::HeaderValue::from_str(
                user_agent,
            )?),
        })
    }
}

#[cfg(not(feature = "multithreaded"))]
impl FetchHttpClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_user_agent(user_agent: &str) -> anyhow::Result<Self> {
        Ok(Self {
            user_agent: Some(http_client::http::header::HeaderValue::from_str(
                user_agent,
            )?),
        })
    }
}

/// Wraps a `!Send` future to satisfy the `Send` bound on `BoxFuture`.
///
/// The wrapped future dereferences `JsValue`s (`web_sys::Request`,
/// `js_sys::Promise`, etc.), which are per-thread JS heap indices and are
/// `!Send` for good reason: touching one from a different `wasm_thread` worker
/// than the one that created it corrupts the JS heap at the boundary.
///
/// This wrapper is therefore only sound when the future is guaranteed never to
/// migrate across workers, i.e. when the owning `FetchHttpClient` is confined
/// to a single thread. Under the default `multithreaded` feature that
/// guarantee can only be established by the caller through the `unsafe`
/// constructors ([`FetchHttpClient::new`] / [`FetchHttpClient::with_user_agent`]),
/// whose contract requires exactly this confinement; there is deliberately no
/// safe `Default`/constructor in that configuration. Without the feature there
/// are no worker threads, so the confinement holds unconditionally.
struct AssertSend<F>(F);

// SAFETY: `AssertSend` is only ever constructed inside `FetchHttpClient::send`.
// A `FetchHttpClient` can only exist under the `multithreaded` feature via the
// `unsafe` constructors, whose safety contract requires the client (and hence
// every future it produces) to stay confined to a single thread. Given that
// confinement the wrapped `JsValue`s are never touched from another worker, so
// asserting `Send` cannot cause cross-worker access.
unsafe impl<F> Send for AssertSend<F> {}

impl<F: Future> Future for AssertSend<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // Safety: pin projection for a single-field newtype wrapper.
        let inner = unsafe { self.map_unchecked_mut(|this| &mut this.0) };
        inner.poll(cx)
    }
}

impl HttpClient for FetchHttpClient {
    fn user_agent(&self) -> Option<&http_client::http::header::HeaderValue> {
        self.user_agent.as_ref()
    }

    fn proxy(&self) -> Option<&http_client::Url> {
        None
    }

    fn send(
        &self,
        req: http_client::http::Request<AsyncBody>,
    ) -> futures::future::BoxFuture<'static, anyhow::Result<http_client::http::Response<AsyncBody>>>
    {
        let (parts, body) = req.into_parts();

        Box::pin(AssertSend(async move {
            let body_bytes = read_body_to_bytes(body).await?;

            let init = web_sys::RequestInit::new();
            init.set_method(parts.method.as_str());

            if let Some(redirect_policy) = parts.extensions.get::<RedirectPolicy>() {
                match redirect_policy {
                    RedirectPolicy::NoFollow => {
                        init.set_redirect(web_sys::RequestRedirect::Manual);
                    }
                    RedirectPolicy::FollowLimit(_) | RedirectPolicy::FollowAll => {
                        init.set_redirect(web_sys::RequestRedirect::Follow);
                    }
                }
            }

            if let Some(ref bytes) = body_bytes {
                let uint8array = js_sys::Uint8Array::from(bytes.as_slice());
                init.set_body(uint8array.as_ref());
            }

            let url = parts.uri.to_string();
            let request = web_sys::Request::new_with_str_and_init(&url, &init)
                .map_err(|error| anyhow!("failed to create fetch Request: {error:?}"))?;

            let request_headers = request.headers();
            for (name, value) in &parts.headers {
                let value_str = value
                    .to_str()
                    .map_err(|_| anyhow!("non-ASCII header value for {name}"))?;
                request_headers
                    .set(name.as_str(), value_str)
                    .map_err(|error| anyhow!("failed to set header {name}: {error:?}"))?;
            }

            let promise = global_fetch(&request)
                .map_err(|error| anyhow!("fetch threw an error: {error:?}"))?;
            let response_value = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|error| anyhow!("fetch failed: {error:?}"))?;

            let web_response: web_sys::Response = response_value
                .dyn_into()
                .map_err(|error| anyhow!("fetch result is not a Response: {error:?}"))?;

            let status = web_response.status();
            let mut builder = http_client::http::Response::builder().status(status);

            // `Headers` is a JS iterable yielding `[name, value]` pairs.
            // `js_sys::Array::from` calls `Array.from()` which accepts any iterable.
            let header_pairs = js_sys::Array::from(&web_response.headers());
            for index in 0..header_pairs.length() {
                match header_pairs.get(index).dyn_into::<js_sys::Array>() {
                    Ok(pair) => match (pair.get(0).as_string(), pair.get(1).as_string()) {
                        (Some(name), Some(value)) => {
                            builder = builder.header(name, value);
                        }
                        (name, value) => {
                            log::warn!(
                                "skipping response header at index {index}: \
                                     name={name:?}, value={value:?}"
                            );
                        }
                    },
                    Err(entry) => {
                        log::warn!("skipping non-array header entry at index {index}: {entry:?}");
                    }
                }
            }

            // The entire response body is eagerly buffered into memory via
            // `arrayBuffer()`. The Fetch API does not expose a synchronous
            // streaming interface; streaming would require `ReadableStream`
            // interop which is significantly more complex.
            let body_promise = web_response
                .array_buffer()
                .map_err(|error| anyhow!("failed to initiate response body read: {error:?}"))?;
            let body_value = wasm_bindgen_futures::JsFuture::from(body_promise)
                .await
                .map_err(|error| anyhow!("failed to read response body: {error:?}"))?;
            let array_buffer: js_sys::ArrayBuffer = body_value
                .dyn_into()
                .map_err(|error| anyhow!("response body is not an ArrayBuffer: {error:?}"))?;
            let response_bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();

            builder
                .body(AsyncBody::from(response_bytes))
                .map_err(|error| anyhow!(error))
        }))
    }
}

async fn read_body_to_bytes(mut body: AsyncBody) -> anyhow::Result<Option<Vec<u8>>> {
    let mut buffer = Vec::new();
    body.read_to_end(&mut buffer).await?;
    if buffer.is_empty() {
        Ok(None)
    } else {
        Ok(Some(buffer))
    }
}
