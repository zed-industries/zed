#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "USBDeviceRequestOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbDeviceRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbDeviceRequestOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDeviceFilter")]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`, `UsbDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(this: &UsbDeviceRequestOptions) -> ::js_sys::Array<UsbDeviceFilter>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDeviceFilter")]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`, `UsbDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &UsbDeviceRequestOptions, val: &[UsbDeviceFilter]);
}
#[cfg(web_sys_unstable_apis)]
impl UsbDeviceRequestOptions {
    #[cfg(feature = "UsbDeviceFilter")]
    #[doc = "Construct a new `UsbDeviceRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`, `UsbDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(filters: &[UsbDeviceFilter]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_filters(filters);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDeviceFilter")]
    #[deprecated = "Use `set_filters()` instead."]
    pub fn filters(&mut self, val: &[UsbDeviceFilter]) -> &mut Self {
        self.set_filters(val);
        self
    }
}
