#![cfg(not(target_arch = "wasm32"))]
#![cfg(not(feature = "rustls-tls-manual-roots-no-provider"))]
mod support;

use std::time::Duration;

use futures_util::future::join_all;
use tower::layer::util::Identity;
use tower::limit::ConcurrencyLimitLayer;
use tower::timeout::TimeoutLayer;

use support::{delay_layer::DelayLayer, server};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn non_op_layer() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::builder()
        .connector_layer(Identity::new())
        .no_proxy()
        .build()
        .unwrap();

    let res = client.get(url).send().await;

    assert!(res.is_ok());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn non_op_layer_with_timeout() {
    let _ = env_logger::try_init();

    let client = reqwest::Client::builder()
        .connector_layer(Identity::new())
        .connect_timeout(Duration::from_millis(200))
        .no_proxy()
        .build()
        .unwrap();

    // never returns
    let url = "http://192.0.2.1:81/slow";

    let res = client.get(url).send().await;

    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn with_connect_timeout_layer_never_returning() {
    let _ = env_logger::try_init();

    let client = reqwest::Client::builder()
        .connector_layer(TimeoutLayer::new(Duration::from_millis(100)))
        .no_proxy()
        .build()
        .unwrap();

    // never returns
    let url = "http://192.0.2.1:81/slow";

    let res = client.get(url).send().await;

    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn with_connect_timeout_layer_slow() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(200)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(100)))
        .no_proxy()
        .build()
        .unwrap();

    let res = client.get(url).send().await;

    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn multiple_timeout_layers_under_threshold() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(200)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(300)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(500)))
        .connect_timeout(Duration::from_millis(200))
        .no_proxy()
        .build()
        .unwrap();

    let res = client.get(url).send().await;

    assert!(res.is_ok());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn multiple_timeout_layers_over_threshold() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(50)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(50)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(50)))
        .connect_timeout(Duration::from_millis(50))
        .no_proxy()
        .build()
        .unwrap();

    let res = client.get(url).send().await;

    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn with_concurrency_limit_layer_timeout() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(ConcurrencyLimitLayer::new(1))
        .timeout(Duration::from_millis(200))
        .pool_max_idle_per_host(0) // disable connection reuse to force resource contention on the concurrency limit semaphore
        .no_proxy()
        .build()
        .unwrap();

    // first call succeeds since no resource contention
    let res = client.get(url.clone()).send().await;
    assert!(res.is_ok());

    // 3 calls where the second two wait on the first and time out
    let mut futures = Vec::new();
    for _ in 0..3 {
        futures.push(client.clone().get(url.clone()).send());
    }

    let all_res = join_all(futures).await;

    let timed_out = all_res
        .into_iter()
        .any(|res| res.is_err_and(|err| err.is_timeout()));

    assert!(timed_out, "at least one request should have timed out");
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn with_concurrency_limit_layer_success() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(200)))
        .connector_layer(ConcurrencyLimitLayer::new(1))
        .timeout(Duration::from_millis(1000))
        .pool_max_idle_per_host(0) // disable connection reuse to force resource contention on the concurrency limit semaphore
        .no_proxy()
        .build()
        .unwrap();

    // first call succeeds since no resource contention
    let res = client.get(url.clone()).send().await;
    assert!(res.is_ok());

    // 3 calls of which all are individually below the inner timeout
    // and the sum is below outer timeout which affects the final call which waited the whole time
    let mut futures = Vec::new();
    for _ in 0..3 {
        futures.push(client.clone().get(url.clone()).send());
    }

    let all_res = join_all(futures).await;

    for res in all_res.into_iter() {
        assert!(
            res.is_ok(),
            "neither outer long timeout or inner short timeout should be exceeded"
        );
    }
}

#[cfg(feature = "blocking")]
#[test]
fn non_op_layer_blocking_client() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::blocking::Client::builder()
        .connector_layer(Identity::new())
        .build()
        .unwrap();

    let res = client.get(url).send();

    assert!(res.is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn timeout_layer_blocking_client() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::blocking::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(50)))
        .no_proxy()
        .build()
        .unwrap();

    let res = client.get(url).send();
    let err = res.unwrap_err();

    assert!(err.is_connect() && err.is_timeout());
}

#[cfg(feature = "blocking")]
#[test]
fn concurrency_layer_blocking_client_timeout() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::blocking::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(ConcurrencyLimitLayer::new(1))
        .timeout(Duration::from_millis(200))
        .pool_max_idle_per_host(0) // disable connection reuse to force resource contention on the concurrency limit semaphore
        .build()
        .unwrap();

    let res = client.get(url.clone()).send();

    assert!(res.is_ok());

    // 3 calls where the second two wait on the first and time out
    let mut join_handles = Vec::new();
    for _ in 0..3 {
        let client = client.clone();
        let url = url.clone();
        let join_handle = std::thread::spawn(move || client.get(url.clone()).send());
        join_handles.push(join_handle);
    }

    let timed_out = join_handles
        .into_iter()
        .any(|handle| handle.join().unwrap().is_err_and(|err| err.is_timeout()));

    assert!(timed_out, "at least one request should have timed out");
}

#[cfg(feature = "blocking")]
#[test]
fn concurrency_layer_blocking_client_success() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::blocking::Client::builder()
        .connector_layer(DelayLayer::new(Duration::from_millis(100)))
        .connector_layer(TimeoutLayer::new(Duration::from_millis(200)))
        .connector_layer(ConcurrencyLimitLayer::new(1))
        .timeout(Duration::from_millis(1000))
        .pool_max_idle_per_host(0) // disable connection reuse to force resource contention on the concurrency limit semaphore
        .build()
        .unwrap();

    let res = client.get(url.clone()).send();

    assert!(res.is_ok());

    // 3 calls of which all are individually below the inner timeout
    // and the sum is below outer timeout which affects the final call which waited the whole time
    let mut join_handles = Vec::new();
    for _ in 0..3 {
        let client = client.clone();
        let url = url.clone();
        let join_handle = std::thread::spawn(move || client.get(url.clone()).send());
        join_handles.push(join_handle);
    }

    for handle in join_handles {
        let res = handle.join().unwrap();
        assert!(
            res.is_ok(),
            "neither outer long timeout or inner short timeout should be exceeded"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn no_generic_bounds_required_for_client_new() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::Client::new();
    let res = client.get(url).send().await;

    assert!(res.is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn no_generic_bounds_required_for_client_new_blocking() {
    let _ = env_logger::try_init();

    let server = server::http(move |_req| async { http::Response::default() });

    let url = format!("http://{}", server.addr());

    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send();

    assert!(res.is_ok());
}
