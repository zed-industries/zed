#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Navigator",
        typescript_type = "Navigator"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Navigator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub type Navigator;
    #[cfg(feature = "Clipboard")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "clipboard")]
    #[doc = "Getter for the `clipboard` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clipboard)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`, `Navigator`*"]
    pub fn clipboard(this: &Navigator) -> Clipboard;
    #[cfg(feature = "Permissions")]
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "permissions")]
    #[doc = "Getter for the `permissions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Permissions`*"]
    pub fn permissions(this: &Navigator) -> Result<Permissions, JsValue>;
    #[cfg(feature = "MimeTypeArray")]
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "mimeTypes")]
    #[doc = "Getter for the `mimeTypes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeTypeArray`, `Navigator`*"]
    pub fn mime_types(this: &Navigator) -> Result<MimeTypeArray, JsValue>;
    #[cfg(feature = "PluginArray")]
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "plugins")]
    #[doc = "Getter for the `plugins` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `PluginArray`*"]
    pub fn plugins(this: &Navigator) -> Result<PluginArray, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "doNotTrack")]
    #[doc = "Getter for the `doNotTrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/doNotTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn do_not_track(this: &Navigator) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "maxTouchPoints")]
    #[doc = "Getter for the `maxTouchPoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn max_touch_points(this: &Navigator) -> i32;
    #[cfg(feature = "MediaCapabilities")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "mediaCapabilities")]
    #[doc = "Getter for the `mediaCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `Navigator`*"]
    pub fn media_capabilities(this: &Navigator) -> MediaCapabilities;
    #[cfg(feature = "NetworkInformation")]
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "connection")]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `NetworkInformation`*"]
    pub fn connection(this: &Navigator) -> Result<NetworkInformation, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "activeVRDisplays")]
    #[doc = "Getter for the `activeVRDisplays` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/activeVRDisplays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn active_vr_displays(this: &Navigator) -> ::js_sys::Array;
    #[cfg(feature = "MediaDevices")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "Navigator",
        js_name = "mediaDevices"
    )]
    #[doc = "Getter for the `mediaDevices` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`, `Navigator`*"]
    pub fn media_devices(this: &Navigator) -> Result<MediaDevices, JsValue>;
    #[cfg(feature = "ServiceWorkerContainer")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "serviceWorker")]
    #[doc = "Getter for the `serviceWorker` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ServiceWorkerContainer`*"]
    pub fn service_worker(this: &Navigator) -> ServiceWorkerContainer;
    #[cfg(feature = "Presentation")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "Navigator",
        js_name = "presentation"
    )]
    #[doc = "Getter for the `presentation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Presentation`*"]
    pub fn presentation(this: &Navigator) -> Result<Option<Presentation>, JsValue>;
    #[cfg(feature = "CredentialsContainer")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "credentials")]
    #[doc = "Getter for the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`, `Navigator`*"]
    pub fn credentials(this: &Navigator) -> CredentialsContainer;
    #[cfg(feature = "UserActivation")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "userActivation")]
    #[doc = "Getter for the `userActivation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userActivation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `UserActivation`*"]
    pub fn user_activation(this: &Navigator) -> UserActivation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Bluetooth")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "bluetooth")]
    #[doc = "Getter for the `bluetooth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/bluetooth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bluetooth(this: &Navigator) -> Option<Bluetooth>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSession")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "mediaSession")]
    #[doc = "Getter for the `mediaSession` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn media_session(this: &Navigator) -> MediaSession;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WakeLock")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "wakeLock")]
    #[doc = "Getter for the `wakeLock` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/wakeLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `WakeLock`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn wake_lock(this: &Navigator) -> WakeLock;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Hid")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "hid")]
    #[doc = "Getter for the `hid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Hid`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn hid(this: &Navigator) -> Hid;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Serial")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "serial")]
    #[doc = "Getter for the `serial` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serial)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn serial(this: &Navigator) -> Serial;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Usb")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "usb")]
    #[doc = "Getter for the `usb` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/usb)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb(this: &Navigator) -> Usb;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSystem")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "xr")]
    #[doc = "Getter for the `xr` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/xr)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn xr(this: &Navigator) -> XrSystem;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Scheduling")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "scheduling")]
    #[doc = "Getter for the `scheduling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/scheduling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scheduling(this: &Navigator) -> Scheduling;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Navigator",
        js_name = "hardwareConcurrency"
    )]
    #[doc = "Getter for the `hardwareConcurrency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn hardware_concurrency(this: &Navigator) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "deviceMemory")]
    #[doc = "Getter for the `deviceMemory` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deviceMemory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_memory(this: &Navigator) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gpu")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "gpu")]
    #[doc = "Getter for the `gpu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn gpu(this: &Navigator) -> Gpu;
    #[cfg(feature = "Geolocation")]
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "geolocation")]
    #[doc = "Getter for the `geolocation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `Navigator`*"]
    pub fn geolocation(this: &Navigator) -> Result<Geolocation, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "appCodeName")]
    #[doc = "Getter for the `appCodeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appCodeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn app_code_name(this: &Navigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "appName")]
    #[doc = "Getter for the `appName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn app_name(this: &Navigator) -> ::alloc::string::String;
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "appVersion")]
    #[doc = "Getter for the `appVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn app_version(this: &Navigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "platform")]
    #[doc = "Getter for the `platform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/platform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn platform(this: &Navigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Navigator", js_name = "userAgent")]
    #[doc = "Getter for the `userAgent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn user_agent(this: &Navigator) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "product")]
    #[doc = "Getter for the `product` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/product)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn product(this: &Navigator) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "language")]
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn language(this: &Navigator) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "languages")]
    #[doc = "Getter for the `languages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn languages(this: &Navigator) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockManager")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "locks")]
    #[doc = "Getter for the `locks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/locks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn locks(this: &Navigator) -> LockManager;
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "onLine")]
    #[doc = "Getter for the `onLine` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn on_line(this: &Navigator) -> bool;
    #[cfg(feature = "StorageManager")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "storage")]
    #[doc = "Getter for the `storage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `StorageManager`*"]
    pub fn storage(this: &Navigator) -> StorageManager;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaData")]
    #[wasm_bindgen(method, getter, js_class = "Navigator", js_name = "userAgentData")]
    #[doc = "Getter for the `userAgentData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgentData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn user_agent_data(this: &Navigator) -> NavigatorUaData;
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "canShare")]
    #[doc = "The `canShare()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn can_share(this: &Navigator) -> bool;
    #[cfg(feature = "ShareData")]
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "canShare")]
    #[doc = "The `canShare()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ShareData`*"]
    pub fn can_share_with_data(this: &Navigator, data: &ShareData) -> bool;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "getGamepads")]
    #[doc = "The `getGamepads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn get_gamepads(this: &Navigator) -> Result<::js_sys::Array, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "getVRDisplays")]
    #[doc = "The `getVRDisplays()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getVRDisplays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn get_vr_displays(this: &Navigator) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "requestMIDIAccess")]
    #[doc = "The `requestMIDIAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn request_midi_access(this: &Navigator) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MidiOptions")]
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "requestMIDIAccess")]
    #[doc = "The `requestMIDIAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`, `Navigator`*"]
    pub fn request_midi_access_with_options(
        this: &Navigator,
        options: &MidiOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "Navigator",
        js_name = "requestMediaKeySystemAccess"
    )]
    #[doc = "The `requestMediaKeySystemAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn request_media_key_system_access(
        this: &Navigator,
        key_system: &str,
        supported_configurations: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "VrServiceTest")]
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "requestVRServiceTest")]
    #[doc = "The `requestVRServiceTest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestVRServiceTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `VrServiceTest`*"]
    pub fn request_vr_service_test(this: &Navigator) -> VrServiceTest;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon(this: &Navigator, url: &str) -> Result<bool, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Navigator`*"]
    pub fn send_beacon_with_opt_blob(
        this: &Navigator,
        url: &str,
        data: Option<&Blob>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_buffer_source(
        this: &Navigator,
        url: &str,
        data: Option<&::js_sys::Object>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_u8_array(
        this: &Navigator,
        url: &str,
        data: Option<&mut [u8]>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_js_u8_array(
        this: &Navigator,
        url: &str,
        data: Option<&::js_sys::Uint8Array>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "FormData")]
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Navigator`*"]
    pub fn send_beacon_with_opt_form_data(
        this: &Navigator,
        url: &str,
        data: Option<&FormData>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "UrlSearchParams")]
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `UrlSearchParams`*"]
    pub fn send_beacon_with_opt_url_search_params(
        this: &Navigator,
        url: &str,
        data: Option<&UrlSearchParams>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_str(
        this: &Navigator,
        url: &str,
        data: Option<&str>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, method, js_class = "Navigator", js_name = "sendBeacon")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ReadableStream`*"]
    pub fn send_beacon_with_opt_readable_stream(
        this: &Navigator,
        url: &str,
        data: Option<&ReadableStream>,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_class = "Navigator")]
    #[doc = "The `share()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn share(this: &Navigator) -> ::js_sys::Promise;
    #[cfg(feature = "ShareData")]
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "share")]
    #[doc = "The `share()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ShareData`*"]
    pub fn share_with_data(this: &Navigator, data: &ShareData) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "vibrate")]
    #[doc = "The `vibrate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn vibrate_with_duration(this: &Navigator, pattern: u32) -> bool;
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "vibrate")]
    #[doc = "The `vibrate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn vibrate_with_pattern(this: &Navigator, pattern: &::wasm_bindgen::JsValue) -> bool;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Navigator",
        js_name = "registerContentHandler"
    )]
    #[doc = "The `registerContentHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerContentHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn register_content_handler(
        this: &Navigator,
        mime_type: &str,
        url: &str,
        title: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Navigator",
        js_name = "registerProtocolHandler"
    )]
    #[doc = "The `registerProtocolHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn register_protocol_handler(
        this: &Navigator,
        scheme: &str,
        url: &str,
        title: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Navigator", js_name = "taintEnabled")]
    #[doc = "The `taintEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn taint_enabled(this: &Navigator) -> bool;
}
