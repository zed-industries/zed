use futures_util::StreamExt;
use ntest::timeout;
use test_log::test;

use zbus::{Connection, MessageStream, message::Message, names::UniqueName};

#[test]
#[timeout(15000)]
fn issue_68() {
    // Tests the fix for https://github.com/z-galaxy/zbus/issues/68
    //
    // While this is not an exact reproduction of the issue 68, the underlying problem it
    // produces is exactly the same: `Connection::call_method` dropping all incoming messages
    // while waiting for the reply to the method call.
    zbus::block_on(issue_68_async());
}

async fn issue_68_async() {
    let conn = Connection::session().await.unwrap();
    let mut stream = MessageStream::from(&conn);

    // Send a message as client before service starts to process messages
    let client_conn = Connection::session().await.unwrap();
    let destination = conn.unique_name().map(UniqueName::<'_>::from).unwrap();
    let msg = Message::method_call("/org/freedesktop/Issue68", "Ping")
        .unwrap()
        .destination(destination)
        .unwrap()
        .interface("org.freedesktop.Issue68")
        .unwrap()
        .build(&())
        .unwrap();
    let serial = msg.primary_header().serial_num();
    client_conn.send(&msg).await.unwrap();

    zbus::fdo::DBusProxy::new(&conn)
        .await
        .unwrap()
        .get_id()
        .await
        .unwrap();

    while let Some(m) = stream.next().await {
        let msg = m.unwrap();

        if msg.primary_header().serial_num() == serial {
            break;
        }
    }
}
