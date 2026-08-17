use futures_util::future::join;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use wasm_streams::transform::*;

use crate::js::*;

#[wasm_bindgen_test]
async fn test_transform_stream_new() {
    let transform = TransformStream::from_raw(new_noop_transform_stream());
    join(
        async {
            let mut writable = transform.writable();
            let mut writer = writable.get_writer();
            writer.write(JsValue::from("Hello")).await.unwrap();
            writer.write(JsValue::from("world!")).await.unwrap();
            writer.close().await.unwrap();
        },
        async {
            let mut readable = transform.readable();
            let mut reader = readable.get_reader();
            assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("Hello")));
            assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("world!")));
            assert_eq!(reader.read().await.unwrap(), None);
        },
    )
    .await;
}

#[wasm_bindgen_test]
async fn test_transform_stream_new_uppercase() {
    let transform = TransformStream::from_raw(new_uppercase_transform_stream());
    join(
        async {
            let mut writable = transform.writable();
            let mut writer = writable.get_writer();
            writer.write(JsValue::from("Hello")).await.unwrap();
            writer.write(JsValue::from("world!")).await.unwrap();
            writer.close().await.unwrap();
        },
        async {
            let mut readable = transform.readable();
            let mut reader = readable.get_reader();
            assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("HELLO")));
            assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("WORLD!")));
            assert_eq!(reader.read().await.unwrap(), None);
        },
    )
    .await;
}
