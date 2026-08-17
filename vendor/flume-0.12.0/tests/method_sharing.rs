#[cfg(feature = "async")]
use flume::*;

#[cfg(feature = "async")]
#[async_std::test]
async fn sender() {
    let (sender, receiver) = bounded(1);

    let sender_fut = sender.send_async(());
    assert_eq!(sender.is_disconnected(), sender_fut.is_disconnected());
    assert_eq!(sender.is_empty(), sender_fut.is_empty());
    assert_eq!(sender.is_full(), sender_fut.is_full());
    assert_eq!(sender.len(), sender_fut.len());
    assert_eq!(sender.capacity(), sender_fut.capacity());

    let sender_sink = sender.sink();
    assert_eq!(sender.is_disconnected(), sender_sink.is_disconnected());
    assert_eq!(sender.is_empty(), sender_sink.is_empty());
    assert_eq!(sender.is_full(), sender_sink.is_full());
    assert_eq!(sender.len(), sender_sink.len());
    assert_eq!(sender.capacity(), sender_sink.capacity());

    let receiver_fut = receiver.recv_async();
    assert_eq!(receiver.is_disconnected(), receiver_fut.is_disconnected());
    assert_eq!(receiver.is_empty(), receiver_fut.is_empty());
    assert_eq!(receiver.is_full(), receiver_fut.is_full());
    assert_eq!(receiver.len(), receiver_fut.len());
    assert_eq!(receiver.capacity(), receiver_fut.capacity());

    let receiver_stream = receiver.stream();
    assert_eq!(
        receiver.is_disconnected(),
        receiver_stream.is_disconnected()
    );
    assert_eq!(receiver.is_empty(), receiver_stream.is_empty());
    assert_eq!(receiver.is_full(), receiver_stream.is_full());
    assert_eq!(receiver.len(), receiver_stream.len());
    assert_eq!(receiver.capacity(), receiver_stream.capacity());
}
