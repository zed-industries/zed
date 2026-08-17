use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use futures_util::AsyncReadExt;
use futures_util::{poll, FutureExt};
use gloo_timers::future::sleep;
use js_sys::Uint8Array;
use wasm_bindgen_test::*;

use wasm_streams::readable::*;

use crate::js::*;
use crate::util::*;

#[wasm_bindgen_test]
async fn test_readable_byte_stream_new() {
    let mut readable = ReadableStream::from_raw(new_readable_byte_stream_from_array(
        vec![
            Uint8Array::from(&[1, 2, 3][..]).into(),
            Uint8Array::from(&[4, 5, 6][..]).into(),
        ]
        .into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    let mut reader = readable.get_byob_reader();
    let mut dst = [0u8; 3];
    assert_eq!(reader.read(&mut dst).await.unwrap(), 3);
    assert_eq!(&dst, &[1, 2, 3]);
    assert_eq!(reader.read(&mut dst).await.unwrap(), 3);
    assert_eq!(&dst, &[4, 5, 6]);
    assert_eq!(reader.read(&mut dst).await.unwrap(), 0);
    assert_eq!(&dst, &[4, 5, 6]);
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_read_with_buffer() {
    let mut readable = ReadableStream::from_raw(new_readable_byte_stream_from_array(
        vec![
            Uint8Array::from(&[1, 2, 3][..]).into(),
            Uint8Array::from(&[4, 5, 6][..]).into(),
        ]
        .into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    let mut reader = readable.get_byob_reader();
    let mut dst = [0u8; 3];
    let buf = Uint8Array::new_with_length(3);
    let (bytes_read, buf) = reader.read_with_buffer(&mut dst, buf).await.unwrap();
    assert_eq!(bytes_read, 3);
    assert_eq!(&dst, &[1, 2, 3]);
    let (bytes_read, buf) = reader
        .read_with_buffer(&mut dst, buf.unwrap())
        .await
        .unwrap();
    assert_eq!(bytes_read, 3);
    assert_eq!(&dst, &[4, 5, 6]);
    let (bytes_read, buf) = reader
        .read_with_buffer(&mut dst, buf.unwrap())
        .await
        .unwrap();
    assert_eq!(bytes_read, 0);
    assert_eq!(&dst, &[4, 5, 6]);
    drop(buf);
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_into_async_read() {
    let readable = ReadableStream::from_raw(new_readable_byte_stream_from_array(
        vec![
            Uint8Array::from(&[1, 2, 3][..]).into(),
            Uint8Array::from(&[4, 5, 6][..]).into(),
        ]
        .into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    let mut async_read = readable.into_async_read();
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
fn test_readable_byte_stream_into_async_read_impl_unpin() {
    let readable = ReadableStream::from_raw(new_noop_readable_byte_stream());
    let async_read = readable.into_async_read();

    let _ = Pin::new(&async_read); // must be Unpin for this to work
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_byob_reader_into_async_read() {
    let mut readable = ReadableStream::from_raw(new_readable_byte_stream_from_array(
        vec![
            Uint8Array::from(&[1, 2, 3][..]).into(),
            Uint8Array::from(&[4, 5, 6][..]).into(),
        ]
        .into_boxed_slice(),
    ));
    assert!(!readable.is_locked());

    {
        // Acquire a BYOB reader and wrap it in a Rust Stream
        let reader = readable.get_byob_reader();
        let mut async_read = reader.into_async_read();

        let mut buf = [0u8; 3];
        assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
        assert_eq!(&buf, &[1, 2, 3]);
    }

    // Dropping the wrapped Stream should release the lock
    assert!(!readable.is_locked());

    {
        // Can acquire a new reader after wrapped Stream is dropped
        let mut reader = readable.get_byob_reader();
        let mut buf = [0u8; 3];
        assert_eq!(reader.read(&mut buf).await.unwrap(), 3);
        assert_eq!(&buf, &[4, 5, 6]);
        assert_eq!(reader.read(&mut buf).await.unwrap(), 0);
        reader.closed().await.unwrap();
    }
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_from_async_read() {
    static ASYNC_READ: [u8; 6] = [1, 2, 3, 4, 5, 6];
    let mut readable = ReadableStream::from_async_read(&ASYNC_READ[..], 2);
    assert!(!readable.is_locked());

    let mut reader = readable.get_byob_reader();
    let mut dst = [0u8; 3];
    let buf = Uint8Array::new_with_length(3);
    let (bytes_read, buf) = reader.read_with_buffer(&mut dst, buf).await.unwrap();
    assert_eq!(bytes_read, 3);
    assert_eq!(&dst, &[1, 2, 3]);
    let (bytes_read, buf) = reader
        .read_with_buffer(&mut dst[0..2], buf.unwrap())
        .await
        .unwrap();
    assert_eq!(bytes_read, 2);
    assert_eq!(&dst, &[4, 5, 3]);
    let (bytes_read, buf) = reader
        .read_with_buffer(&mut dst[2..], buf.unwrap())
        .await
        .unwrap();
    assert_eq!(bytes_read, 1);
    assert_eq!(&dst, &[4, 5, 6]);
    let (bytes_read, buf) = reader
        .read_with_buffer(&mut dst, buf.unwrap())
        .await
        .unwrap();
    assert_eq!(bytes_read, 0);
    assert_eq!(&dst, &[4, 5, 6]);
    drop(buf);
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_from_async_read_cancel() {
    static ASYNC_READ: [u8; 6] = [1, 2, 3, 4, 5, 6];
    let mut readable = ReadableStream::from_async_read(&ASYNC_READ[..], 2);

    let mut reader = readable.get_byob_reader();
    let mut dst = [0u8; 3];
    assert_eq!(reader.read(&mut dst).await.unwrap(), 3);
    assert_eq!(&dst, &[1, 2, 3]);
    assert_eq!(reader.cancel().await, Ok(()));
    reader.closed().await.unwrap();
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_multiple_byob_readers() {
    let mut readable = ReadableStream::from_raw(new_noop_readable_byte_stream());
    assert!(!readable.is_locked());

    // Release explicitly
    let reader = readable.get_byob_reader();
    reader.release_lock();
    assert!(!readable.is_locked());

    // Release by drop
    let reader = readable.get_byob_reader();
    drop(reader);
    assert!(!readable.is_locked());

    let reader = readable.get_byob_reader();
    reader.release_lock();
    assert!(!readable.is_locked());
}

async fn test_readable_byte_stream_abort_read(readable: ReadableStream) {
    if supports_release_lock_with_pending_read() {
        test_readable_byte_stream_abort_read_new(readable).await;
    } else {
        test_readable_byte_stream_abort_read_old(readable).await;
    }
}

async fn test_readable_byte_stream_abort_read_new(mut readable: ReadableStream) {
    let mut reader = readable.get_byob_reader();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut dst = [0u8; 3];
    let mut fut = reader.read(&mut dst).boxed_local();
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

async fn test_readable_byte_stream_abort_read_old(mut readable: ReadableStream) {
    let mut reader = readable.get_byob_reader();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut dst = [0u8; 3];
    let mut fut = reader.read(&mut dst).boxed_local();
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
    reader.cancel().await.unwrap();

    // Can release lock after cancelling
    reader.release_lock();
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_abort_read_from_raw() {
    let readable = ReadableStream::from_raw(new_noop_readable_byte_stream());
    test_readable_byte_stream_abort_read(readable).await
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_abort_read_from_async_read() {
    static ASYNC_READ: [u8; 6] = [1, 2, 3, 4, 5, 6];
    let readable = ReadableStream::from_async_read(&ASYNC_READ[..], 2);
    test_readable_byte_stream_abort_read(readable).await
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_into_async_read_auto_cancel() {
    let raw_readable = new_noop_readable_byte_stream();
    let readable = ReadableStream::from_raw(raw_readable.clone());
    let mut async_read = readable.into_async_read();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut buf = [0u8; 1];
    let mut fut = async_read.read(&mut buf).boxed_local();
    // We need to poll the future at least once to start the read
    let poll_result = poll!(&mut fut);
    assert!(matches!(poll_result, Poll::Pending));
    // Drop the future, to regain control over the AsyncRead
    drop(fut);

    // Drop the AsyncRead
    drop(async_read);

    // Stream must be unlocked and cancelled
    let mut readable = ReadableStream::from_raw(raw_readable);
    assert!(!readable.is_locked());
    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), None);
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_into_async_read_auto_cancel_rejects() {
    let _guard = UnhandledErrorGuard::new();

    let raw_readable = new_readable_byte_stream_with_rejecting_cancel();
    let readable = ReadableStream::from_raw(raw_readable.clone());
    let async_read = readable.into_async_read();

    // Drop the AsyncRead
    drop(async_read);

    // Stream must be unlocked and cancelled
    let mut readable = ReadableStream::from_raw(raw_readable);
    assert!(!readable.is_locked());
    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), None);

    // Wait a little bit for any unhandled rejections
    sleep(Duration::from_millis(100)).await;
}

#[wasm_bindgen_test]
async fn test_readable_byte_stream_into_async_read_manual_cancel() {
    let raw_readable = new_noop_readable_byte_stream();
    let readable = ReadableStream::from_raw(raw_readable.clone());
    let mut async_read = readable.into_async_read();

    // Start reading
    // Since the stream will never produce a chunk, this read will remain pending forever
    let mut buf = [0u8; 1];
    let mut fut = async_read.read(&mut buf).boxed_local();
    // We need to poll the future at least once to start the read
    let poll_result = poll!(&mut fut);
    assert!(matches!(poll_result, Poll::Pending));
    // Drop the future, to regain control over the AsyncRead
    drop(fut);

    // Cancel the AsyncRead
    async_read.cancel().await.unwrap();

    // Stream must be unlocked and cancelled
    let mut readable = ReadableStream::from_raw(raw_readable);
    assert!(!readable.is_locked());
    let mut reader = readable.get_reader();
    assert_eq!(reader.read().await.unwrap(), None);
}
