use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use futures_util::stream::{iter, pending, StreamExt, TryStreamExt};
use futures_util::{poll, AsyncReadExt, FutureExt};
use gloo_timers::future::sleep;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use wasm_streams::readable::*;

use crate::js::*;
use crate::util::*;

#[wasm_bindgen_test]
async fn test_readable_stream_new() {
    let mut readable = ReadableStream::from_raw(new_readable_stream_from_array(
        vec![JsValue::from("Hello"), JsValue::from("world!")].into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("Hello")));
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("world!")));
    assert_eq!(reader.read().await.unwrap(), None);
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_stream_into_stream() {
    let readable = ReadableStream::from_raw(new_readable_stream_from_array(
        vec![JsValue::from("Hello"), JsValue::from("world!")].into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    let mut stream = readable.into_stream();

    assert_eq!(stream.next().await, Some(Ok(JsValue::from("Hello"))));
    assert_eq!(stream.next().await, Some(Ok(JsValue::from("world!"))));
    assert_eq!(stream.next().await, None);
}

#[wasm_bindgen_test]
fn test_readable_stream_into_stream_impl_unpin() {
    let readable = ReadableStream::from_raw(new_noop_readable_stream());
    let stream: IntoStream = readable.into_stream();

    let _ = Pin::new(&stream); // must be Unpin for this to work
}

#[wasm_bindgen_test]
async fn test_readable_stream_reader_into_stream() {
    let mut readable = ReadableStream::from_raw(new_readable_stream_from_array(
        vec![JsValue::from("Hello"), JsValue::from("world!")].into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    {
        // Acquire a reader and wrap it in a Rust Stream
        let reader = readable.get_reader();
        let mut stream = reader.into_stream();

        assert_eq!(stream.next().await, Some(Ok(JsValue::from("Hello"))));
    }

    // Dropping the wrapped Stream should release the lock
    assert!(!readable.is_locked());

    {
        // Can acquire a new reader after wrapped stream is dropped
        let mut reader = readable.get_reader();
        assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("world!")));
        assert_eq!(reader.read().await.unwrap(), None);
    }
}

#[wasm_bindgen_test]
async fn test_readable_stream_from_stream() {
    let stream = iter(vec!["Hello", "world!"]).map(|s| Ok(JsValue::from(s)));
    let mut readable = ReadableStream::from_stream(stream);

    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("Hello")));
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("world!")));
    assert_eq!(reader.read().await.unwrap(), None);
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_stream_from_stream_cancel() {
    let stream = iter(vec!["Hello", "world!"]).map(|s| Ok(JsValue::from(s)));
    let (stream, observer) = observe_drop(stream);
    let mut readable = ReadableStream::from_stream(stream);

    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("Hello")));
    assert!(!observer.is_dropped());
    assert_eq!(reader.cancel().await, Ok(()));
    assert!(observer.is_dropped());
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_stream_multiple_readers() {
    let mut readable = ReadableStream::from_raw(new_noop_readable_stream());
    assert!(!readable.is_locked());

    // Release explicitly
    let reader = readable.get_reader();
    reader.release_lock();
    assert!(!readable.is_locked());

    // Release by drop
    let reader = readable.get_reader();
    drop(reader);
    assert!(!readable.is_locked());

    let reader = readable.get_reader();
    reader.release_lock();
    assert!(!readable.is_locked());
}

#[wasm_bindgen_test]
async fn test_readable_stream_abort_read() {
    if supports_release_lock_with_pending_read() {
        test_readable_stream_abort_read_new().await;
    } else {
        test_readable_stream_abort_read_old().await;
    }
}

async fn test_readable_stream_abort_read_new() {
    let stream = pending();
    let mut readable = ReadableStream::from_stream(stream);
    let mut reader = readable.get_reader();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut fut = reader.read().boxed_local();
    // We need to poll the future at least once to start the read
    let poll_result = poll!(&mut fut);
    assert!(matches!(poll_result, Poll::Pending));
    // Drop the future, to regain control over the reader
    drop(fut);

    // Releasing the lock should work even while there are pending reads
    reader
        .try_release_lock()
        .expect("releasing the reader should work even while there are pending reads");
}

async fn test_readable_stream_abort_read_old() {
    let stream = pending();
    let (stream, observer) = observe_drop(stream);
    let mut readable = ReadableStream::from_stream(stream);
    let mut reader = readable.get_reader();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut fut = reader.read().boxed_local();
    // We need to poll the future at least once to start the read
    let poll_result = poll!(&mut fut);
    assert!(matches!(poll_result, Poll::Pending));
    // Drop the future, to regain control over the reader
    drop(fut);

    // Cannot release the lock while there are pending reads
    let (_err, mut reader) = reader
        .try_release_lock()
        .expect_err("reader was released while there are pending reads");

    // Cancel all pending reads
    assert!(!observer.is_dropped());
    reader.cancel().await.unwrap();
    assert!(observer.is_dropped());

    // Can release lock after cancelling
    reader.release_lock();
}

#[wasm_bindgen_test]
async fn test_readable_stream_from_stream_then_into_stream() {
    let stream = iter(vec!["Hello", "world!"]).map(|s| Ok(JsValue::from(s)));
    let readable = ReadableStream::from_stream(stream);

    let mut stream = readable.into_stream();

    assert_eq!(stream.next().await, Some(Ok(JsValue::from("Hello"))));
    assert_eq!(stream.next().await, Some(Ok(JsValue::from("world!"))));
    assert_eq!(stream.next().await, None);
}

#[wasm_bindgen_test]
async fn test_readable_stream_into_stream_then_from_stream() {
    let readable = ReadableStream::from_raw(new_readable_stream_from_array(
        vec![JsValue::from("Hello"), JsValue::from("world!")].into_boxed_slice(),
    ));
    let stream = readable.into_stream();
    let mut readable = ReadableStream::from_stream(stream);

    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("Hello")));
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("world!")));
    assert_eq!(reader.read().await.unwrap(), None);
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_stream_tee() {
    let chunks = vec![JsValue::from("Hello"), JsValue::from("world!")];
    let readable = ReadableStream::from_raw(new_readable_stream_from_array(
        chunks.clone().into_boxed_slice(),
    ));

    let (left, right) = readable.tee();

    let left_chunks = left.into_stream().try_collect::<Vec<_>>().await.unwrap();
    let right_chunks = right.into_stream().try_collect::<Vec<_>>().await.unwrap();

    assert_eq!(left_chunks, chunks);
    assert_eq!(right_chunks, chunks);
}

#[wasm_bindgen_test]
async fn test_readable_stream_into_stream_auto_cancel() {
    let raw_readable = new_noop_readable_stream();
    let readable = ReadableStream::from_raw(raw_readable.clone());
    let mut stream = readable.into_stream();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut fut = stream.next().boxed_local();
    // We need to poll the future at least once to start the read
    let poll_result = poll!(&mut fut);
    assert!(matches!(poll_result, Poll::Pending));
    // Drop the future, to regain control over the stream
    drop(fut);

    // Drop the stream
    drop(stream);

    // Stream must be unlocked and cancelled
    let mut readable = ReadableStream::from_raw(raw_readable);
    assert!(!readable.is_locked());
    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), None);
}

#[wasm_bindgen_test]
async fn test_readable_stream_into_stream_manual_cancel() {
    let raw_readable = new_noop_readable_stream();
    let readable = ReadableStream::from_raw(raw_readable.clone());
    let mut stream = readable.into_stream();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut fut = stream.next().boxed_local();
    // We need to poll the future at least once to start the read
    let poll_result = poll!(&mut fut);
    assert!(matches!(poll_result, Poll::Pending));
    // Drop the future, to regain control over the stream
    drop(fut);

    // Cancel the stream
    stream.cancel().await.unwrap();

    // Stream must be unlocked and cancelled
    let mut readable = ReadableStream::from_raw(raw_readable);
    assert!(!readable.is_locked());
    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), None);
}

#[wasm_bindgen_test]
async fn test_readable_stream_into_stream_auto_cancel_rejects() {
    let _guard = UnhandledErrorGuard::new();

    let raw_readable = new_readable_stream_with_rejecting_cancel();
    let readable = ReadableStream::from_raw(raw_readable.clone());
    let stream = readable.into_stream();

    // Drop the stream
    drop(stream);

    // Stream must be unlocked and cancelled
    let mut readable = ReadableStream::from_raw(raw_readable);
    assert!(!readable.is_locked());
    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), None);

    // Wait a little bit for any unhandled rejections
    sleep(Duration::from_millis(100)).await;
}

#[wasm_bindgen_test]
async fn test_readable_stream_into_stream_then_into_async_read() {
    let readable = ReadableStream::from_raw(new_readable_stream_from_array(
        vec![
            Uint8Array::from(&[1, 2, 3][..]).into(),
            Uint8Array::from(&[4, 5, 6][..]).into(),
        ]
        .into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    let mut async_read = readable
        .into_stream()
        .map_ok(|value| value.dyn_into::<Uint8Array>().unwrap().to_vec())
        .map_err(|_err| std::io::Error::from(std::io::ErrorKind::Other))
        .into_async_read();
    let mut buf = [0u8; 3];
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf, &[1, 2, 3]);
    assert_eq!(async_read.read(&mut buf[..1]).await.unwrap(), 1);
    assert_eq!(&buf, &[4, 2, 3]);
    assert_eq!(async_read.read(&mut buf[1..]).await.unwrap(), 2);
    assert_eq!(&buf, &[4, 5, 6]);
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 0);
    assert_eq!(&buf, &[4, 5, 6]);
}

#[wasm_bindgen_test]
async fn test_readable_stream_from_js_array() {
    let js_array =
        js_sys::Array::from_iter([JsValue::from_str("Hello"), JsValue::from_str("world!")]);
    let mut readable = match ReadableStream::try_from(js_array.unchecked_into()) {
        Ok(readable) => readable,
        Err(err) => {
            // ReadableStream.from() is not yet supported in all browsers.
            assert_eq!(err.name(), "TypeError");
            assert_eq!(
                err.message().as_string().unwrap(),
                "ReadableStream.from is not a function"
            );
            return;
        }
    };
    assert!(!readable.is_locked());

    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("Hello")));
    assert_eq!(reader.read().await.unwrap(), Some(JsValue::from("world!")));
    assert_eq!(reader.read().await.unwrap(), None);
    reader.closed().await.unwrap();
}
