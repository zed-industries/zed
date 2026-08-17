#![allow(clippy::disallowed_names)]

mod iface_and_proxy;

#[cfg(all(unix, not(feature = "tokio"), feature = "p2p"))]
use std::os::unix::net::UnixStream;
use std::time::Duration;
#[cfg(all(unix, feature = "tokio", feature = "p2p"))]
use tokio::net::UnixStream;

use ntest::timeout;
use test_log::test;
use tokio::sync::mpsc::channel;
use tracing::{debug, instrument};
use zbus::{block_on, connection, fdo::ObjectManager, object_server::InterfaceRef};

use iface_and_proxy::{
    client::my_iface_test,
    iface::{MyIface, MyIfaceSignals},
    types::NextAction,
};

#[test]
#[timeout(15000)]
fn iface_and_proxy() {
    block_on(iface_and_proxy_(false));
}

#[cfg(feature = "p2p")]
#[cfg(unix)]
#[test]
#[timeout(15000)]
fn iface_and_proxy_unix_p2p() {
    block_on(iface_and_proxy_(true));
}

#[instrument]
async fn iface_and_proxy_(#[allow(unused)] p2p: bool) {
    let event = event_listener::Event::new();
    #[cfg(feature = "p2p")]
    let guid = zbus::Guid::generate();

    let session_conns_build = || {
        let service_conn_builder = connection::Builder::session()
            .unwrap()
            .name("org.freedesktop.MyService")
            .unwrap()
            .name("org.freedesktop.MyService.foo")
            .unwrap()
            .name("org.freedesktop.MyService.bar")
            .unwrap()
            .name("org.freedesktop.MyEmitsChangedSignalIface")
            .unwrap();
        let client_conn_builder = connection::Builder::session().unwrap();

        (service_conn_builder, client_conn_builder)
    };
    #[cfg(feature = "p2p")]
    let (service_conn_builder, client_conn_builder) = if p2p {
        #[cfg(unix)]
        {
            let (p0, p1) = UnixStream::pair().unwrap();

            (
                connection::Builder::unix_stream(p0)
                    .server(guid)
                    .unwrap()
                    .p2p(),
                connection::Builder::unix_stream(p1).p2p(),
            )
        }

        #[cfg(windows)]
        {
            #[cfg(not(feature = "tokio"))]
            {
                let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
                let addr = listener.local_addr().unwrap();
                let p1 = std::net::TcpStream::connect(addr).unwrap();
                let p0 = listener.incoming().next().unwrap().unwrap();

                (
                    connection::Builder::tcp_stream(p0)
                        .server(guid)
                        .unwrap()
                        .p2p(),
                    connection::Builder::tcp_stream(p1).p2p(),
                )
            }

            #[cfg(feature = "tokio")]
            {
                let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
                let addr = listener.local_addr().unwrap();
                let p1 = tokio::net::TcpStream::connect(addr).await.unwrap();
                let p0 = listener.accept().await.unwrap().0;

                (
                    connection::Builder::tcp_stream(p0)
                        .server(guid)
                        .unwrap()
                        .p2p(),
                    connection::Builder::tcp_stream(p1).p2p(),
                )
            }
        }
    } else {
        session_conns_build()
    };
    #[cfg(not(feature = "p2p"))]
    let (service_conn_builder, client_conn_builder) = session_conns_build();

    debug!(
        "Client connection builder created: {:?}",
        client_conn_builder
    );
    debug!(
        "Service connection builder created: {:?}",
        service_conn_builder
    );
    let (next_tx, mut next_rx) = channel(64);
    let iface = MyIface::new(next_tx.clone());
    let service_conn_builder = service_conn_builder
        .serve_at("/org/freedesktop/MyService", iface)
        .unwrap()
        .serve_at("/zbus/test", ObjectManager)
        .unwrap();
    debug!("ObjectServer set-up.");

    let (service_conn, client_conn) = futures_util::try_join!(
        service_conn_builder.build(),
        client_conn_builder
            .method_timeout(Duration::from_millis(100))
            .build(),
    )
    .unwrap();
    debug!("Client connection created: {:?}", client_conn);
    debug!("Service connection created: {:?}", service_conn);

    let listen = event.listen();
    let child = client_conn
        .executor()
        .spawn(my_iface_test(client_conn.clone(), event), "client_task");
    debug!("Child task spawned.");
    // Wait for the listener to be ready
    listen.await;
    debug!("Child task signaled it's ready.");

    let iface: InterfaceRef<MyIface> = service_conn
        .object_server()
        .interface("/org/freedesktop/MyService")
        .await
        .unwrap();
    iface
        .get()
        .await
        .count_changed(iface.signal_emitter())
        .await
        .unwrap();
    debug!("`PropertiesChanged` emitted for `Count` property.");

    loop {
        iface.alert_count(51).await.unwrap();
        debug!("`AlertCount` signal emitted.");

        match next_rx.recv().await.unwrap() {
            NextAction::Quit => break,
            NextAction::CreateObj(key) => {
                let path = format!("/zbus/test/{key}");
                service_conn
                    .object_server()
                    .at(path.clone(), MyIface::new(next_tx.clone()))
                    .await
                    .unwrap();
                debug!("Object `{path}` added.");
            }
            NextAction::DestroyObj(key) => {
                let path = format!("/zbus/test/{key}");
                service_conn
                    .object_server()
                    .remove::<MyIface, _>(path.clone())
                    .await
                    .unwrap();
                debug!("Object `{path}` removed.");
            }
        }
    }
    debug!("Server done.");

    // don't close the connection before we end the loop
    drop(client_conn);
    debug!("Connection closed.");

    let val = child.await.unwrap();
    debug!("Client task done.");
    assert_eq!(val.unwrap(), 2);

    if p2p {
        debug!("p2p connection, no need to release names..");
        return;
    }

    // Release primary name explicitly and let others be released implicitly.
    assert_eq!(
        service_conn.release_name("org.freedesktop.MyService").await,
        Ok(true)
    );
    debug!("Bus name `org.freedesktop.MyService` released.");
    assert_eq!(
        service_conn
            .release_name("org.freedesktop.MyService.foo")
            .await,
        Ok(true)
    );
    debug!("Bus name `org.freedesktop.MyService.foo` released.");
    assert_eq!(
        service_conn
            .release_name("org.freedesktop.MyService.bar")
            .await,
        Ok(true)
    );
    debug!("Bus name `org.freedesktop.MyService.bar` released.");

    // Let's ensure all names were released.
    let proxy = zbus::fdo::DBusProxy::new(&service_conn).await.unwrap();
    debug!("DBusProxy created to ensure all names were released.");
    assert_eq!(
        proxy
            .name_has_owner("org.freedesktop.MyService".try_into().unwrap())
            .await,
        Ok(false)
    );
    assert_eq!(
        proxy
            .name_has_owner("org.freedesktop.MyService.foo".try_into().unwrap())
            .await,
        Ok(false)
    );
    assert_eq!(
        proxy
            .name_has_owner("org.freedesktop.MyService.bar".try_into().unwrap())
            .await,
        Ok(false)
    );
    debug!("Bus confirmed that all names were definitely released.");
}
