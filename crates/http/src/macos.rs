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

pub struct SystemProxiesStore(SCDynamicStore);

#[derive(Default)]
pub struct SystemProxySettings {
    pub http: Option<String>,
    pub https: Option<String>,
    pub socks: Option<String>,
}

impl SystemProxySettings {
    fn from_dynamic_store(store: &SCDynamicStore) -> Self {
        let Some(proxy_map) = store.get_proxies() else {
            return Self::default();
        };
        SystemProxySettings {
            http: SystemProxiesStore::get_type(&proxy_map, ProxyType::Http),
            https: SystemProxiesStore::get_type(&proxy_map, ProxyType::Https),
            socks: SystemProxiesStore::get_type(&proxy_map, ProxyType::Socks5),
        }
    }

    /// prioritize socks over https over http
    pub fn select(&self) -> Option<String> {
        if let Some(socks) = &self.socks {
            Some(socks.as_str())
        } else if let Some(https) = &self.https {
            Some(https.as_str())
        } else if let Some(http) = &self.http {
            Some(http.as_str())
        } else {
            None
        }
        .map(str::to_string)
    }
}

struct ProxyCallbackInfo<F> {
    proxy_settings: SystemProxySettings,
    update_callback: F,
}

#[derive(Clone, Copy)]
enum ProxyType {
    Http,
    Https,
    Socks5,
}

impl SystemProxiesStore {
    pub fn new<F: FnMut(&SystemProxySettings)>(update_callback: F) -> Self {
        // TODO: update proxy settings with this callback
        let dynamic_store = SCDynamicStoreBuilder::new("Zed")
            .callback_context(SCDynamicStoreCallBackContext {
                callout: |store, _, info: &mut ProxyCallbackInfo<F>| {
                    info.proxy_settings = SystemProxySettings::from_dynamic_store(&store);
                    (&mut info.update_callback)(&info.proxy_settings)
                },
                info: ProxyCallbackInfo {
                    proxy_settings: SystemProxySettings::default(),
                    update_callback,
                },
            })
            .build();
        dynamic_store
            .set_notification_keys(&Self::notification_keys(), &Self::notification_patterns());
        Self(dynamic_store)
    }

    pub fn proxy_settings(&self) -> SystemProxySettings {
        SystemProxySettings::from_dynamic_store(&self.0)
    }

    fn get_type(
        proxy_map: &CFDictionary<CFString, CFType>,
        proxy_type: ProxyType,
    ) -> Option<String> {
        if !Self::proxy_enabled(&proxy_map, proxy_type) {
            return None;
        }
        let scheme = match proxy_type {
            ProxyType::Http => "http",
            ProxyType::Https => "https",
            ProxyType::Socks5 => "socks5",
        };
        let host = Self::proxy_host(&proxy_map, proxy_type);
        let port = Self::proxy_port(&proxy_map, proxy_type);
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

    fn proxy_host(
        proxy_map: &CFDictionary<CFString, CFType>,
        proxy_type: ProxyType,
    ) -> Option<String> {
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

    fn proxy_port(
        proxy_map: &CFDictionary<CFString, CFType>,
        proxy_type: ProxyType,
    ) -> Option<i32> {
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
}
