use event_listener::Event;
use futures_util::{StreamExt, TryStreamExt};
use std::convert::TryInto;
use tracing::{debug, instrument};
use zbus::{
    Connection, Error, Message, MessageStream,
    fdo::{ObjectManagerProxy, PropertiesProxy},
    message,
    proxy::CacheProperties,
};
use zvariant::Value;

use super::{
    helpers::{check_hash_map, check_ipv4_address, check_ipv4_address_hashmap},
    iface::MyIfaceProxy,
    types::{ArgStructTest, IP4Adress, MyIfaceError},
};

#[instrument]
pub async fn my_iface_test(conn: Connection, event: Event) -> zbus::Result<u32> {
    debug!("client side starting..");
    // Use low-level API for `TestResponseNotify` because we need to ensure that the signal is
    // always received after the response.
    let mut stream = MessageStream::from(&conn);
    let method = Message::method_call("/org/freedesktop/MyService", "TestResponseNotify")?
        .interface("org.freedesktop.MyIface")?
        .destination("org.freedesktop.MyService")?
        .build(&())?;
    let serial = method.primary_header().serial_num();
    conn.send(&method).await?;
    let mut method_returned = false;
    let mut signal_received = false;
    while !method_returned && !signal_received {
        let msg = stream.try_next().await?.unwrap();

        let hdr = msg.header();
        if hdr.message_type() == message::Type::MethodReturn && hdr.reply_serial() == Some(serial) {
            assert!(!signal_received);
            method_returned = true;
        } else if hdr.message_type() == message::Type::Signal
            && hdr.interface().unwrap() == "org.freedesktop.MyService"
            && hdr.member().unwrap() == "TestResponseNotified"
        {
            assert!(method_returned);
            signal_received = true;
        }
    }
    drop(stream);

    let root_introspect_proxy = zbus::fdo::IntrospectableProxy::builder(&conn)
        .destination("org.freedesktop.MyService")?
        .path("/")?
        .build()
        .await?;
    debug!("Created: {:?}", root_introspect_proxy);

    let root_xml = root_introspect_proxy.introspect().await?;
    let root_node = zbus_xml::Node::from_reader(root_xml.as_bytes())
        .map_err(|e| Error::Failure(e.to_string()))?;
    let mut node = &root_node;
    for name in ["org", "freedesktop", "MyService"] {
        node = node
            .nodes()
            .iter()
            .find(|&n| n.name().is_some_and(|n| n == name))
            .expect("Child node not exist");
    }
    let my_iface = node
        .interfaces()
        .iter()
        .find(|i| i.name() == "org.freedesktop.MyIface")
        .unwrap();
    // Test if the r# prefix for the keyword was removed
    assert!(my_iface.methods().iter().any(|m| m.name() == "Type"));
    assert!(my_iface.properties().iter().any(|p| p.name() == "Let"));
    assert!(my_iface.signals().iter().any(|s| s.name() == "Match"));
    assert_eq!(
        my_iface
            .methods()
            .iter()
            .find(|m| m.name() == "RawIdentifierParameter")
            .unwrap()
            .args()
            .iter()
            .next()
            .unwrap()
            .name()
            .unwrap(),
        "type"
    );

    let proxy = MyIfaceProxy::builder(&conn)
        .destination("org.freedesktop.MyService")?
        .path("/org/freedesktop/MyService")?
        // the server isn't yet running
        .cache_properties(CacheProperties::No)
        .build()
        .await?;
    debug!("Created: {:?}", proxy);

    // First let's call a non-existent method. It should immediately fail.
    // There was a regression where object server would just not reply in this case:
    // https://github.com/z-galaxy/zbus/issues/905
    assert!(
        proxy
            .inner()
            .call::<_, _, ()>("NonExistantMethod", &())
            .await
            .is_err()
    );

    let props_proxy = PropertiesProxy::builder(&conn)
        .destination("org.freedesktop.MyService")?
        .path("/org/freedesktop/MyService")?
        .build()
        .await?;
    debug!("Created: {:?}", props_proxy);

    let mut props_changed_stream = props_proxy.receive_properties_changed().await?;
    debug!("Created: {:?}", props_changed_stream);
    event.notify(1);
    debug!("Notified service that client is ready");

    match props_changed_stream.next().await {
        Some(changed) => {
            assert_eq!(
                *changed.args()?.changed_properties().keys().next().unwrap(),
                "Count"
            );
        }
        None => panic!(""),
    };
    drop(props_changed_stream);

    proxy.ping().await?;
    proxy.set_test_header_prop(true).await?;
    assert!(proxy.test_header_prop().await?);
    assert_eq!(proxy.count().await?, 1);
    assert_eq!(proxy.cached_count()?, None);

    // Test that properties can be set.
    proxy.set_count(8888).await?;
    assert_eq!(proxy.count().await?, 8888);
    proxy.set_count2(1).await?;
    assert_eq!(proxy.count().await?, 1);

    proxy.test_header().await?;
    proxy
        .test_single_struct_arg(ArgStructTest {
            foo: 1,
            bar: "TestString".into(),
        })
        .await?;

    proxy.test_error().await.unwrap_err();
    assert_eq!(
        proxy.test_custom_error().await.unwrap_err(),
        MyIfaceError::SomethingWentWrong("oops".to_string())
    );

    check_hash_map(proxy.test_hashmap_return().await?);
    check_hash_map(proxy.hash_map().await?);
    proxy
        .set_address_data(IP4Adress {
            address: "localhost".to_string(),
            prefix: 1234,
        })
        .await?;
    proxy.set_str_prop("This is an str ref").await?;
    check_ipv4_address(proxy.address_data().await?);
    check_ipv4_address_hashmap(proxy.address_data2().await?);

    proxy.test_no_reply().await?;
    proxy.test_no_autostart().await?;
    proxy.test_interactive_auth().await?;

    assert_eq!(proxy.r#let().await?, 0);
    proxy.set_let(1).await?;
    assert_eq!(proxy.r#let().await?, 1);

    assert_eq!(
        proxy.raw_identifier_parameter("param").await?,
        "type: param",
    );

    let err = proxy.fail_property().await;
    assert_eq!(
        err.unwrap_err(),
        zbus::Error::FDO(Box::new(zbus::fdo::Error::UnknownProperty(
            "FailProperty".into()
        ))),
    );

    assert_eq!(proxy.optional_property().await?, Some(42).into());

    let xml = proxy.inner().introspect().await?;
    debug!("Introspection: {}", xml);
    let node =
        zbus_xml::Node::from_reader(xml.as_bytes()).map_err(|e| Error::Failure(e.to_string()))?;
    let ifaces = node.interfaces();
    let iface = ifaces
        .iter()
        .find(|i| i.name() == "org.freedesktop.MyIface")
        .unwrap();
    let methods = iface.methods();
    for method in methods {
        if method.name() != "TestSingleStructRet"
            && method.name() != "TestMultiRet"
            && method.name() != "TestSingleRetWithName"
        {
            continue;
        }
        let args = method.args();
        let mut out_args = args
            .iter()
            .filter(|a| a.direction().unwrap() == zbus_xml::ArgDirection::Out);

        if method.name() == "TestSingleStructRet" {
            assert_eq!(args.len(), 1);
            assert_eq!(out_args.next().unwrap().ty(), "(is)");
            assert!(out_args.next().is_none());
        } else if method.name() == "TestMultiRet" {
            assert_eq!(args.len(), 2);
            let foo = out_args.find(|a| a.name() == Some("foo")).unwrap();
            assert_eq!(foo.ty(), "i");
            let bar = out_args.find(|a| a.name() == Some("bar")).unwrap();
            assert_eq!(bar.ty(), "s");
        } else if method.name() == "TestSingleRetWithName" {
            // Test that single output with out_args attribute gets the name
            assert_eq!(args.len(), 1);
            let out_arg = out_args.next().unwrap();
            assert_eq!(out_arg.name(), Some("SomeOutput"));
            assert_eq!(out_arg.ty(), "s");
            assert!(out_args.next().is_none());
        }
    }
    // build-time check to see if macro is doing the right thing.
    let _ = proxy.test_single_struct_ret().await?.foo;
    let _ = proxy.test_multi_ret().await?.1;
    let _ = proxy.test_single_ret_with_name().await?;

    let val = proxy.ping().await?;

    let obj_manager_proxy = ObjectManagerProxy::builder(&conn)
        .destination("org.freedesktop.MyService")?
        .path("/zbus/test")?
        .build()
        .await?;
    debug!("Created: {:?}", obj_manager_proxy);
    let mut ifaces_added_stream = obj_manager_proxy.receive_interfaces_added().await?;
    debug!("Created: {:?}", ifaces_added_stream);

    // Must process in parallel, so the stream listener does not block receiving
    // the method return message.
    let (ifaces_added, _) = futures_util::future::join(
        async {
            let ret = ifaces_added_stream.next().await.unwrap();
            drop(ifaces_added_stream);
            ret
        },
        async {
            proxy.create_obj("MyObj").await.unwrap();
        },
    )
    .await;

    #[cfg(feature = "option-as-array")]
    {
        assert!(proxy.optional_args(None).await.unwrap().is_none());
        assert_eq!(
            proxy.optional_args(Some("🚌")).await.unwrap().unwrap(),
            "Hello 🚌",
        );
    }

    assert_eq!(ifaces_added.args()?.object_path(), "/zbus/test/MyObj");
    let args = ifaces_added.args()?;
    let ifaces = args.interfaces_and_properties();
    let _ = ifaces.get("org.freedesktop.MyIface").unwrap();
    // TODO: Check if the properties are correct.

    // issue#207: interface panics on incorrect number of args.
    assert!(proxy.inner().call_method("CreateObj", &()).await.is_err());

    let my_obj_proxy = MyIfaceProxy::builder(&conn)
        .destination("org.freedesktop.MyService")?
        .path("/zbus/test/MyObj")?
        .build()
        .await?;
    debug!("Created: {:?}", my_obj_proxy);
    let mut stream = my_obj_proxy.receive_count_changed().await;
    // Calling this after creating the stream was panicking if the property doesn't get cached
    // before the call (MR !460).
    my_obj_proxy.cached_count()?;
    assert_eq!(my_obj_proxy.count().await?, 0);
    assert_eq!(my_obj_proxy.cached_count()?, Some(0));
    assert_eq!(
        my_obj_proxy.inner().cached_property_raw("Count").as_deref(),
        Some(&Value::from(0u32))
    );
    my_obj_proxy.ping().await?;

    // The stream should first yield the current value.
    assert_eq!(stream.next().await.unwrap().get().await?, 0);
    for test_case in [55, 66, 77, 88] {
        let (value, _) = futures_util::try_join!(
            async { stream.next().await.unwrap().get().await },
            my_obj_proxy.set_count(test_case)
        )?;
        assert_eq!(test_case, value);
    }

    let mut ifaces_removed_stream = obj_manager_proxy.receive_interfaces_removed().await?;
    debug!("Created: {:?}", ifaces_removed_stream);
    // Must process in parallel, so the stream listener does not block receiving
    // the method return message.
    let (ifaces_removed, _) = futures_util::future::join(
        async {
            let ret = ifaces_removed_stream.next().await.unwrap();
            drop(ifaces_removed_stream);
            ret
        },
        async {
            proxy.destroy_obj("MyObj").await.unwrap();
        },
    )
    .await;

    let args = ifaces_removed.args()?;
    assert_eq!(args.object_path(), "/zbus/test/MyObj");
    assert_eq!(args.interfaces().as_ref(), &["org.freedesktop.MyIface"]);

    assert!(my_obj_proxy.inner().introspect().await.is_err());
    assert!(my_obj_proxy.ping().await.is_err());

    // Make sure methods modifying the ObjectServer can be called without
    // deadlocks.
    proxy
        .inner()
        .call_method("CreateObjInside", &("CreatedInside"))
        .await?;
    let created_inside_proxy = MyIfaceProxy::builder(&conn)
        .destination("org.freedesktop.MyService")?
        .path("/zbus/test/CreatedInside")?
        .build()
        .await?;
    created_inside_proxy.ping().await?;
    proxy.destroy_obj("CreatedInside").await?;

    // Test that interfaces emit signals when properties change
    // according to their emits_changed_signal flags.
    let mut props_changed = props_proxy.receive_properties_changed().await?;
    let expected_property_value = 4;

    proxy
        .set_emits_changed_default(expected_property_value)
        .await?;
    let changed = props_changed.next().await.unwrap();
    let expected_property_key = "EmitsChangedDefault";
    let args = changed.args()?;
    assert!(args.invalidated_properties().is_empty());

    let changed_prop_key = *args.changed_properties().keys().next().unwrap();
    assert_eq!(changed_prop_key, expected_property_key);

    let changed_value = &args.changed_properties()[expected_property_key];
    assert_eq!(
        <&Value as TryInto<u32>>::try_into(changed_value).unwrap(),
        expected_property_value
    );
    assert!(args.invalidated_properties().is_empty());

    proxy
        .set_emits_changed_true(expected_property_value)
        .await?;
    let changed = props_changed.next().await.unwrap();
    let expected_property_key = "EmitsChangedTrue";
    let args = changed.args()?;
    assert!(args.invalidated_properties().is_empty());

    let changed_prop_key = *args.changed_properties().keys().next().unwrap();
    assert_eq!(changed_prop_key, expected_property_key);

    let changed_value = &args.changed_properties()[expected_property_key];
    assert_eq!(
        <&Value as TryInto<u32>>::try_into(changed_value).unwrap(),
        expected_property_value
    );
    assert!(args.invalidated_properties().is_empty());

    proxy
        .set_emits_changed_invalidates(expected_property_value)
        .await?;
    let changed = props_changed.next().await.unwrap();
    let expected_property_key = "EmitsChangedInvalidates";
    let args = changed.args()?;
    assert_eq!(
        args.invalidated_properties(),
        &vec![expected_property_key.to_string()]
    );
    assert!(args.changed_properties().is_empty());

    // First set a property for which we don't expect a signal
    // then set a property for which we do (and we checked above
    // that we receive it. The next item in the iter should correspond
    // to the second property we set.
    proxy
        .set_emits_changed_const(expected_property_value)
        .await?;
    proxy
        .set_emits_changed_true(expected_property_value)
        .await?;
    let changed = props_changed.next().await.unwrap();
    let unexpected_property_key = "EmitsChangedConst";
    let args = changed.args()?;
    assert_ne!(
        args.invalidated_properties(),
        &vec![unexpected_property_key.to_string()]
    );
    assert!(!args.changed_properties().is_empty());
    assert!(args.invalidated_properties().is_empty());

    proxy
        .set_emits_changed_false(expected_property_value)
        .await?;
    proxy
        .set_emits_changed_true(expected_property_value)
        .await?;
    let changed = props_changed.next().await.unwrap();
    let unexpected_property_key = "EmitsChangedFalse";
    let args = changed.args()?;
    assert_ne!(
        args.invalidated_properties(),
        &vec![unexpected_property_key.to_string()]
    );
    assert!(!args.changed_properties().is_empty());
    assert!(args.invalidated_properties().is_empty());

    // Test method timeout
    assert!(
        conn.method_timeout().is_some(),
        "method timeout should be set"
    );
    match proxy.never_return().await {
        Err(Error::InputOutput(e)) if e.kind() == std::io::ErrorKind::TimedOut => {}
        r => panic!(
            "Should produce InputOutput(TimedOut) error. Got {:?} instead",
            r
        ),
    };

    proxy.quit().await?;

    Ok(val)
}
