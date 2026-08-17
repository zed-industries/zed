use futures::pin_mut;
use futures_test::task::noop_context;
use std::io::IoSlice;
use std::task::Poll;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio_test::task::spawn;
use tokio_test::{assert_pending, assert_ready};
use tokio_util::io::simplex;

/// Sanity check for single-threaded operation.
#[tokio::test]
async fn single_thread() {
    const N: usize = 64;
    const MSG: &[u8] = b"Hello, world!";
    const CAPS: &[usize] = &[1, MSG.len() / 2, MSG.len() - 1, MSG.len(), MSG.len() + 1];

    // test different buffer capacities to cover edge cases
    for &capacity in CAPS {
        let (mut tx, mut rx) = simplex::new(capacity);

        for _ in 0..N {
            let mut read = 0;
            let mut write = 0;
            let mut buf = [0; MSG.len()];

            while read < MSG.len() || write < MSG.len() {
                if write < MSG.len() {
                    let n = tx.write(&MSG[write..]).await.unwrap();
                    write += n;
                }

                if read < MSG.len() {
                    let n = rx.read(&mut buf[read..]).await.unwrap();
                    read += n;
                }
            }

            assert_eq!(&buf[..], MSG);
        }
    }
}

/// Sanity check for multi-threaded operation.
#[test]
#[cfg(not(target_os = "wasi"))] // No thread on wasi.
fn multi_thread() {
    use futures::executor::block_on;
    use std::thread;

    const N: usize = 64;
    const MSG: &[u8] = b"Hello, world!";
    const CAPS: &[usize] = &[1, MSG.len() / 2, MSG.len() - 1, MSG.len(), MSG.len() + 1];

    // test different buffer capacities to cover edge cases
    for &capacity in CAPS {
        let (mut tx, mut rx) = simplex::new(capacity);

        let jh0 = thread::spawn(move || {
            block_on(async {
                let mut buf = vec![0; MSG.len()];
                for _ in 0..N {
                    rx.read_exact(&mut buf).await.unwrap();
                    assert_eq!(&buf[..], MSG);
                    buf.clear();
                    buf.resize(MSG.len(), 0);
                }
            });
        });

        let jh1 = thread::spawn(move || {
            block_on(async {
                for _ in 0..N {
                    tx.write_all(MSG).await.unwrap();
                }
            });
        });

        jh0.join().unwrap();
        jh1.join().unwrap();
    }
}

#[test]
#[should_panic(expected = "capacity must be greater than zero")]
fn zero_capacity() {
    let _ = simplex::new(0);
}

/// The `Receiver::poll_read` should return `Poll::Ready(Ok(()))`
/// if the `ReadBuf` has zero remaining capacity.
#[tokio::test]
async fn read_buf_is_full() {
    let (_tx, rx) = simplex::new(32);
    let mut buf = ReadBuf::new(&mut []);
    tokio::pin!(rx);
    assert_ready!(rx.as_mut().poll_read(&mut noop_context(), &mut buf)).unwrap();
    assert_eq!(buf.filled().len(), 0);
}

/// The `Sender::poll_write` should return `Poll::Ready(Ok(0))`
/// if the input buffer has zero length.
#[tokio::test]
async fn write_buf_is_empty() {
    let (tx, _rx) = simplex::new(32);
    tokio::pin!(tx);
    let n = assert_ready!(tx.as_mut().poll_write(&mut noop_context(), &[])).unwrap();
    assert_eq!(n, 0);
}

/// The `Sender` should returns error if the `Receiver` has been dropped.
#[tokio::test]
async fn drop_receiver_0() {
    let (mut tx, rx) = simplex::new(32);
    drop(rx);

    tx.write_u8(1).await.unwrap_err();
}

/// The `Sender` should be woken up if the `Receiver` has been dropped.
#[tokio::test]
async fn drop_receiver_1() {
    let (mut tx, rx) = simplex::new(1);
    let mut write_task = spawn(tx.write_u16(1));
    assert_pending!(write_task.poll());

    assert!(!write_task.is_woken());
    drop(rx);
    assert!(write_task.is_woken());
}

/// The `Receiver` should return error if:
///
/// - The `Sender` has been dropped.
/// - AND there is no remaining data in the buffer.
#[tokio::test]
async fn drop_sender_0() {
    const MSG: &[u8] = b"Hello, world!";

    let (tx, mut rx) = simplex::new(32);
    drop(tx);

    let mut buf = vec![0; MSG.len()];
    rx.read_exact(&mut buf).await.unwrap_err();
}

/// The `Receiver` should be woken up if:
///
/// - The `Sender` has been dropped.
/// - AND there is still remaining data in the buffer.
#[tokio::test]
async fn drop_sender_1() {
    let (mut tx, mut rx) = simplex::new(2);
    let mut buf = vec![];
    let mut read_task = spawn(rx.read_to_end(&mut buf));
    assert_pending!(read_task.poll());

    tx.write_u8(1).await.unwrap();
    assert_pending!(read_task.poll());

    assert!(!read_task.is_woken());
    drop(tx);
    assert!(read_task.is_woken());

    read_task.await.unwrap();
    assert_eq!(buf, vec![1]);
}

/// All following calls to `Sender::poll_write` and `Sender::poll_flush`
/// should return error after `shutdown` has been called.
#[tokio::test]
async fn shutdown_sender_0() {
    const MSG: &[u8] = b"Hello, world!";

    let (mut tx, _rx) = simplex::new(32);
    tx.shutdown().await.unwrap();

    tx.write_all(MSG).await.unwrap_err();
    tx.flush().await.unwrap_err();
}

/// The `Sender::poll_shutdown` should be called multiple times
/// without error.
#[tokio::test]
async fn shutdown_sender_1() {
    let (mut tx, _rx) = simplex::new(32);
    tx.shutdown().await.unwrap();
    tx.shutdown().await.unwrap();
}

/// The `Sender::poll_shutdown` should wake up the `Receiver`
#[tokio::test]
async fn shutdown_sender_2() {
    let (mut tx, mut rx) = simplex::new(32);

    let mut buf = vec![];
    let mut read_task = spawn(rx.read_to_end(&mut buf));
    assert_pending!(read_task.poll());

    tx.write_u8(1).await.unwrap();
    assert_pending!(read_task.poll());

    assert!(!read_task.is_woken());
    tx.shutdown().await.unwrap();
    assert!(read_task.is_woken());

    read_task.await.unwrap();
    assert_eq!(buf, vec![1]);
}

/// Both `Sender` and `Receiver` should yield periodically
/// in a tight-loop.
#[tokio::test]
#[cfg(feature = "rt")]
async fn cooperative_scheduling() {
    // this magic number is copied from
    // https://github.com/tokio-rs/tokio/blob/925c614c89d0a26777a334612e2ed6ad0e7935c3/tokio/src/task/coop/mod.rs#L116
    const INITIAL_BUDGET: usize = 128;

    let (tx, _rx) = simplex::new(INITIAL_BUDGET * 2);
    pin_mut!(tx);
    let mut is_pending = false;
    for _ in 0..INITIAL_BUDGET + 1 {
        match tx.as_mut().poll_write(&mut noop_context(), &[0u8; 1]) {
            Poll::Pending => {
                is_pending = true;
                break;
            }
            Poll::Ready(Ok(1)) => {}
            Poll::Ready(Ok(n)) => panic!("wrote too many bytes: {n}"),
            Poll::Ready(Err(e)) => panic!("{e}"),
        }
    }
    assert!(is_pending);

    let (tx, _rx) = simplex::new(INITIAL_BUDGET * 2);
    pin_mut!(tx);
    let mut is_pending = false;
    let io_slices = &[IoSlice::new(&[0u8; 1])];
    for _ in 0..INITIAL_BUDGET + 1 {
        match tx
            .as_mut()
            .poll_write_vectored(&mut noop_context(), io_slices)
        {
            Poll::Pending => {
                is_pending = true;
                break;
            }
            Poll::Ready(Ok(1)) => {}
            Poll::Ready(Ok(n)) => panic!("wrote too many bytes: {n}"),
            Poll::Ready(Err(e)) => panic!("{e}"),
        }
    }
    assert!(is_pending);

    let (mut tx, rx) = simplex::new(INITIAL_BUDGET * 2);
    tx.write_all(&[0u8; INITIAL_BUDGET + 2]).await.unwrap();
    pin_mut!(rx);
    let mut is_pending = false;
    for _ in 0..INITIAL_BUDGET + 1 {
        let mut buf = [0u8; 1];
        let mut buf = ReadBuf::new(&mut buf);
        match rx.as_mut().poll_read(&mut noop_context(), &mut buf) {
            Poll::Pending => {
                is_pending = true;
                break;
            }
            Poll::Ready(Ok(())) => assert_eq!(buf.filled().len(), 1),
            Poll::Ready(Err(e)) => panic!("{e}"),
        }
    }
    assert!(is_pending);
}

/// The capacity is exactly same as the total length of the vectored buffers.
#[tokio::test]
async fn poll_write_vectored_0() {
    const MSG1: &[u8] = b"1";
    const MSG2: &[u8] = b"22";
    const MSG3: &[u8] = b"333";
    const MSG_LEN: usize = MSG1.len() + MSG2.len() + MSG3.len();

    let io_slices = &[IoSlice::new(MSG1), IoSlice::new(MSG2), IoSlice::new(MSG3)];

    let (tx, mut rx) = simplex::new(MSG_LEN);
    tokio::pin!(tx);
    let res = tx.poll_write_vectored(&mut noop_context(), io_slices);
    let n = assert_ready!(res).unwrap();
    assert_eq!(n, MSG_LEN);
    let mut buf = [0; MSG_LEN];
    let n = rx.read_exact(&mut buf).await.unwrap();
    assert_eq!(n, MSG_LEN);
    assert_eq!(&buf, b"122333");
}

/// The capacity is smaller than the total length of the vectored buffers.
#[tokio::test]
async fn poll_write_vectored_1() {
    const MSG1: &[u8] = b"1";
    const MSG2: &[u8] = b"22";
    const MSG3: &[u8] = b"333";
    const CAPACITY: usize = MSG1.len() + MSG2.len() + 1;

    let io_slices = &[IoSlice::new(MSG1), IoSlice::new(MSG2), IoSlice::new(MSG3)];

    let (tx, mut rx) = simplex::new(CAPACITY);
    tokio::pin!(tx);

    // ==== The poll_write_vectored should write MSG1 and MSG2 fully, and MSG3 partially. ====
    let res = tx.poll_write_vectored(&mut noop_context(), io_slices);
    let n = assert_ready!(res).unwrap();
    assert_eq!(n, CAPACITY);
    let mut buf = [0; CAPACITY];
    let n = rx.read_exact(&mut buf).await.unwrap();
    assert_eq!(n, CAPACITY);
    assert_eq!(&buf, b"1223");
}

/// There are two empty buffers in the vectored buffers.
#[tokio::test]
async fn poll_write_vectored_2() {
    const MSG1: &[u8] = b"1";
    const MSG2: &[u8] = b"";
    const MSG3: &[u8] = b"22";
    const MSG4: &[u8] = b"";
    const MSG5: &[u8] = b"333";
    const MSG_LEN: usize = MSG1.len() + MSG2.len() + MSG3.len() + MSG4.len() + MSG5.len();

    let io_slices = &[
        IoSlice::new(MSG1),
        IoSlice::new(MSG2),
        IoSlice::new(MSG3),
        IoSlice::new(MSG4),
        IoSlice::new(MSG5),
    ];

    let (tx, mut rx) = simplex::new(MSG_LEN);
    tokio::pin!(tx);
    let res = tx.poll_write_vectored(&mut noop_context(), io_slices);
    let n = assert_ready!(res).unwrap();
    assert_eq!(n, MSG_LEN);
    let mut buf = [0; MSG_LEN];
    let n = rx.read_exact(&mut buf).await.unwrap();
    assert_eq!(n, MSG_LEN);
    assert_eq!(&buf, b"122333");
}

/// The `Sender::poll_write_vectored` should return `Poll::Ready(Ok(0))`
/// if all the input buffers have zero length.
#[tokio::test]
async fn poll_write_vectored_3() {
    let io_slices = &[IoSlice::new(&[]), IoSlice::new(&[]), IoSlice::new(&[])];
    let (tx, _rx) = simplex::new(32);
    tokio::pin!(tx);
    let n = assert_ready!(tx.poll_write_vectored(&mut noop_context(), io_slices)).unwrap();
    assert_eq!(n, 0);
}
