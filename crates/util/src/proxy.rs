#[cfg(target_os = "macos")]
use system_configuration::{
    core_foundation::{
        base::CFType,
        dictionary::CFDictionary,
        number::CFNumber,
        string::{CFString, CFStringRef},
    },
    dynamic_store::SCDynamicStoreBuilder,
    sys::schema_definitions::{
        kSCPropNetProxiesHTTPEnable, kSCPropNetProxiesHTTPPort, kSCPropNetProxiesHTTPProxy,
        kSCPropNetProxiesHTTPSEnable, kSCPropNetProxiesHTTPSPort, kSCPropNetProxiesHTTPSProxy,
        kSCPropNetProxiesSOCKSEnable, kSCPropNetProxiesSOCKSPort, kSCPropNetProxiesSOCKSProxy,
    },
};

pub fn http_proxy_from_env() -> Option<isahc::http::Uri> {
    let all_keys = [
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ];
    for env_key in all_keys {
        if let Ok(env) = std::env::var(env_key) {
            return env.parse::<isahc::http::Uri>().ok();
        }
    }
    proxy_from_platform()
}

#[cfg(target_os = "macos")]
fn proxy_from_platform() -> Option<isahc::http::Uri> {
    let store_map = SCDynamicStoreBuilder::new("Zed").build();
    let Some(proxies_map) = store_map.get_proxies() else {
        return None;
    };
    unsafe {
        let entries = [
            (
                "http",
                kSCPropNetProxiesHTTPEnable,
                kSCPropNetProxiesHTTPProxy,
                kSCPropNetProxiesHTTPPort,
            ),
            (
                "https",
                kSCPropNetProxiesHTTPSEnable,
                kSCPropNetProxiesHTTPSProxy,
                kSCPropNetProxiesHTTPSPort,
            ),
            (
                "socks5",
                kSCPropNetProxiesSOCKSEnable,
                kSCPropNetProxiesSOCKSProxy,
                kSCPropNetProxiesSOCKSPort,
            ),
        ];
        for entry in entries {
            if let Some(url) =
                load_url_from_dynamic_store(&proxies_map, entry.1, entry.2, entry.3, entry.0)
            {
                return url.parse().ok();
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn load_url_from_dynamic_store(
    proxies_map: &CFDictionary<CFString, CFType>,
    type_key: CFStringRef,
    host_key: CFStringRef,
    port_key: CFStringRef,
    scheme: &str,
) -> Option<String> {
    let support = proxies_map
        .find(type_key)
        .and_then(|val| val.downcast::<CFNumber>())
        .and_then(|val| val.to_i32())
        .unwrap_or_default()
        == 1;
    if support {
        let host_val = proxies_map
            .find(host_key)
            .and_then(|val| val.downcast::<CFString>())
            .map(|val| val.to_string());
        let port_val = proxies_map
            .find(port_key)
            .and_then(|val| val.downcast::<CFNumber>())
            .and_then(|val| val.to_i32());

        return match (host_val, port_val) {
            (Some(host), Some(port)) => Some(format!("{}://{}:{}", scheme, host, port)),
            (Some(host), None) => Some(format!("{}://{}", scheme, host)),
            _ => None,
        };
    }
    None
}

#[cfg(not(target_os = "macos"))]
fn proxy_from_platform() -> Option<isahc::http::Uri> {
    None
}
