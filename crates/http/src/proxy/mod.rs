#[cfg(target_os = "macos")]
pub mod macos;

use isahc::http::Uri;
use parking_lot::Mutex;
use std::sync::Arc;

/// Dynamically updated proxy settings.
#[derive(Clone)]
pub struct Proxy {
    /// currently active proxy uri.
    current_proxy: Arc<Mutex<Option<Uri>>>,
    /// zed proxy settings.
    zed_proxy_settings: Arc<Mutex<Option<String>>>,
    /// system proxy settings.
    sys_proxy_settings: Arc<Mutex<SysProxiesSettings>>,
}

/// System proxies settings.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct SysProxiesSettings {
    pub http: Option<String>,
    pub https: Option<String>,
    pub socks: Option<String>,
}

impl Proxy {
    /// Dynamic proxy object following system proxy settings
    /// (currently available on macOS).
    pub fn init(zed_proxy_settings: &Option<String>) -> Self {
        let proxy = Proxy {
            current_proxy: Arc::new(Mutex::new(None)),
            zed_proxy_settings: Arc::new(Mutex::new(zed_proxy_settings.clone())),
            sys_proxy_settings: Arc::new(Mutex::new(SysProxiesSettings::default())),
        };

        // Subscribe to system notification.
        #[cfg(target_os = "macos")]
        macos::init_proxy(&proxy);

        proxy
    }

    /// Static proxy object with env proxy.
    pub fn env_proxy() -> Self {
        let proxy = Proxy::no_proxy();
        *proxy.current_proxy.lock() = read_proxy_from_env();
        proxy
    }

    /// Static proxy object with no proxy.
    pub fn no_proxy() -> Self {
        Self {
            current_proxy: Default::default(),
            zed_proxy_settings: Default::default(),
            sys_proxy_settings: Default::default(),
        }
    }

    /// read: as string
    pub fn to_string(&self) -> Option<String> {
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

    /// read: as Uri
    pub fn to_uri(&self) -> Option<Uri> {
        self.current_proxy.lock().clone()
    }

    /// update: zed settings changed
    pub fn update_zed_settings(&self, zed_proxy_settings: &Option<String>) {
        {
            let mut zed_proxy_settings_lock = self.zed_proxy_settings.lock();
            if &*zed_proxy_settings_lock == zed_proxy_settings {
                // no update, early return
                return;
            } else {
                zed_proxy_settings_lock.clone_from(zed_proxy_settings);
            }
        }
        let sys_proxy_settings = self.sys_proxy_settings.lock().choose_one();
        let current_proxy = get_current_proxy(zed_proxy_settings, &sys_proxy_settings);
        log::debug!(
            "updated proxy settings to {:?} (on zed settings update)",
            current_proxy
        );
        *self.current_proxy.lock() = current_proxy;
    }

    /// update: system proxy settings changed
    pub fn update_sys_settings(&self, sys_proxy_settings: &SysProxiesSettings) {
        {
            let mut sys_proxy_settings_lock = self.sys_proxy_settings.lock();
            if &*sys_proxy_settings_lock == sys_proxy_settings {
                // no update, early return
                return;
            } else {
                sys_proxy_settings_lock.clone_from(sys_proxy_settings);
            }
        }
        let zed_proxy_settings = self.zed_proxy_settings.lock().clone();
        let current_proxy =
            get_current_proxy(&zed_proxy_settings, &sys_proxy_settings.choose_one());
        log::debug!(
            "updated proxy settings to {:?} (on sys settings update)",
            current_proxy
        );
        *self.current_proxy.lock() = current_proxy;
    }
}

impl SysProxiesSettings {
    /// Choose one available proxy if any.
    /// Prioritize socks > https > http.
    pub fn choose_one(&self) -> Option<String> {
        //
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

/// Compute current proxy settings based on zed settings, system settings,
/// and environment variable.
///
/// ## Compute Logic
/// If zed proxy settings is `"system"`, then try to use system proxy settings.
/// Otherwise, try to use the url of zed proxy settings.
/// If both are not available, try environment variables.
fn get_current_proxy(
    zed_proxy_settings: &Option<String>,
    sys_proxy_settings: &Option<String>,
) -> Option<Uri> {
    const USE_SYSTEM_PROXY: &str = "system";

    zed_proxy_settings
        .as_ref()
        .and_then(|proxy| {
            if proxy != USE_SYSTEM_PROXY {
                proxy
                    .parse::<Uri>()
                    .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                    .ok()
            } else {
                sys_proxy_settings.as_ref().and_then(|input| {
                    input
                        .parse::<Uri>()
                        .inspect_err(|e| log::error!("Error parsing system proxy settings: {}", e))
                        .ok()
                })
            }
        })
        .or_else(|| read_proxy_from_env())
}

pub fn read_proxy_from_env() -> Option<Uri> {
    const ENV_VARS: &[&str] = &[
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ];

    for var in ENV_VARS {
        if let Ok(env) = std::env::var(var) {
            return env.parse::<Uri>().ok();
        }
    }
    None
}
