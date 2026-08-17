#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HIDDeviceRequestOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidDeviceRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidDeviceRequestOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDeviceFilter")]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`, `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(this: &HidDeviceRequestOptions) -> ::js_sys::Array<HidDeviceFilter>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDeviceFilter")]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`, `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &HidDeviceRequestOptions, val: &[HidDeviceFilter]);
}
#[cfg(web_sys_unstable_apis)]
impl HidDeviceRequestOptions {
    #[cfg(feature = "HidDeviceFilter")]
    #[doc = "Construct a new `HidDeviceRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`, `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(filters: &[HidDeviceFilter]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_filters(filters);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDeviceFilter")]
    #[deprecated = "Use `set_filters()` instead."]
    pub fn filters(&mut self, val: &[HidDeviceFilter]) -> &mut Self {
        self.set_filters(val);
        self
    }
}
