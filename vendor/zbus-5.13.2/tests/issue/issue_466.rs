use test_log::test;

#[test]
#[ignore]
fn issue_466() {
    #[zbus::proxy(interface = "org.Some.Thing1", assume_defaults = true)]
    trait MyGreeter {
        fn foo(
            &self,
            arg: &(u32, zbus::zvariant::Value<'_>),
        ) -> zbus::Result<(u32, zbus::zvariant::OwnedValue)>;

        #[zbus(property)]
        fn bar(&self) -> zbus::Result<(u32, zbus::zvariant::OwnedValue)>;
    }
}
