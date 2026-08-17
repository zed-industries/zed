use async_broadcast::{Receiver, broadcast};

use futures_util::StreamExt;
use ntest::timeout;
use test_log::test;
use zbus::block_on;

use zbus::object_server::SignalEmitter;

#[test]
#[timeout(15000)]
fn issue_173() {
    // Tests the fix for https://github.com/z-galaxy/zbus/issues/173
    //
    // The issue is caused by proxy not keeping track of its destination's owner changes
    // (service restart) and failing to receive signals as a result.
    let (tx, rx) = broadcast(16);
    let child = std::thread::spawn(move || {
        block_on(async move {
            let conn = zbus::Connection::session().await.unwrap();
            #[zbus::proxy(
                interface = "org.freedesktop.zbus.ComeAndGo",
                default_service = "org.freedesktop.zbus.ComeAndGo",
                default_path = "/org/freedesktop/zbus/ComeAndGo"
            )]
            trait ComeAndGo {
                #[zbus(signal)]
                fn the_signal(&self) -> zbus::Result<()>;
            }

            let proxy = ComeAndGoProxy::new(&conn).await.unwrap();
            let mut signals = proxy.receive_the_signal().await.unwrap().take(2);
            tx.broadcast_direct(()).await.unwrap();

            // We receive two signals, each time from different unique names. W/o the fix for
            // issue#173, the second iteration hangs.
            while signals.next().await.is_some() {
                tx.broadcast_direct(()).await.unwrap();
            }
        })
    });

    zbus::block_on(issue_173_async(rx));

    child.join().unwrap();
}

async fn issue_173_async(mut rx: Receiver<()>) {
    struct ComeAndGo;
    #[zbus::interface(name = "org.freedesktop.zbus.ComeAndGo")]
    impl ComeAndGo {
        #[zbus(signal)]
        async fn the_signal(signal_emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
    }

    rx.recv().await.unwrap();
    for _ in 0..2 {
        let conn = zbus::connection::Builder::session()
            .unwrap()
            .serve_at("/org/freedesktop/zbus/ComeAndGo", ComeAndGo)
            .unwrap()
            .name("org.freedesktop.zbus.ComeAndGo")
            .unwrap()
            .build()
            .await
            .unwrap();

        let iface_ref = conn
            .object_server()
            .interface::<_, ComeAndGo>("/org/freedesktop/zbus/ComeAndGo")
            .await
            .unwrap();
        ComeAndGo::the_signal(iface_ref.signal_emitter())
            .await
            .unwrap();

        rx.recv().await.unwrap();

        // Now we release the name ownership to use a different connection (i-e new unique
        // name).
        conn.release_name("org.freedesktop.zbus.ComeAndGo")
            .await
            .unwrap();
    }
}
