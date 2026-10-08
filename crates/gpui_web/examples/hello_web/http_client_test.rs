//! Exercises browser Fetch response bodies on the main thread and workers.
//!
//! The browser harness supplies synthetic Fetch streams to control backpressure,
//! cancellation, and errors while using the production HTTP client and dispatcher.

use futures::AsyncReadExt as _;
use gpui::{BackgroundExecutor, Platform as _};
use gpui_web::WebPlatform;
use http_client::{AsyncBody, HttpClient as _};
use std::thread::ThreadId;
use wasm_bindgen::prelude::*;

fn main() {}

#[wasm_bindgen]
pub struct FetchTest {
    platform: WebPlatform,
}

#[wasm_bindgen]
impl FetchTest {
    #[wasm_bindgen(constructor)]
    pub fn new(threaded: bool) -> Self {
        Self {
            platform: WebPlatform::new(threaded),
        }
    }

    pub async fn open(&self, url: String) -> Result<ResponseBodyTest, JsValue> {
        let response = self
            .platform
            .fetch_http_client()
            .get(&url, Default::default(), true)
            .await
            .map_err(js_error)?;
        Ok(ResponseBodyTest {
            body: Some(response.into_body()),
            background: self.platform.background_executor(),
            main_thread: std::thread::current().id(),
        })
    }
}

#[wasm_bindgen]
pub struct ResponseBodyTest {
    body: Option<AsyncBody>,
    background: BackgroundExecutor,
    main_thread: ThreadId,
}

#[wasm_bindgen]
impl ResponseBodyTest {
    pub async fn read(&mut self, on_worker: bool) -> Result<Vec<u8>, JsValue> {
        let mut body = self
            .body
            .take()
            .ok_or_else(|| JsValue::from_str("response body already consumed"))?;
        let main_thread = self.main_thread;
        let read = async move {
            if on_worker {
                anyhow::ensure!(
                    std::thread::current().id() != main_thread,
                    "response body must be read on a real worker"
                );
            }
            let mut bytes = Vec::new();
            body.read_to_end(&mut bytes).await?;
            anyhow::Ok(bytes)
        };
        if on_worker {
            self.background.spawn(read).await.map_err(js_error)
        } else {
            read.await.map_err(js_error)
        }
    }
}

fn js_error(error: anyhow::Error) -> JsValue {
    JsValue::from_str(&error.to_string())
}
