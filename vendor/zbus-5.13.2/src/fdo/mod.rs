mod error;
pub use error::{Error, Result};

pub(crate) mod dbus;
pub use dbus::{
    ConnectionCredentials, DBusProxy, NameAcquired, NameAcquiredArgs, NameAcquiredStream, NameLost,
    NameLostArgs, NameLostStream, NameOwnerChanged, NameOwnerChangedArgs, NameOwnerChangedStream,
    ReleaseNameReply, RequestNameFlags, RequestNameReply, StartServiceReply,
};

pub(crate) mod introspectable;
pub(crate) use introspectable::Introspectable;
pub use introspectable::IntrospectableProxy;

pub(crate) mod monitoring;
pub use monitoring::MonitoringProxy;

pub(crate) mod object_manager;
pub use object_manager::{
    InterfacesAdded, InterfacesAddedArgs, InterfacesAddedStream, InterfacesRemoved,
    InterfacesRemovedArgs, InterfacesRemovedStream, ManagedObjects, ObjectManager,
    ObjectManagerProxy,
};

pub(crate) mod peer;
pub(crate) use peer::Peer;
pub use peer::PeerProxy;

pub(crate) mod properties;
pub use properties::{
    Properties, PropertiesChanged, PropertiesChangedArgs, PropertiesChangedStream, PropertiesProxy,
};

pub(crate) mod stats;
pub use stats::StatsProxy;

#[cfg(test)]
mod tests {
    use crate::{DBusError, Error, fdo, interface, message::Message};
    use futures_util::StreamExt;
    use ntest::timeout;
    use test_log::test;
    use tokio::runtime;
    use zbus_names::WellKnownName;

    use super::PeerProxy;

    #[test]
    fn error_from_zerror() {
        let m = Message::method_call("/", "foo")
            .unwrap()
            .destination(":1.2")
            .unwrap()
            .build(&())
            .unwrap();
        let m = Message::error(&m.header(), "org.freedesktop.DBus.Error.TimedOut")
            .unwrap()
            .build(&("so long"))
            .unwrap();
        let e: Error = m.into();
        let e: fdo::Error = e.into();
        assert_eq!(e, fdo::Error::TimedOut("so long".to_string()),);
        assert_eq!(e.name(), "org.freedesktop.DBus.Error.TimedOut");
        assert_eq!(e.description(), Some("so long"));
    }

    #[test]
    #[timeout(15000)]
    fn signal() {
        // Multi-threaded scheduler.
        runtime::Runtime::new().unwrap().block_on(test_signal());

        // single-threaded scheduler.
        runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap()
            .block_on(test_signal());
    }

    async fn test_signal() {
        let conn = crate::Connection::session().await.unwrap();
        let proxy = fdo::DBusProxy::new(&conn).await.unwrap();

        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let well_known = "org.freedesktop.zbus.FdoSignalStreamTest";
        let unique_name = conn.unique_name().unwrap();
        let owner_change_stream = proxy
            .receive_name_owner_changed_with_args(&[(0, well_known), (2, unique_name.as_str())])
            .await
            .unwrap();

        let name_acquired_stream = proxy
            .receive_name_acquired_with_args(&[(0, well_known)])
            .await
            .unwrap();
        let mut stream = owner_change_stream.zip(name_acquired_stream);

        let well_known: WellKnownName<'static> = well_known.try_into().unwrap();
        proxy
            .request_name(
                well_known.as_ref(),
                fdo::RequestNameFlags::ReplaceExisting.into(),
            )
            .await
            .unwrap();

        let (name_owner_changed, name_acquired) = stream.next().await.unwrap();
        assert_eq!(name_owner_changed.args().unwrap().name(), &well_known);
        assert_eq!(
            *name_owner_changed
                .args()
                .unwrap()
                .new_owner()
                .as_ref()
                .unwrap(),
            *unique_name
        );
        assert_eq!(name_acquired.args().unwrap().name(), &well_known);

        let result = proxy.release_name(well_known.as_ref()).await.unwrap();
        assert_eq!(result, fdo::ReleaseNameReply::Released);

        let result = proxy.release_name(well_known).await.unwrap();
        assert_eq!(result, fdo::ReleaseNameReply::NonExistent);

        let _stream = proxy
            .receive_features_changed()
            .await
            .filter_map(|changed| async move {
                let v = changed.get().await.ok();
                dbg!(v)
            });
    }

    #[test]
    #[timeout(15000)]
    fn no_object_manager_signals_before_hello() {
        crate::block_on(no_object_manager_signals_before_hello_async());
    }

    async fn no_object_manager_signals_before_hello_async() {
        // We were emitting `InterfacesAdded` signals before `Hello` was called, which is wrong and
        // results in us getting disconnected by the bus. This test case ensures we don't do that
        // and also that the signals are eventually emitted.

        // Let's first create an interator to get the signals (it has to be another connection).
        let conn = zbus::Connection::session().await.unwrap();
        let mut stream = zbus::MessageStream::for_match_rule(
            zbus::MatchRule::builder()
                .msg_type(zbus::message::Type::Signal)
                .interface("org.freedesktop.DBus.ObjectManager")
                .unwrap()
                .path("/org/zbus/NoObjectManagerSignalsBeforeHello")
                .unwrap()
                .build(),
            &conn,
            None,
        )
        .await
        .unwrap();

        // Now create the service side.
        struct TestObj;
        #[interface(name = "org.zbus.TestObj")]
        impl TestObj {
            #[zbus(property)]
            fn test(&self) -> String {
                "test".into()
            }
        }
        let _conn = zbus::conn::Builder::session()
            .unwrap()
            .name("org.zbus.NoObjectManagerSignalsBeforeHello")
            .unwrap()
            .serve_at("/org/zbus/NoObjectManagerSignalsBeforeHello/Obj", TestObj)
            .unwrap()
            .serve_at(
                "/org/zbus/NoObjectManagerSignalsBeforeHello",
                super::ObjectManager,
            )
            .unwrap()
            .build()
            .await
            .unwrap();

        // Let's see if the `InterfacesAdded` signal was emitted.
        let msg = stream.next().await.unwrap().unwrap();
        let signal = super::InterfacesAdded::from_message(msg).unwrap();
        assert_eq!(
            signal.args().unwrap().interfaces_and_properties,
            vec![(
                "org.zbus.TestObj".try_into().unwrap(),
                vec![("Test", zvariant::Value::new("test"))]
                    .into_iter()
                    .collect()
            )]
            .into_iter()
            .collect()
        );
    }

    #[test]
    #[timeout(15000)]
    fn peer_on_arbitrary_path() {
        crate::block_on(peer_on_arbitrary_path_async());
    }

    /// Test that org.freedesktop.DBus.Peer works on arbitrary paths.
    ///
    /// According to the D-Bus specification:
    /// "On receipt of the METHOD_CALL message org.freedesktop.DBus.Peer.Ping, an application
    /// should do nothing other than reply with a METHOD_RETURN as usual. It does not matter
    /// which object path a ping is sent to."
    ///
    /// This test verifies that Ping and GetMachineId work on paths that haven't been
    /// registered via ObjectServer::at.
    async fn peer_on_arbitrary_path_async() {
        // Create a service with only a registered path at /registered
        struct TestObj;
        #[interface(name = "org.zbus.TestObj")]
        impl TestObj {}

        let _service_conn = zbus::conn::Builder::session()
            .unwrap()
            .name("org.zbus.PeerArbitraryPathTest")
            .unwrap()
            .serve_at("/registered", TestObj)
            .unwrap()
            .build()
            .await
            .unwrap();

        // Create a client connection
        let client_conn = zbus::Connection::session().await.unwrap();

        // Ping on an unregistered path should succeed
        let proxy = PeerProxy::new(
            &client_conn,
            "org.zbus.PeerArbitraryPathTest",
            "/this/path/does/not/exist",
        )
        .await
        .expect("Failed to create PeerProxy");
        let result = proxy.ping().await;
        assert!(result.is_ok(), "Ping on unregistered path should succeed");

        // GetMachineId on an unregistered path should succeed
        let proxy = PeerProxy::new(
            &client_conn,
            "org.zbus.PeerArbitraryPathTest",
            "/another/unregistered/path",
        )
        .await
        .expect("Failed to create PeerProxy");
        let result = proxy.get_machine_id().await;
        // GetMachineId may fail on some platforms (like *BSD), but should not fail
        // with "Unknown object" error
        match &result {
            Ok(id) => {
                assert!(!id.is_empty(), "Machine ID should not be empty");
            }
            #[cfg(not(any(
                target_os = "linux",
                target_os = "macos",
                target_os = "freebsd",
                target_os = "dragonfly",
                target_os = "openbsd",
                target_os = "netbsd",
                windows,
            )))]
            Err(fdo::Error::NotSupported(_)) => {
                // NotSupported is acceptable on platforms where it's not implemented
            }
            Err(e) => {
                panic!("GetMachineId failed unexpectedly: {:?}", e);
            }
        }

        // Ping on registered path should still work
        let proxy = PeerProxy::new(
            &client_conn,
            "org.zbus.PeerArbitraryPathTest",
            "/registered",
        )
        .await
        .expect("Failed to create PeerProxy");
        let result = proxy.ping().await;
        assert!(result.is_ok(), "Ping on registered path should succeed");

        // Unknown method on Peer interface should fail properly
        let result = client_conn
            .call_method(
                Some("org.zbus.PeerArbitraryPathTest"),
                "/unregistered",
                Some("org.freedesktop.DBus.Peer"),
                "UnknownMethod",
                &(),
            )
            .await;
        assert!(result.is_err(), "Unknown method should fail");
    }
}
