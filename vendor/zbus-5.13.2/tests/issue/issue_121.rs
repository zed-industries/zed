use test_log::test;

use zvariant::OwnedObjectPath;

// This one we just want to see if it builds, no need to run it. For details see:
//
// https://github.com/z-galaxy/zbus/issues/121
#[test]
#[ignore]
fn issue_121() {
    use zbus::proxy;

    #[proxy(interface = "org.freedesktop.IBus", assume_defaults = true)]
    trait IBus {
        /// CurrentInputContext property
        #[zbus(property)]
        fn current_input_context(&self) -> zbus::Result<OwnedObjectPath>;

        /// Engines property
        #[zbus(property)]
        fn engines(&self) -> zbus::Result<Vec<zvariant::OwnedValue>>;
    }
}
