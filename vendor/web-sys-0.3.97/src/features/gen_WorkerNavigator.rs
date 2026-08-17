#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WorkerNavigator",
        typescript_type = "WorkerNavigator"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerNavigator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub type WorkerNavigator;
    #[cfg(feature = "NetworkInformation")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "connection"
    )]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`, `WorkerNavigator`*"]
    pub fn connection(this: &WorkerNavigator) -> Result<NetworkInformation, JsValue>;
    #[cfg(feature = "MediaCapabilities")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "mediaCapabilities"
    )]
    #[doc = "Getter for the `mediaCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/mediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `WorkerNavigator`*"]
    pub fn media_capabilities(this: &WorkerNavigator) -> MediaCapabilities;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Serial")]
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "serial")]
    #[doc = "Getter for the `serial` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/serial)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn serial(this: &WorkerNavigator) -> Serial;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Usb")]
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "usb")]
    #[doc = "Getter for the `usb` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/usb)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb(this: &WorkerNavigator) -> Usb;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "hardwareConcurrency"
    )]
    #[doc = "Getter for the `hardwareConcurrency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hardwareConcurrency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn hardware_concurrency(this: &WorkerNavigator) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "deviceMemory")]
    #[doc = "Getter for the `deviceMemory` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/deviceMemory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_memory(this: &WorkerNavigator) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gpu")]
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "gpu")]
    #[doc = "Getter for the `gpu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/gpu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn gpu(this: &WorkerNavigator) -> Gpu;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "appCodeName"
    )]
    #[doc = "Getter for the `appCodeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appCodeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn app_code_name(this: &WorkerNavigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "appName")]
    #[doc = "Getter for the `appName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn app_name(this: &WorkerNavigator) -> ::alloc::string::String;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "appVersion"
    )]
    #[doc = "Getter for the `appVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn app_version(this: &WorkerNavigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "platform"
    )]
    #[doc = "Getter for the `platform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/platform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn platform(this: &WorkerNavigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "userAgent"
    )]
    #[doc = "Getter for the `userAgent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn user_agent(this: &WorkerNavigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "product")]
    #[doc = "Getter for the `product` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/product)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn product(this: &WorkerNavigator) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "language")]
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn language(this: &WorkerNavigator) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "languages")]
    #[doc = "Getter for the `languages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/languages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn languages(this: &WorkerNavigator) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockManager")]
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "locks")]
    #[doc = "Getter for the `locks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/locks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn locks(this: &WorkerNavigator) -> LockManager;
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "onLine")]
    #[doc = "Getter for the `onLine` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/onLine)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn on_line(this: &WorkerNavigator) -> bool;
    #[cfg(feature = "StorageManager")]
    #[wasm_bindgen(method, getter, js_class = "WorkerNavigator", js_name = "storage")]
    #[doc = "Getter for the `storage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`, `WorkerNavigator`*"]
    pub fn storage(this: &WorkerNavigator) -> StorageManager;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaData")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WorkerNavigator",
        js_name = "userAgentData"
    )]
    #[doc = "Getter for the `userAgentData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgentData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn user_agent_data(this: &WorkerNavigator) -> NavigatorUaData;
    #[wasm_bindgen(method, js_class = "WorkerNavigator", js_name = "taintEnabled")]
    #[doc = "The `taintEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/taintEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    pub fn taint_enabled(this: &WorkerNavigator) -> bool;
}
