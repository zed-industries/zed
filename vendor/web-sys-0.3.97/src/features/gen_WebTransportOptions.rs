#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WebTransportOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowPooling` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "allowPooling")]
    pub fn get_allow_pooling(this: &WebTransportOptions) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowPooling` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "allowPooling")]
    pub fn set_allow_pooling(this: &WebTransportOptions, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportCongestionControl")]
    #[doc = "Get the `congestionControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCongestionControl`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "congestionControl")]
    pub fn get_congestion_control(
        this: &WebTransportOptions,
    ) -> Option<WebTransportCongestionControl>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportCongestionControl")]
    #[doc = "Change the `congestionControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCongestionControl`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "congestionControl")]
    pub fn set_congestion_control(this: &WebTransportOptions, val: WebTransportCongestionControl);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `requireUnreliable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "requireUnreliable")]
    pub fn get_require_unreliable(this: &WebTransportOptions) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `requireUnreliable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "requireUnreliable")]
    pub fn set_require_unreliable(this: &WebTransportOptions, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportHash")]
    #[doc = "Get the `serverCertificateHashes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportHash`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "serverCertificateHashes")]
    pub fn get_server_certificate_hashes(
        this: &WebTransportOptions,
    ) -> Option<::js_sys::Array<WebTransportHash>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportHash")]
    #[doc = "Change the `serverCertificateHashes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportHash`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "serverCertificateHashes")]
    pub fn set_server_certificate_hashes(this: &WebTransportOptions, val: &[WebTransportHash]);
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportOptions {
    #[doc = "Construct a new `WebTransportOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_allow_pooling()` instead."]
    pub fn allow_pooling(&mut self, val: bool) -> &mut Self {
        self.set_allow_pooling(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportCongestionControl")]
    #[deprecated = "Use `set_congestion_control()` instead."]
    pub fn congestion_control(&mut self, val: WebTransportCongestionControl) -> &mut Self {
        self.set_congestion_control(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_require_unreliable()` instead."]
    pub fn require_unreliable(&mut self, val: bool) -> &mut Self {
        self.set_require_unreliable(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportHash")]
    #[deprecated = "Use `set_server_certificate_hashes()` instead."]
    pub fn server_certificate_hashes(&mut self, val: &[WebTransportHash]) -> &mut Self {
        self.set_server_certificate_hashes(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportOptions {
    fn default() -> Self {
        Self::new()
    }
}
