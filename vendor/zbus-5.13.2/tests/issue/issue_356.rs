use test_log::test;
use tracing::instrument;
use zbus::zvariant::OwnedObjectPath;

/// The "child" interface that will be referenced by the property.
struct Adapter {
    name: String,
}

#[zbus::interface(interface = "org.test.Adapter", proxy(assume_defaults = true))]
impl Adapter {
    #[zbus(property)]
    fn name(&self) -> String {
        self.name.clone()
    }
}

/// The "parent" interface with a property that returns an ObjectPath to an Adapter.
struct Device {
    adapter_path: OwnedObjectPath,
}

#[zbus::interface(interface = "org.test.Device", proxy(assume_defaults = true))]
impl Device {
    /// This property returns an ObjectPath, but the generated proxy should return an AdapterProxy.
    #[zbus(property, proxy(object = "Adapter"))]
    fn adapter(&self) -> OwnedObjectPath {
        self.adapter_path.clone()
    }
}

#[test(tokio::test(flavor = "multi_thread"))]
#[instrument]
async fn issue_356() {
    use zbus::connection::Builder;

    let adapter_path: OwnedObjectPath = "/org/test/adapter0".try_into().unwrap();
    let device_path: OwnedObjectPath = "/org/test/device0".try_into().unwrap();

    // Create the service connection with both interfaces registered
    let connection = Builder::session()
        .unwrap()
        .serve_at(
            &adapter_path,
            Adapter {
                name: "TestAdapter".to_string(),
            },
        )
        .unwrap()
        .serve_at(
            &device_path,
            Device {
                adapter_path: adapter_path.clone(),
            },
        )
        .unwrap()
        .name("org.test.Issue356")
        .unwrap()
        .build()
        .await
        .unwrap();

    // Create a proxy for the Device interface using the auto-generated DeviceProxy
    let device_proxy = DeviceProxy::builder(&connection)
        .destination("org.test.Issue356")
        .unwrap()
        .path(&device_path)
        .unwrap()
        .build()
        .await
        .unwrap();

    // Call the adapter() property - this should return an AdapterProxy, not OwnedObjectPath
    let adapter_proxy = device_proxy.adapter().await.unwrap();

    // Verify we can use the returned proxy to call methods on the Adapter interface
    let adapter_name = adapter_proxy.name().await.unwrap();
    assert_eq!(adapter_name, "TestAdapter");
}
