use ntest::timeout;
use test_log::test;
use zbus::block_on;

use zbus::Result;

#[test]
#[timeout(15000)]
fn uncached_property() {
    block_on(test_uncached_property()).unwrap();
}

async fn test_uncached_property() -> Result<()> {
    // A dummy boolean test service. It starts as `false` and can be
    // flipped to `true`. Two properties can access the inner value, with
    // and without caching.
    #[derive(Default)]
    struct ServiceUncachedPropertyTest(bool);
    #[zbus::interface(name = "org.freedesktop.zbus.UncachedPropertyTest")]
    impl ServiceUncachedPropertyTest {
        #[zbus(property)]
        fn cached_prop(&self) -> bool {
            self.0
        }
        #[zbus(property)]
        fn uncached_prop(&self) -> bool {
            self.0
        }
        async fn set_inner_to_true(&mut self) -> zbus::fdo::Result<()> {
            self.0 = true;
            Ok(())
        }
    }

    #[zbus::proxy(
        interface = "org.freedesktop.zbus.UncachedPropertyTest",
        default_service = "org.freedesktop.zbus.UncachedPropertyTest",
        default_path = "/org/freedesktop/zbus/UncachedPropertyTest"
    )]
    trait UncachedPropertyTest {
        #[zbus(property)]
        fn cached_prop(&self) -> zbus::Result<bool>;

        #[zbus(property(emits_changed_signal = "false"))]
        fn uncached_prop(&self) -> zbus::Result<bool>;

        fn set_inner_to_true(&self) -> zbus::Result<()>;
    }

    let service = zbus::connection::Builder::session()
        .unwrap()
        .serve_at(
            "/org/freedesktop/zbus/UncachedPropertyTest",
            ServiceUncachedPropertyTest(false),
        )
        .unwrap()
        .build()
        .await
        .unwrap();

    let dest = service.unique_name().unwrap();

    let client_conn = zbus::Connection::session().await.unwrap();
    let client = UncachedPropertyTestProxy::builder(&client_conn)
        .destination(dest)
        .unwrap()
        .build()
        .await
        .unwrap();

    // Query properties; this populates the cache too.
    assert!(!client.cached_prop().await.unwrap());
    assert!(!client.uncached_prop().await.unwrap());

    // Flip the inner value so we can observe the different semantics of
    // the two properties.
    client.set_inner_to_true().await.unwrap();

    // Query properties again; the first one should incur a stale read from
    // cache, while the second one should be able to read the live/updated
    // value.
    assert!(!client.cached_prop().await.unwrap());
    assert!(client.uncached_prop().await.unwrap());

    Ok(())
}
