use event_listener::Event;
use test_log::test;
use tracing::instrument;
use zbus::block_on;

use zbus::Result;

#[instrument]
#[test]
fn concurrent_interface_methods() {
    // This is  test case for ensuring the regression of #799 doesn't come back.
    block_on(async {
        struct Iface(Event);

        #[zbus::interface(name = "org.zbus.test.issue799")]
        impl Iface {
            async fn method1(&self) {
                self.0.notify(1);
                // Never return
                std::future::pending::<()>().await;
            }

            async fn method2(&self) {}
        }

        let event = Event::new();
        let listener = event.listen();
        let iface = Iface(event);
        let conn = zbus::connection::Builder::session()
            .unwrap()
            .name("org.zbus.test.issue799")
            .unwrap()
            .serve_at("/org/zbus/test/issue799", iface)
            .unwrap()
            .build()
            .await
            .unwrap();

        #[zbus::proxy(
            default_service = "org.zbus.test.issue799",
            default_path = "/org/zbus/test/issue799",
            interface = "org.zbus.test.issue799"
        )]
        trait Iface {
            async fn method1(&self) -> Result<()>;
            async fn method2(&self) -> Result<()>;
        }

        let proxy = IfaceProxy::new(&conn).await.unwrap();
        let proxy_clone = proxy.clone();
        conn.executor()
            .spawn(
                async move {
                    proxy_clone.method1().await.unwrap();
                },
                "method1",
            )
            .detach();
        // Wait till the `method1`` is called.
        listener.await;

        // Now while the `method1` is in progress, a call to `method2` should just work.
        proxy.method2().await.unwrap();
    })
}
