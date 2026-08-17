use ntest::timeout;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use zbus::{connection::Builder, proxy::Defaults, zvariant::Type};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type)]
struct SingleFieldStruct {
    field: u32,
}

struct Iface;

#[zbus::interface(
    name = "org.zbus.Issue1015",
    proxy(
        default_path = "/org/zbus/Issue1015",
        default_service = "org.zbus.Issue1015",
    )
)]
impl Iface {
    fn return_struct(&mut self) -> SingleFieldStruct {
        SingleFieldStruct { field: 3 }
    }
}

#[instrument]
#[test]
#[timeout(15000)]
fn issue_1015() {
    // Reproducer for issue #1015, where a regression from signature overhaul caused inconsistency
    // between the client and server on how the body signature of a struct with a signle field is
    // handled.
    zbus::block_on(issue_1015_async());
}

async fn issue_1015_async() {
    let conn = Builder::session()
        .unwrap()
        .serve_at(IfaceProxy::PATH.as_ref().unwrap(), Iface)
        .unwrap()
        .name(IfaceProxy::DESTINATION.clone().unwrap())
        .unwrap()
        .build()
        .await
        .unwrap();

    let proxy = IfaceProxy::new(&conn).await.unwrap();
    let _ = proxy.return_struct().await.unwrap();
}
