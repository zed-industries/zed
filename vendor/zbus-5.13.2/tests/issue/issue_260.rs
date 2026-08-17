use ntest::timeout;
use test_log::test;

use zbus::{Connection, Result, block_on};

#[test]
#[timeout(15000)]
fn issue_260() {
    // Low-level server example in the book doesn't work. The reason was that
    // `Connection::request_name` implicitly created the associated `ObjectServer` to avoid
    // #68. This meant that the `ObjectServer` ended up replying to the incoming method call
    // with an error, before the service code could do so.
    block_on(async {
        let connection = Connection::session().await?;

        connection.request_name("org.zbus.Issue260").await?;

        futures_util::try_join!(service(&connection), client(&connection),)?;

        Ok::<(), zbus::Error>(())
    })
    .unwrap();
}

async fn service(connection: &Connection) -> Result<()> {
    use futures_util::stream::TryStreamExt;

    let mut stream = zbus::MessageStream::from(connection);
    while let Some(msg) = stream.try_next().await? {
        let msg_header = msg.header();

        match msg_header.message_type() {
            zbus::message::Type::MethodCall => {
                connection.reply(&msg_header, &()).await?;

                break;
            }
            _ => continue,
        }
    }

    Ok(())
}

async fn client(connection: &Connection) -> Result<()> {
    zbus::Proxy::new(
        connection,
        "org.zbus.Issue260",
        "/org/zbus/Issue260",
        "org.zbus.Issue260",
    )
    .await?
    .call::<_, _, ()>("Whatever", &())
    .await?;
    Ok(())
}
