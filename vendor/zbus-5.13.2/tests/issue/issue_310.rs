use std::sync::Arc;

use test_log::test;
use tracing::instrument;

use zvariant::OwnedObjectPath;

#[test(tokio::test(flavor = "multi_thread"))]
#[instrument]
async fn issue_310() {
    // The issue was we were deadlocking on fetching the new property value after invalidation.
    // This turned out to be caused by us trying to grab a read lock on resource while holding
    // a write lock. Thanks to connman for being weird and invalidating the property just before
    // updating it, so this issue could be exposed.
    use futures_util::StreamExt;
    use zbus::connection::Builder;

    struct Station(u64);

    #[zbus::interface(name = "net.connman.iwd.Station")]
    impl Station {
        #[zbus(property)]
        fn connected_network(&self) -> OwnedObjectPath {
            format!("/net/connman/iwd/0/33/Network/{}", self.0)
                .try_into()
                .unwrap()
        }
    }

    #[zbus::proxy(
        interface = "net.connman.iwd.Station",
        default_service = "net.connman.iwd"
    )]
    trait Station {
        #[zbus(property)]
        fn connected_network(&self) -> zbus::Result<OwnedObjectPath>;
    }
    let connection = Builder::session()
        .unwrap()
        .serve_at("/net/connman/iwd/0/33", Station(0))
        .unwrap()
        .name("net.connman.iwd")
        .unwrap()
        .build()
        .await
        .unwrap();
    let event = Arc::new(event_listener::Event::new());
    let conn_clone = connection.clone();
    let event_clone = event.clone();
    tokio::spawn(async move {
        for _ in 0..10 {
            let listener = event_clone.listen();
            let iface_ref = conn_clone
                .object_server()
                .interface::<_, Station>("/net/connman/iwd/0/33")
                .await
                .unwrap();

            {
                let iface = iface_ref.get().await;
                iface
                    .connected_network_changed(iface_ref.signal_emitter())
                    .await
                    .unwrap();
            }
            listener.await;
            iface_ref.get_mut().await.0 += 1;
        }
    });

    let station = StationProxy::builder(&connection)
        .path("/net/connman/iwd/0/33")
        .unwrap()
        .build()
        .await
        .unwrap();

    let mut changes = station.receive_connected_network_changed().await;

    let mut last_received = 0;
    while last_received < 9 {
        let change = changes.next().await.unwrap();
        let path = change.get().await.unwrap();
        let received: u64 = path
            .split('/')
            .next_back()
            .unwrap()
            .parse()
            .expect("invalid path");
        assert!(received >= last_received);
        last_received = received;
        event.notify(1);
    }
}
