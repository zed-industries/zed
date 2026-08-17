use test_log::test;
use tracing::instrument;

#[test(tokio::test(flavor = "multi_thread", worker_threads = 2))]
#[instrument]
async fn issue_279() {
    // On failure to read from the socket, we were closing the error channel from the sender
    // side and since the underlying tokio API doesn't provide a `close` method on the sender,
    // the async-channel abstraction was achieving this through calling `close` on receiver,
    // which is behind an async mutex and we end up with a deadlock.
    use futures_util::{stream::TryStreamExt, try_join};
    use tokio::net::UnixStream;
    use zbus::{MessageStream, connection::Builder};

    let guid = zbus::Guid::generate();
    let (p0, p1) = UnixStream::pair().unwrap();

    let server = Builder::unix_stream(p0).server(guid).unwrap().p2p().build();
    let client = Builder::unix_stream(p1).p2p().build();
    let (client, server) = try_join!(client, server).unwrap();
    let mut stream = MessageStream::from(client);
    let next_msg_fut = stream.try_next();

    drop(server);

    assert!(next_msg_fut.await.is_err());
}
