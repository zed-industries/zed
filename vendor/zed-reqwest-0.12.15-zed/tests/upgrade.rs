#![cfg(not(target_arch = "wasm32"))]
#![cfg(not(feature = "rustls-tls-manual-roots-no-provider"))]
mod support;
use support::server;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn http_upgrade() {
    let server = server::http(move |req| {
        assert_eq!(req.method(), "GET");
        assert_eq!(req.headers()["connection"], "upgrade");
        assert_eq!(req.headers()["upgrade"], "foobar");

        tokio::spawn(async move {
            let mut upgraded = hyper_util::rt::TokioIo::new(hyper::upgrade::on(req).await.unwrap());

            let mut buf = vec![0; 7];
            upgraded.read_exact(&mut buf).await.unwrap();
            assert_eq!(buf, b"foo=bar");

            upgraded.write_all(b"bar=foo").await.unwrap();
        });

        async {
            http::Response::builder()
                .status(http::StatusCode::SWITCHING_PROTOCOLS)
                .header(http::header::CONNECTION, "upgrade")
                .header(http::header::UPGRADE, "foobar")
                .body(reqwest::Body::default())
                .unwrap()
        }
    });

    let res = reqwest::Client::builder()
        .build()
        .unwrap()
        .get(format!("http://{}", server.addr()))
        .header(http::header::CONNECTION, "upgrade")
        .header(http::header::UPGRADE, "foobar")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), http::StatusCode::SWITCHING_PROTOCOLS);
    let mut upgraded = res.upgrade().await.unwrap();

    upgraded.write_all(b"foo=bar").await.unwrap();

    let mut buf = vec![];
    upgraded.read_to_end(&mut buf).await.unwrap();
    assert_eq!(buf, b"bar=foo");
}
