#![cfg(not(target_arch = "wasm32"))]
mod support;
use support::server;

use std::time::Duration;

#[tokio::test]
async fn client_timeout() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| {
        async {
            // delay returning the response
            tokio::time::sleep(Duration::from_secs(2)).await;
            http::Response::default()
        }
    });

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .unwrap();

    let url = format!("http://{}/slow", server.addr());

    let res = client.get(&url).send().await;

    let err = res.unwrap_err();

    assert!(err.is_timeout());
    assert_eq!(err.url().map(|u| u.as_str()), Some(url.as_str()));
}

#[tokio::test]
async fn request_timeout() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| {
        async {
            // delay returning the response
            tokio::time::sleep(Duration::from_secs(2)).await;
            http::Response::default()
        }
    });

    let client = reqwest::Client::builder().build().unwrap();

    let url = format!("http://{}/slow", server.addr());

    let res = client
        .get(&url)
        .timeout(Duration::from_millis(500))
        .send()
        .await;

    let err = res.unwrap_err();

    if cfg!(not(target_arch = "wasm32")) {
        assert!(err.is_timeout() && !err.is_connect());
    } else {
        assert!(err.is_timeout());
    }
    assert_eq!(err.url().map(|u| u.as_str()), Some(url.as_str()));
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn connect_timeout() {
    let _ = env_logger::try_init();

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(100))
        .build()
        .unwrap();

    let url = "http://10.255.255.1:81/slow";

    let res = client
        .get(url)
        .timeout(Duration::from_millis(1000))
        .send()
        .await;

    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn connect_many_timeout_succeeds() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });
    let port = server.addr().port();

    let client = reqwest::Client::builder()
        .resolve_to_addrs(
            "many_addrs",
            &["10.255.255.1:81".parse().unwrap(), server.addr()],
        )
        .connect_timeout(Duration::from_millis(100))
        .build()
        .unwrap();

    let url = format!("http://many_addrs:{port}/eventual");

    let _res = client
        .get(url)
        .timeout(Duration::from_millis(1000))
        .send()
        .await
        .unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn connect_many_timeout() {
    let _ = env_logger::try_init();

    let client = reqwest::Client::builder()
        .resolve_to_addrs(
            "many_addrs",
            &[
                "10.255.255.1:81".parse().unwrap(),
                "10.255.255.2:81".parse().unwrap(),
            ],
        )
        .connect_timeout(Duration::from_millis(100))
        .build()
        .unwrap();

    let url = "http://many_addrs:81/slow".to_string();

    let res = client
        .get(url)
        .timeout(Duration::from_millis(1000))
        .send()
        .await;

    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[tokio::test]
async fn response_timeout() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| {
        async {
            // immediate response, but delayed body
            let body = hyper::Body::wrap_stream(futures_util::stream::once(async {
                tokio::time::sleep(Duration::from_secs(2)).await;
                Ok::<_, std::convert::Infallible>("Hello")
            }));

            http::Response::new(body)
        }
    });

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(500))
        .no_proxy()
        .build()
        .unwrap();

    let url = format!("http://{}/slow", server.addr());
    let res = client.get(&url).send().await.expect("Failed to get");
    let body = res.text().await;

    let err = body.unwrap_err();

    assert!(err.is_timeout());
}

/// Tests that internal client future cancels when the oneshot channel
/// is canceled.
#[cfg(feature = "blocking")]
#[test]
fn timeout_closes_connection() {
    let _ = env_logger::try_init();

    // Make Client drop *after* the Server, so the background doesn't
    // close too early.
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .unwrap();

    let server = server::http(move |_req| {
        async {
            // delay returning the response
            tokio::time::sleep(Duration::from_secs(2)).await;
            http::Response::default()
        }
    });

    let url = format!("http://{}/closes", server.addr());
    let err = client.get(&url).send().unwrap_err();

    assert!(err.is_timeout());
    assert_eq!(err.url().map(|u| u.as_str()), Some(url.as_str()));
}

#[cfg(feature = "blocking")]
#[test]
fn timeout_blocking_request() {
    let _ = env_logger::try_init();

    // Make Client drop *after* the Server, so the background doesn't
    // close too early.
    let client = reqwest::blocking::Client::builder().build().unwrap();

    let server = server::http(move |_req| {
        async {
            // delay returning the response
            tokio::time::sleep(Duration::from_secs(2)).await;
            http::Response::default()
        }
    });

    let url = format!("http://{}/closes", server.addr());
    let err = client
        .get(&url)
        .timeout(Duration::from_millis(500))
        .send()
        .unwrap_err();

    assert!(err.is_timeout());
    assert_eq!(err.url().map(|u| u.as_str()), Some(url.as_str()));
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_timeout_body() {
    let _ = env_logger::try_init();

    let client = reqwest::blocking::Client::builder()
        // this should be overridden
        .connect_timeout(Duration::from_millis(200))
        // this should be overridden
        .timeout(Duration::from_millis(200))
        .build()
        .unwrap();

    let server = server::http(move |_req| {
        async {
            // immediate response, but delayed body
            let body = hyper::Body::wrap_stream(futures_util::stream::once(async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                Ok::<_, std::convert::Infallible>("Hello")
            }));

            http::Response::new(body)
        }
    });

    let url = format!("http://{}/closes", server.addr());
    let res = client
        .get(&url)
        // longer than client timeout
        .timeout(Duration::from_secs(5))
        .send()
        .expect("get response");

    let text = res.text().unwrap();
    assert_eq!(text, "Hello");
}

#[cfg(feature = "blocking")]
#[test]
fn write_timeout_large_body() {
    let _ = env_logger::try_init();
    let body = vec![b'x'; 20_000];
    let len = 8192;

    // Make Client drop *after* the Server, so the background doesn't
    // close too early.
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .unwrap();

    let server = server::http(move |_req| {
        async {
            // delay returning the response
            tokio::time::sleep(Duration::from_secs(2)).await;
            http::Response::default()
        }
    });

    let cursor = std::io::Cursor::new(body);
    let url = format!("http://{}/write-timeout", server.addr());
    let err = client
        .post(&url)
        .body(reqwest::blocking::Body::sized(cursor, len as u64))
        .send()
        .unwrap_err();

    assert!(err.is_timeout());
    assert_eq!(err.url().map(|u| u.as_str()), Some(url.as_str()));
}
