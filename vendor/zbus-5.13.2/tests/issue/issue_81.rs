use test_log::test;

use zvariant::OwnedObjectPath;

#[test]
#[ignore]
fn issue_81() {
    use zbus::proxy;
    use zvariant::{OwnedValue, Type};

    #[derive(
        Debug, PartialEq, Eq, Clone, Type, OwnedValue, serde::Serialize, serde::Deserialize,
    )]
    pub struct DbusPath {
        id: String,
        path: OwnedObjectPath,
    }

    #[proxy(assume_defaults = true)]
    trait Session {
        #[zbus(property)]
        fn sessions_tuple(&self) -> zbus::Result<(String, String)>;

        #[zbus(property)]
        fn sessions_struct(&self) -> zbus::Result<DbusPath>;
    }
}
