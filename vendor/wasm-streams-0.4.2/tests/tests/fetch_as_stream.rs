use futures_util::{AsyncReadExt, TryStreamExt};
use js_sys::{global, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use wasm_streams::ReadableStream;
use web_sys::{Response, Window};

#[wasm_bindgen_test]
async fn test_fetch_as_stream() {
    // Make a fetch request
    let url = "https://rustwasm.github.io/assets/wasm-ferris.png";
    let window = global().unchecked_into::<Window>(); // hack to also support Node.js
    let resp_value = JsFuture::from(window.fetch_with_str(url))
        .await
        .unwrap_throw();
    let resp: Response = resp_value.dyn_into().unwrap_throw();

    // Get the response's body as a JS ReadableStream
    let raw_body = resp.body().unwrap_throw();
    let body = ReadableStream::from_raw(raw_body);

    // Convert the JS ReadableStream to a Rust stream
    let stream = body.into_stream();

    // Consume to an AsyncRead
    let mut async_read = stream
        .map_ok(|js_value| js_value.dyn_into::<Uint8Array>().unwrap_throw().to_vec())
        .map_err(|_js_error| std::io::Error::new(std::io::ErrorKind::Other, "failed to read"))
        .into_async_read();

    // Read the first 4 bytes
    let mut buf = [0u8; 4];
    assert_eq!(async_read.read(&mut buf).await.unwrap_throw(), 4);
    assert_eq!(&buf, b"\x89PNG");
}
