use super::{SysProxiesSettings, SystemProxiesStore};
use system_configuration::{
    core_foundation::{
        array::CFArray,
        base::CFType,
        dictionary::CFDictionary,
        number::CFNumber,
        string::{CFString, CFStringRef},
    },
    dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder, SCDynamicStoreCallBackContext},
    sys::schema_definitions::{
        kSCPropNetProxiesHTTPEnable, kSCPropNetProxiesHTTPPort, kSCPropNetProxiesHTTPProxy,
        kSCPropNetProxiesHTTPSEnable, kSCPropNetProxiesHTTPSPort, kSCPropNetProxiesHTTPSProxy,
        kSCPropNetProxiesSOCKSEnable, kSCPropNetProxiesSOCKSPort, kSCPropNetProxiesSOCKSProxy,
    },
};

#[derive(Clone)]
pub struct MacSysProxiesStore(SCDynamicStore);
unsafe impl Send for MacSysProxiesStore {}

impl SystemProxiesStore for MacSysProxiesStore {
    fn new<F: FnMut(&SysProxiesSettings)>(update_callback: F) -> Self {
        struct ProxyCallbackInfo<F> {
            proxy_settings: SysProxiesSettings,
            update_callback: F,
        }

        let dynamic_store = SCDynamicStoreBuilder::new("Zed")
            .callback_context(SCDynamicStoreCallBackContext {
                callout: |store, _, info: &mut ProxyCallbackInfo<F>| {
                    info.proxy_settings = MacSysProxiesStore(store).proxy_settings();
                    (&mut info.update_callback)(&info.proxy_settings)
                },
                info: ProxyCallbackInfo {
                    proxy_settings: SysProxiesSettings::default(),
                    update_callback,
                },
            })
            .build();
        dynamic_store.set_notification_keys(&notification_keys(), &notification_patterns());
        Self(dynamic_store)
    }

    fn proxy_settings(&self) -> SysProxiesSettings {
        let Some(proxy_map) = self.0.get_proxies() else {
            return SysProxiesSettings::default();
        };
        SysProxiesSettings {
            http: get_type(&proxy_map, ProxyType::Http),
            https: get_type(&proxy_map, ProxyType::Https),
            socks: get_type(&proxy_map, ProxyType::Socks5),
        }
    }
}

#[derive(Clone, Copy)]
enum ProxyType {
    Http,
    Https,
    Socks5,
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
    unsafe {
        CFArray::from_copyable(&[
            kSCPropNetProxiesHTTPEnable,
            kSCPropNetProxiesHTTPPort,
            kSCPropNetProxiesHTTPProxy,
            kSCPropNetProxiesHTTPSEnable,
            kSCPropNetProxiesHTTPSPort,
            kSCPropNetProxiesHTTPSProxy,
            kSCPropNetProxiesSOCKSEnable,
            kSCPropNetProxiesSOCKSPort,
            kSCPropNetProxiesSOCKSProxy,
        ])
    }
}

fn notification_patterns() -> CFArray<CFStringRef> {
    CFArray::from_copyable(&[])
}
