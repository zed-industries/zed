#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XRSystem",
        typescript_type = "XRSystem"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrSystem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrSystem;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRSystem", js_name = "ondevicechange")]
    #[doc = "Getter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ondevicechange(this: &XrSystem) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "XRSystem", js_name = "ondevicechange")]
    #[doc = "Setter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ondevicechange(this: &XrSystem, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionMode")]
    #[wasm_bindgen(method, js_class = "XRSystem", js_name = "isSessionSupported")]
    #[doc = "The `isSessionSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/isSessionSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionMode`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_session_supported(
        this: &XrSystem,
        mode: XrSessionMode,
    ) -> ::js_sys::Promise<::js_sys::Boolean>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrSession", feature = "XrSessionMode",))]
    #[wasm_bindgen(method, js_class = "XRSystem", js_name = "requestSession")]
    #[doc = "The `requestSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSession`, `XrSessionMode`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_session(this: &XrSystem, mode: XrSessionMode) -> ::js_sys::Promise<XrSession>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "XrSession",
        feature = "XrSessionInit",
        feature = "XrSessionMode",
    ))]
    #[wasm_bindgen(method, js_class = "XRSystem", js_name = "requestSession")]
    #[doc = "The `requestSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSession`, `XrSessionInit`, `XrSessionMode`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_session_with_options(
        this: &XrSystem,
        mode: XrSessionMode,
        options: &XrSessionInit,
    ) -> ::js_sys::Promise<XrSession>;
}
