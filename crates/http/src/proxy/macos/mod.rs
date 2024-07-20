#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod dynamic_store;
pub(crate) mod system_configuration_sys_extra {
    use core_foundation::{
        array::CFArrayRef,
        base::{CFAllocatorRef, CFIndex, CFTypeID},
        dictionary::CFDictionaryRef,
        runloop::CFRunLoopSourceRef,
        string::CFStringRef,
    };
    include!(concat!(env!("OUT_DIR"), "/system_configuration_sys.rs"));
}

use super::{SysProxiesSettings, SystemProxiesStore};
use core_foundation::{
    array::CFArray,
    base::{kCFAllocatorDefault, CFType, TCFType},
    dictionary::CFDictionary,
    number::CFNumber,
    runloop::{kCFRunLoopDefaultMode, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun},
    string::{CFString, CFStringRef},
};
use dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder, SCDynamicStoreCallBackContext};
use std::ptr::null;
use system_configuration_sys_extra::*;

pub struct MacSysProxiesStore;

impl SystemProxiesStore for MacSysProxiesStore {
    fn init<F>(update_callback: F) -> SysProxiesSettings
    where
        F: FnMut(&SysProxiesSettings) + Send + Sync + 'static,
    {
        struct ProxyCallbackInfo<F> {
            update_callback: F,
        }

        let store = SCDynamicStoreBuilder::new("MonitorProxySettings")
            .callback_context(SCDynamicStoreCallBackContext {
                callout: |store, _, info: &mut ProxyCallbackInfo<F>| {
                    (&mut info.update_callback)(&get_proxy_settings(&store));
                },
                info: ProxyCallbackInfo { update_callback },
            })
            .build();
        set_notification_keys(&store, &notification_keys());
        let init_proxy_settings = get_proxy_settings(&store);
        let store = SCDynamicStoreSendWrap(store);

        std::thread::spawn(move || unsafe {
            let store = store;
            let run_loop_source = store.0.create_run_loop_source();
            CFRunLoopAddSource(
                CFRunLoopGetCurrent(),
                run_loop_source.as_concrete_TypeRef(),
                kCFRunLoopDefaultMode,
            );
            CFRunLoopRun();
        });

        init_proxy_settings
    }
}

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
        http: get_type(&proxy_map, ProxyType::Http),
        https: get_type(&proxy_map, ProxyType::Https),
        socks: get_type(&proxy_map, ProxyType::Socks5),
    }
}

fn get_type(proxy_map: &CFDictionary<CFString, CFType>, proxy_type: ProxyType) -> Option<String> {
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

fn notification_keys() -> CFArray<CFStringRef> {
    unsafe { CFArray::from_copyable(&[SCDynamicStoreKeyCreateProxies(kCFAllocatorDefault)]) }
}

fn set_notification_keys<T>(store: &SCDynamicStore, keys: &CFArray<T>) -> bool {
    unsafe {
        SCDynamicStoreSetNotificationKeys(
            store.as_concrete_TypeRef(),
            keys.as_concrete_TypeRef(),
            null(),
        ) != 0
    }
}

struct SCDynamicStoreSendWrap(SCDynamicStore);
unsafe impl Send for SCDynamicStoreSendWrap {}
