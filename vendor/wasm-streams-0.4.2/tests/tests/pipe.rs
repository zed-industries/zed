use futures_util::stream::iter;
use futures_util::{SinkExt, StreamExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use wasm_streams::readable::*;
use wasm_streams::writable::*;

use crate::js::*;
use crate::util::*;

#[wasm_bindgen_test]
async fn test_pipe_js_to_rust() {
    let chunks = vec![JsValue::from("Hello"), JsValue::from("world!")];
    let mut readable = ReadableStream::from_raw(new_readable_stream_from_array(
        chunks.clone().into_boxed_slice(),
    ));

    let (sink, stream) = SimpleChannel::<JsValue>::new().split();
    let sink = sink.sink_map_err(|_| JsValue::from_str("cannot happen"));
    let mut writable = WritableStream::from_sink(sink);

    readable.pipe_to(&mut writable).await.unwrap();

    // All chunks must be sent to sink
    let output = stream.collect::<Vec<_>>().await;
    assert_eq!(output, chunks);

    // Both streams must be closed
    readable.get_reader().closed().await.unwrap();
    writable.get_writer().closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_pipe_rust_to_js() {
    let stream = iter(vec!["Hello", "world!"]).map(|s| Ok(JsValue::from(s)));
    let mut readable = ReadableStream::from_stream(stream);

    let recording_stream = RecordingWritableStream::new();
    let mut writable = WritableStream::from_raw(recording_stream.stream());

    readable.pipe_to(&mut writable).await.unwrap();

    // All chunks must be sent to sink
    assert_eq!(
        recording_stream.events(),
        [
            RecordedEvent::Write(JsValue::from("Hello")),
            RecordedEvent::Write(JsValue::from("world!")),
            RecordedEvent::Close
        ]
    );

    // Both streams must be closed
    readable.get_reader().closed().await.unwrap();
    writable.get_writer().closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_pipe_prevent_close() {
    let chunks = vec![JsValue::from("Hello"), JsValue::from("world!")];
    let mut readable = ReadableStream::from_raw(new_readable_stream_from_array(
        chunks.clone().into_boxed_slice(),
    ));

    let recording_stream = RecordingWritableStream::new();
    let mut writable = WritableStream::from_raw(recording_stream.stream());

    readable
        .pipe_to_with_options(&mut writable, PipeOptions::new().prevent_close(true))
        .await
        .unwrap();

    // All chunks must be sent to sink, without closing it
    assert_eq!(
        recording_stream.events(),
        [
            RecordedEvent::Write(JsValue::from("Hello")),
            RecordedEvent::Write(JsValue::from("world!"))
        ]
    );

    // Readable stream must be closed
    readable.get_reader().closed().await.unwrap();
}
