use crate::WebDispatcher;
use anyhow::{Context as _, anyhow};
use futures::{AsyncReadExt as _, channel::oneshot};
use http_client::{AsyncBody, HttpClient, RedirectPolicy};
use std::sync::Arc;
use wasm_bindgen::JsCast as _;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = "fetch")]
    fn global_fetch(input: &web_sys::Request) -> Result<js_sys::Promise, JsValue>;
}

pub struct FetchHttpClient {
    dispatcher: Arc<WebDispatcher>,
    user_agent: Option<http_client::http::header::HeaderValue>,
    credentials: FetchCredentials,
}

/// Controls whether browser Fetch requests include credentials.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FetchCredentials {
    /// Never send credentials in the request or include credentials in the response.
    Omit,
    /// Only send and include credentials for same-origin requests. This is the default.
    #[default]
    SameOrigin,
    /// Always include credentials, even for cross-origin requests.
    Include,
}

impl FetchHttpClient {
    pub(crate) fn new(dispatcher: Arc<WebDispatcher>) -> Self {
        Self {
            dispatcher,
            user_agent: None,
            credentials: FetchCredentials::default(),
        }
    }

    pub(crate) fn with_user_agent(
        dispatcher: Arc<WebDispatcher>,
        user_agent: &str,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            dispatcher,
            user_agent: Some(http_client::http::header::HeaderValue::from_str(
                user_agent,
            )?),
            credentials: FetchCredentials::default(),
        })
    }

    pub fn with_credentials(mut self, credentials: FetchCredentials) -> Self {
        self.credentials = credentials;
        self
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
        let credentials = self.credentials;
        let dispatcher = self.dispatcher.clone();

        Box::pin(async move {
            let body_bytes = read_body_to_bytes(body).await?;
            let (sender, receiver) = oneshot::channel();

            dispatcher.dispatch_function_on_main_thread(move || {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = fetch(parts, body_bytes, credentials).await;
                    if sender.send(result).is_err() {
                        log::debug!("fetch response receiver was dropped");
                    }
                });
            });

            receiver.await.context("browser fetch task was canceled")?
        })
    }
}

async fn fetch(
    parts: http_client::http::request::Parts,
    body_bytes: Option<Vec<u8>>,
    credentials: FetchCredentials,
) -> anyhow::Result<http_client::http::Response<AsyncBody>> {
    let init = web_sys::RequestInit::new();
    init.set_method(parts.method.as_str());
    init.set_credentials(match credentials {
        FetchCredentials::Omit => web_sys::RequestCredentials::Omit,
        FetchCredentials::SameOrigin => web_sys::RequestCredentials::SameOrigin,
        FetchCredentials::Include => web_sys::RequestCredentials::Include,
    });

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

    let promise =
        global_fetch(&request).map_err(|error| anyhow!("fetch threw an error: {error:?}"))?;
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
