#[cfg(target_os = "macos")]
pub mod macos;

use isahc::http::Uri;
use parking_lot::Mutex;
use std::{ops::Deref, sync::Arc};

pub fn create_proxy_info(zed_proxy_settings: Option<String>) -> Arc<dyn ProxyInfo> {
    #[cfg(target_os = "macos")]
    return Arc::new(ProxyInfoImpl::<macos::MacSysProxiesStore>::new(
        zed_proxy_settings,
    ));

    #[cfg(not(target_os = "macos"))]
    return Arc::new(ProxyInfoImpl::<DefaultSystemProxiesStore>::new(
        zed_proxy_settings,
    ));
}

/// Trait representing current proxy settings.
pub trait ProxyInfo: Send + Sync {
    fn proxy_string(&self) -> Option<String>;
    fn proxy_uri(&self) -> Option<Uri>;
    /// receive update from zed settings.
    fn update_zed_settings(&self, zed_proxy_settings: Option<String>);
}

pub struct DefaultProxyInfo;

impl ProxyInfo for DefaultProxyInfo {
    fn proxy_string(&self) -> Option<String> {
        None
    }
    fn proxy_uri(&self) -> Option<Uri> {
        None
    }
    fn update_zed_settings(&self, _: Option<String>) {}
}

/// A dynamic proxy info that's shared across the app.
pub struct ProxyInfoImpl<S> {
    current_proxy: Arc<Mutex<Option<Uri>>>,
    zed_proxy_settings: Arc<Mutex<Option<String>>>,
    system_proxy_store: Mutex<S>,
}

impl<S: SystemProxiesStore> ProxyInfo for ProxyInfoImpl<S> {
    fn proxy_string(&self) -> Option<String> {
        self.current_proxy.lock().as_ref().map(|proxy| {
            // Map proxy settings from `http://localhost:10809` to `http://127.0.0.1:10809`
            // NodeRuntime without environment information can not parse `localhost`
            // correctly.
            // TODO: map to `[::1]` if we are using ipv6
            proxy
                .to_string()
                .to_ascii_lowercase()
                .replace("localhost", "127.0.0.1")
        })
    }

    fn proxy_uri(&self) -> Option<Uri> {
        self.current_proxy.lock().clone()
    }

    fn update_zed_settings(&self, zed_proxy_settings: Option<String>) {
        let mut zed_proxy_settings_lock = self.zed_proxy_settings.lock();
        if zed_proxy_settings_lock.deref() == &zed_proxy_settings {
            // no update to proxy settings, early return
            return;
        } else {
            zed_proxy_settings_lock.clone_from(&zed_proxy_settings);
        }
        drop(zed_proxy_settings_lock);

        let system_proxy = self.system_proxy_store.lock().proxy_settings().select();
        let new_settings = Self::get_proxy(zed_proxy_settings, system_proxy);
        log::debug!(
            "updated proxy settings to {:?} (on zed settings update)",
            new_settings
        );
        *self.current_proxy.lock() = new_settings;
    }
}

impl<S: SystemProxiesStore> ProxyInfoImpl<S> {
    pub fn new(zed_proxy_string: Option<String>) -> Self {
        let current_proxy = Arc::new(Mutex::new(None));
        let zed_proxy_settings = Arc::new(Mutex::new(zed_proxy_string.clone()));

        // callback when system proxy get updated.
        let update_callback = {
            let current_proxy = current_proxy.clone();
            let zed_proxy_settings = zed_proxy_settings.clone();
            move |new_sys_proxy: &SysProxiesSettings| {
                let zed_proxy_settings = zed_proxy_settings.lock().clone();
                let new_settings = Self::get_proxy(zed_proxy_settings, new_sys_proxy.select());
                log::debug!(
                    "updated proxy settings to {:?} (on sys settings update)",
                    new_settings
                );
                *current_proxy.lock() = new_settings;
            }
        };
        let system_proxy_store = Mutex::new(S::new(update_callback));

        let info = Self {
            current_proxy,
            zed_proxy_settings,
            system_proxy_store,
        };
        info.update_zed_settings(zed_proxy_string);
        info
    }

    /// The priority being:
    /// If proxy settings is 'system', then use system proxy settings.
    /// If proxy settings is 'None', then use environment variables.
    fn get_proxy(zed_proxy_settings: Option<String>, system_proxy: Option<String>) -> Option<Uri> {
        macro_rules! try_env {
            ($($env:literal),+) => {
                $(
                    if let Ok(env) = std::env::var($env) {
                        return env.parse::<isahc::http::Uri>().ok();
                    }
                )+
            };
        }

        const USE_SYSTEM_PROXY: &str = "system";

        zed_proxy_settings
            .and_then(|proxy| {
                if proxy != USE_SYSTEM_PROXY {
                    log::debug!("trying zed proxy settings");
                    proxy
                        .parse::<isahc::http::Uri>()
                        .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                        .ok()
                } else {
                    log::debug!("trying system proxy settings");
                    system_proxy.and_then(|input| {
                        input
                            .parse::<isahc::http::Uri>()
                            .inspect_err(|e| {
                                log::error!("Error parsing system proxy settings: {}", e)
                            })
                            .ok()
                    })
                }
            })
            .or_else(|| {
                log::debug!("trying env proxy settings");
                try_env!(
                    "ALL_PROXY",
                    "all_proxy",
                    "HTTPS_PROXY",
                    "https_proxy",
                    "HTTP_PROXY",
                    "http_proxy"
                );
                None
            })
    }
}

/// System proxy settings.
#[derive(Default)]
pub struct SysProxiesSettings {
    pub http: Option<String>,
    pub https: Option<String>,
    pub socks: Option<String>,
}

/// Trait representing a system proxy store.
pub trait SystemProxiesStore: Send {
    /// Pass a callback function to receive system settings update.
    fn new<F>(update_callback: F) -> Self
    where
        F: FnMut(&SysProxiesSettings) + Send + Sync + 'static;
    /// Query latest proxy settings.
    fn proxy_settings(&self) -> SysProxiesSettings {
        return SysProxiesSettings::default();
    }
}

impl SysProxiesSettings {
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

#[derive(Clone)]
pub struct DefaultSystemProxiesStore;

impl SystemProxiesStore for DefaultSystemProxiesStore {
    fn new<F: FnMut(&SysProxiesSettings)>(_: F) -> Self {
        Self
    }
}
