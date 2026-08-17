#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RequestDeviceOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestDeviceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RequestDeviceOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `acceptAllDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "acceptAllDevices")]
    pub fn get_accept_all_devices(this: &RequestDeviceOptions) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `acceptAllDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "acceptAllDevices")]
    pub fn set_accept_all_devices(this: &RequestDeviceOptions, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothLeScanFilterInit")]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`, `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(
        this: &RequestDeviceOptions,
    ) -> Option<::js_sys::Array<BluetoothLeScanFilterInit>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothLeScanFilterInit")]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothLeScanFilterInit`, `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &RequestDeviceOptions, val: &[BluetoothLeScanFilterInit]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "optionalServices")]
    pub fn get_optional_services(this: &RequestDeviceOptions) -> Option<::js_sys::Array>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optionalServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "optionalServices")]
    pub fn set_optional_services(this: &RequestDeviceOptions, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optionalServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "optionalServices")]
    pub fn set_optional_services_u32_sequence(
        this: &RequestDeviceOptions,
        val: &[::js_sys::Number],
    );
}
#[cfg(web_sys_unstable_apis)]
impl RequestDeviceOptions {
    #[doc = "Construct a new `RequestDeviceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_accept_all_devices()` instead."]
    pub fn accept_all_devices(&mut self, val: bool) -> &mut Self {
        self.set_accept_all_devices(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothLeScanFilterInit")]
    #[deprecated = "Use `set_filters()` instead."]
    pub fn filters(&mut self, val: &[BluetoothLeScanFilterInit]) -> &mut Self {
        self.set_filters(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_optional_services()` instead."]
    pub fn optional_services(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_optional_services(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RequestDeviceOptions {
    fn default() -> Self {
        Self::new()
    }
}
