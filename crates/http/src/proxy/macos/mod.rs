mod system_configuration;

use super::{Proxy, SysProxiesSettings};
use core_foundation::{
    array::CFArray,
    base::{kCFAllocatorDefault, CFType, TCFType},
    dictionary::CFDictionary,
    number::CFNumber,
    runloop::{kCFRunLoopDefaultMode, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun},
    string::CFString,
};
use system_configuration::{
    sys::*, SCDynamicStore, SCDynamicStoreBuilder, SCDynamicStoreCallBackContext,
};

/// Start listening to system proxy update and call update_sys_settings
/// for the first time.
pub fn init_proxy(proxy: &Proxy) {
    let info = proxy.clone();
    let dynamic_store = SCDynamicStoreBuilder::new("MonitorProxySettings")
        .callback_context(SCDynamicStoreCallBackContext {
            callout: |store, _, info: &mut Proxy| {
                info.update_sys_settings(&get_proxy_settings(&store))
            },
            info,
        })
        .build();
    proxy.update_sys_settings(&get_proxy_settings(&dynamic_store));
    // safety: do not clone, but only move (Clone + Send = racing!).
    unsafe impl Send for SCDynamicStore {}
    dynamic_store.set_notification_keys(
        &notification_keys(),
        &CFArray::<CFStringRef>::from_copyable(&[]),
    );
    std::thread::spawn(move || unsafe {
        let run_loop_source = dynamic_store.create_run_loop_source();
        CFRunLoopAddSource(
            CFRunLoopGetCurrent(),
            run_loop_source.as_concrete_TypeRef(),
            kCFRunLoopDefaultMode,
        );
        CFRunLoopRun();
    });
}

fn notification_keys() -> CFArray<CFStringRef> {
    CFArray::from_copyable(&[unsafe { SCDynamicStoreKeyCreateProxies(kCFAllocatorDefault) }])
}

/*
 * macOS dynamic store proxy schema related code.
 */

#[derive(Clone, Copy)]
enum ProxyType {
    Http,
    Https,
    Socks5,
}

fn get_proxy_settings(store: &SCDynamicStore) -> SysProxiesSettings {
    let Some(proxy_map) = store.get_proxies() else {
        return SysProxiesSettings::default();
    };
    SysProxiesSettings {
        http: get_proxy_of_type(&proxy_map, ProxyType::Http),
        https: get_proxy_of_type(&proxy_map, ProxyType::Https),
        socks: get_proxy_of_type(&proxy_map, ProxyType::Socks5),
    }
}

fn get_proxy_of_type(
    proxy_map: &CFDictionary<CFString, CFType>,
    proxy_type: ProxyType,
) -> Option<String> {
    if !proxy_enabled(&proxy_map, proxy_type) {
        return None;
    }
    let scheme = match proxy_type {
        ProxyType::Http => "http",
        ProxyType::Https => "https",
        ProxyType::Socks5 => "socks5",
    };
    let host = proxy_host(&proxy_map, proxy_type);
    let port = proxy_port(&proxy_map, proxy_type);
    match (host, port) {
        (Some(host), Some(port)) => Some(format!("{}://{}:{}", scheme, host, port)),
        (Some(host), None) => Some(format!("{}://{}", scheme, host)),
        _ => None,
    }
}

fn proxy_enabled(proxy_map: &CFDictionary<CFString, CFType>, proxy_type: ProxyType) -> bool {
    let enable_key = unsafe {
        match proxy_type {
            ProxyType::Http => kSCPropNetProxiesHTTPEnable,
            ProxyType::Https => kSCPropNetProxiesHTTPSEnable,
            ProxyType::Socks5 => kSCPropNetProxiesSOCKSEnable,
        }
    };
    proxy_map
        .find(enable_key)
        .and_then(|val| val.downcast::<CFNumber>())
        .and_then(|val| val.to_i32())
        .unwrap_or_default()
        == 1
}

fn proxy_host(proxy_map: &CFDictionary<CFString, CFType>, proxy_type: ProxyType) -> Option<String> {
    let host_key = unsafe {
        match proxy_type {
            ProxyType::Http => kSCPropNetProxiesHTTPProxy,
            ProxyType::Https => kSCPropNetProxiesHTTPSProxy,
            ProxyType::Socks5 => kSCPropNetProxiesSOCKSProxy,
        }
    };
    proxy_map
        .find(host_key)
        .and_then(|val| val.downcast::<CFString>())
        .map(|val| val.to_string())
}

fn proxy_port(proxy_map: &CFDictionary<CFString, CFType>, proxy_type: ProxyType) -> Option<i32> {
    let port_key = unsafe {
        match proxy_type {
            ProxyType::Http => kSCPropNetProxiesHTTPPort,
            ProxyType::Https => kSCPropNetProxiesHTTPSPort,
            ProxyType::Socks5 => kSCPropNetProxiesSOCKSPort,
        }
    };
    proxy_map
        .find(port_key)
        .and_then(|val| val.downcast::<CFNumber>())
        .and_then(|val| val.to_i32())
}
