use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use tracing::{debug, instrument};
use zbus::{
    connection::Connection,
    interface,
    message::{self, Header},
    object_server::{ObjectServer, ResponseDispatchNotifier, SignalEmitter},
};
use zvariant::{Optional, OwnedValue, Value};

use super::types::{ArgStructTest, IP4Adress, MyIfaceError, NextAction, RefType};

#[derive(Debug)]
pub struct MyIface {
    next_tx: Sender<NextAction>,
    count: u32,
    emits_changed_default: u32,
    emits_changed_true: u32,
    emits_changed_invalidates: u32,
    emits_changed_const: u32,
    emits_changed_false: u32,
    r#let: u32,
}

impl MyIface {
    pub fn new(next_tx: Sender<NextAction>) -> Self {
        Self {
            next_tx,
            count: 0,
            emits_changed_default: 0,
            emits_changed_true: 0,
            emits_changed_invalidates: 0,
            emits_changed_const: 0,
            emits_changed_false: 0,
            r#let: 0,
        }
    }
}

#[interface(
    interface = "org.freedesktop.MyIface",
    proxy(gen_blocking = true, assume_defaults = true, visibility = "pub(crate)")
)]
impl MyIface {
    #[instrument]
    async fn ping(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) -> u32 {
        self.count += 1;
        if self.count % 3 == 0 {
            emitter
                .alert_count(self.count)
                .await
                .expect("Failed to emit signal");
            debug!("emitted `AlertCount` signal.");
        } else {
            debug!("Didn't emit `AlertCount` signal.");
        }
        self.count
    }

    #[instrument]
    async fn quit(&self) {
        debug!("Client asked to quit.");
        self.next_tx.send(NextAction::Quit).await.unwrap();
    }

    #[instrument]
    fn test_header(&self, #[zbus(header)] header: Header<'_>) {
        debug!("`TestHeader` called.");
        assert_eq!(header.message_type(), message::Type::MethodCall);
        assert_eq!(header.member().unwrap(), "TestHeader");
    }

    #[instrument]
    fn test_error(&self) -> zbus::fdo::Result<()> {
        debug!("`TestError` called.");
        Err(zbus::fdo::Error::Failed("error raised".to_string()))
    }

    #[instrument]
    fn test_custom_error(&self) -> Result<(), MyIfaceError> {
        debug!("`TestCustomError` called.");
        Err(MyIfaceError::SomethingWentWrong("oops".to_string()))
    }

    #[instrument]
    fn test_single_struct_arg(
        &self,
        arg: ArgStructTest,
        #[zbus(header)] header: Header<'_>,
    ) -> zbus::fdo::Result<()> {
        debug!("`TestSingleStructArg` called.");
        assert_eq!(header.signature(), "(is)");
        assert_eq!(arg.foo, 1);
        assert_eq!(arg.bar, "TestString");

        Ok(())
    }

    #[instrument]
    fn test_single_struct_ret(&self) -> zbus::fdo::Result<ArgStructTest> {
        debug!("`TestSingleStructRet` called.");
        Ok(ArgStructTest {
            foo: 42,
            bar: String::from("Meaning of life"),
        })
    }

    #[instrument]
    #[zbus(out_args("foo", "bar"))]
    fn test_multi_ret(&self) -> zbus::fdo::Result<(i32, String)> {
        debug!("`TestMultiRet` called.");
        Ok((42, String::from("Meaning of life")))
    }

    #[instrument]
    #[zbus(out_args("SomeOutput"))]
    fn test_single_ret_with_name(&self) -> zbus::fdo::Result<String> {
        debug!("`TestSingleRetWithName` called.");
        Ok(String::from("test output"))
    }

    #[instrument]
    fn test_response_notify(
        &self,
        #[zbus(connection)] conn: &Connection,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<ResponseDispatchNotifier<String>> {
        debug!("`TestResponseNotify` called.");
        let (response, listener) = ResponseDispatchNotifier::new(String::from("Meaning of life"));
        let emitter = emitter.to_owned();
        conn.executor()
            .spawn(
                async move {
                    listener.await;

                    Self::test_response_notified(emitter).await.unwrap();
                },
                "TestResponseNotify",
            )
            .detach();

        Ok(response)
    }

    #[zbus(signal)]
    async fn test_response_notified(emitter: SignalEmitter<'_>) -> zbus::Result<()>;

    #[instrument]
    async fn test_hashmap_return(&self) -> zbus::fdo::Result<HashMap<String, String>> {
        debug!("`TestHashmapReturn` called.");
        let mut map = HashMap::new();
        map.insert("hi".into(), "hello".into());
        map.insert("bye".into(), "now".into());

        Ok(map)
    }

    #[instrument]
    async fn create_obj(&self, key: &str) {
        debug!("`CreateObj` called.");
        self.next_tx
            .send(NextAction::CreateObj(key.into()))
            .await
            .unwrap();
    }

    #[instrument]
    async fn create_obj_inside(
        &self,
        #[zbus(object_server)] object_server: &ObjectServer,
        key: String,
    ) {
        debug!("`CreateObjInside` called.");
        object_server
            .at(
                format!("/zbus/test/{key}"),
                MyIface::new(self.next_tx.clone()),
            )
            .await
            .unwrap();
    }

    #[instrument]
    async fn destroy_obj(&self, key: &str) {
        debug!("`DestroyObj` called.");
        self.next_tx
            .send(NextAction::DestroyObj(key.into()))
            .await
            .unwrap();
    }

    #[instrument]
    async fn r#type(&self) -> zbus::fdo::Result<String> {
        debug!("`r#type` called.");
        Ok("r# prefix method works".into())
    }

    #[instrument]
    async fn raw_identifier_parameter(&self, r#type: &str) -> zbus::fdo::Result<String> {
        debug!("`raw_identifier_parameter` called.");
        let t = r#type;
        Ok(format!("type: {t}"))
    }

    #[cfg(feature = "option-as-array")]
    #[instrument]
    async fn optional_args(&self, arg: Option<&str>) -> zbus::fdo::Result<Option<String>> {
        debug!("`OptionalArgs` called.");
        Ok(arg.map(|s| format!("Hello {}", s)))
    }

    #[instrument]
    #[zbus(property)]
    fn set_count(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`Count` setter called.");
        if val == 42 {
            return Err(zbus::fdo::Error::InvalidArgs("Tsss tsss!".to_string()));
        }
        self.count = val;
        Ok(())
    }

    #[instrument]
    #[zbus(property)]
    fn count(&self) -> u32 {
        debug!("`Count` getter called.");
        self.count
    }

    // This is a write-only property.
    // It actually is equivalent to `set_count` above.
    #[instrument]
    #[zbus(property)]
    fn set_count2(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`Count` setter called from a write-only property.");
        self.set_count(val)
    }

    #[instrument]
    #[zbus(property)]
    fn test_header_prop(
        &self,
        #[zbus(header)] header: Option<Header<'_>>,
        #[zbus(connection)] connection: &Connection,
        #[zbus(object_server)] object_server: &ObjectServer,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> bool {
        debug!(
            "`TestHeaderProp` getter called, header: {:?}, connection: {:?}, object_server: {:?}, emitter: {:?}",
            header, connection, object_server, emitter
        );
        header.is_some()
    }

    #[instrument]
    #[zbus(property)]
    fn set_test_header_prop(
        &self,
        value: bool,
        #[zbus(header)] header: Option<Header<'_>>,
        #[zbus(connection)] connection: &Connection,
        #[zbus(object_server)] object_server: &ObjectServer,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) {
        debug!(
            "`TestHeaderProp` setter called, value: {}, header: {:?}, connection: {:?}, object_server: {:?}, emitter: {:?}",
            value, header, connection, object_server, emitter
        );
        assert!(header.is_some());
    }

    #[instrument]
    #[zbus(property)]
    async fn hash_map(&self) -> HashMap<String, String> {
        debug!("`HashMap` getter called.");
        self.test_hashmap_return().await.unwrap()
    }

    #[instrument]
    #[zbus(property)]
    async fn fail_property(&self) -> zbus::fdo::Result<u32> {
        Err(zbus::fdo::Error::UnknownProperty(
            "FailProperty".to_string(),
        ))
    }

    #[instrument]
    #[zbus(property)]
    fn optional_property(&self) -> Optional<u32> {
        debug!("`OptionalAsProp` getter called.");
        Some(42).into()
    }

    #[instrument]
    #[zbus(property)]
    fn address_data(&self) -> IP4Adress {
        debug!("`AddressData` getter called.");
        IP4Adress {
            address: "127.0.0.1".to_string(),
            prefix: 1234,
        }
    }

    #[instrument]
    #[zbus(property)]
    fn set_address_data(&self, addr: IP4Adress) {
        debug!("`AddressData` setter called with {:?}", addr);
    }

    // On the bus, this should return the same value as address_data above. We want to test if
    // this works both ways.
    #[instrument]
    #[zbus(property)]
    fn address_data2(&self) -> HashMap<String, OwnedValue> {
        debug!("`AddressData2` getter called.");
        let mut map = HashMap::new();
        map.insert(
            "address".into(),
            Value::from("127.0.0.1").try_into().unwrap(),
        );
        map.insert("prefix".into(), 1234u32.into());

        map
    }

    #[instrument]
    #[zbus(property)]
    fn str_prop(&self) -> String {
        "Hello".to_string()
    }

    #[instrument]
    #[zbus(property)]
    fn set_str_prop(&self, str_prop: &str) {
        debug!("`SetStrRef` called with {:?}", str_prop);
    }

    #[instrument]
    #[zbus(property)]
    fn ref_prop(&self) -> RefType<'_> {
        RefType {
            field1: "Hello".into(),
        }
    }

    #[instrument]
    #[zbus(property)]
    fn set_ref_prop(&self, ref_type: RefType<'_>) {
        debug!("`SetRefType` called with {:?}", ref_type);
    }

    #[instrument]
    #[zbus(proxy(no_reply))]
    fn test_no_reply(&self, #[zbus(header)] header: Header<'_>) {
        debug!("`TestNoReply` called");
        assert_eq!(header.message_type(), zbus::message::Type::MethodCall);
        assert!(
            header
                .primary()
                .flags()
                .contains(zbus::message::Flags::NoReplyExpected)
        );
    }

    #[instrument]
    #[zbus(proxy(no_autostart))]
    fn test_no_autostart(&self, #[zbus(header)] header: Header<'_>) {
        debug!("`TestNoAutostart` called");
        assert_eq!(header.message_type(), zbus::message::Type::MethodCall);
        assert!(
            header
                .primary()
                .flags()
                .contains(zbus::message::Flags::NoAutoStart)
        );
    }

    #[instrument]
    #[zbus(proxy(allow_interactive_auth))]
    fn test_interactive_auth(&self, #[zbus(header)] header: Header<'_>) {
        debug!("`TestInteractiveAuth` called");
        assert_eq!(header.message_type(), zbus::message::Type::MethodCall);
        assert!(
            header
                .primary()
                .flags()
                .contains(zbus::message::Flags::AllowInteractiveAuth)
        );
    }

    #[zbus(signal)]
    pub async fn alert_count(emitter: &SignalEmitter<'_>, val: u32) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn r#match(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[instrument]
    #[zbus(property)]
    fn emits_changed_default(&self) -> u32 {
        debug!("`EmitsChangedDefault` getter called.");
        self.emits_changed_default
    }

    #[instrument]
    #[zbus(property)]
    fn set_emits_changed_default(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`EmitsChangedDefault` setter called.");
        self.emits_changed_default = val;
        Ok(())
    }

    #[instrument]
    #[zbus(property(emits_changed_signal = "true"))]
    fn emits_changed_true(&self) -> u32 {
        debug!("`EmitsChangedTrue` getter called.");
        self.emits_changed_true
    }

    #[instrument]
    #[zbus(property)]
    fn set_emits_changed_true(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`EmitsChangedTrue` setter called.");
        self.emits_changed_true = val;
        Ok(())
    }

    #[instrument]
    #[zbus(property(emits_changed_signal = "invalidates"))]
    fn emits_changed_invalidates(&self) -> u32 {
        debug!("`EmitsChangedInvalidates` getter called.");
        self.emits_changed_invalidates
    }

    #[instrument]
    #[zbus(property)]
    fn set_emits_changed_invalidates(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`EmitsChangedInvalidates` setter called.");
        self.emits_changed_invalidates = val;
        Ok(())
    }

    #[instrument]
    #[zbus(property(emits_changed_signal = "const"))]
    fn emits_changed_const(&self) -> u32 {
        debug!("`EmitsChangedConst` getter called.");
        self.emits_changed_const
    }

    #[instrument]
    #[zbus(property)]
    fn set_emits_changed_const(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`EmitsChangedConst` setter called.");
        self.emits_changed_const = val;
        Ok(())
    }

    #[instrument]
    #[zbus(property(emits_changed_signal = "false"))]
    fn emits_changed_false(&self) -> u32 {
        debug!("`EmitsChangedFalse` getter called.");
        self.emits_changed_false
    }

    #[instrument]
    #[zbus(property)]
    fn set_emits_changed_false(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`EmitsChangedFalse` setter called.");
        self.emits_changed_false = val;
        Ok(())
    }

    #[instrument]
    #[zbus(property)]
    fn r#let(&self) -> u32 {
        debug!("`Let` getter called.");
        self.r#let
    }

    #[instrument]
    #[zbus(property)]
    fn set_let(&mut self, val: u32) -> zbus::fdo::Result<()> {
        debug!("`Let` setter called.");
        self.r#let = val;
        Ok(())
    }

    async fn never_return(&self) {
        debug!("`NeverReturn` called.");

        std::future::pending::<()>().await;
    }
}
