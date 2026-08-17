#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "USBPermissionStorage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbPermissionStorage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbPermissionStorage;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AllowedUsbDevice")]
    #[doc = "Get the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`, `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "allowedDevices")]
    pub fn get_allowed_devices(
        this: &UsbPermissionStorage,
    ) -> Option<::js_sys::Array<AllowedUsbDevice>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AllowedUsbDevice")]
    #[doc = "Change the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`, `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "allowedDevices")]
    pub fn set_allowed_devices(this: &UsbPermissionStorage, val: &[AllowedUsbDevice]);
}
#[cfg(web_sys_unstable_apis)]
impl UsbPermissionStorage {
    #[doc = "Construct a new `UsbPermissionStorage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AllowedUsbDevice")]
    #[deprecated = "Use `set_allowed_devices()` instead."]
    pub fn allowed_devices(&mut self, val: &[AllowedUsbDevice]) -> &mut Self {
        self.set_allowed_devices(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UsbPermissionStorage {
    fn default() -> Self {
        Self::new()
    }
}
