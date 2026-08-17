#![cfg(not(target_arch = "wasm32"))]

#[cfg(all(feature = "__tls", not(feature = "rustls-tls-manual-roots")))]
#[tokio::test]
async fn test_badssl_modern() {
    let text = reqwest::Client::builder()
        .no_proxy()
        .build()
        .unwrap()
        .get("https://mozilla-modern.badssl.com/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(text.contains("<title>mozilla-modern.badssl.com</title>"));
}

#[cfg(any(
    feature = "rustls-tls-webpki-roots",
    feature = "rustls-tls-native-roots"
))]
#[tokio::test]
async fn test_rustls_badssl_modern() {
    let text = reqwest::Client::builder()
        .use_rustls_tls()
        .no_proxy()
        .build()
        .unwrap()
        .get("https://mozilla-modern.badssl.com/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(text.contains("<title>mozilla-modern.badssl.com</title>"));
}

#[cfg(feature = "__tls")]
#[tokio::test]
async fn test_badssl_self_signed() {
    let text = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .no_proxy()
        .build()
        .unwrap()
        .get("https://self-signed.badssl.com/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(text.contains("<title>self-signed.badssl.com</title>"));
}

#[cfg(feature = "__tls")]
#[tokio::test]
async fn test_badssl_no_built_in_roots() {
    let result = reqwest::Client::builder()
        .tls_built_in_root_certs(false)
        .no_proxy()
        .build()
        .unwrap()
        .get("https://mozilla-modern.badssl.com/")
        .send()
        .await;

    assert!(result.is_err());
}

#[cfg(feature = "native-tls")]
#[tokio::test]
async fn test_badssl_wrong_host() {
    let text = reqwest::Client::builder()
        .danger_accept_invalid_hostnames(true)
        .no_proxy()
        .build()
        .unwrap()
        .get("https://wrong.host.badssl.com/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(text.contains("<title>wrong.host.badssl.com</title>"));

    let result = reqwest::Client::builder()
        .danger_accept_invalid_hostnames(true)
        .build()
        .unwrap()
        .get("https://self-signed.badssl.com/")
        .send()
        .await;

    assert!(result.is_err());
}
