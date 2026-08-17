#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "BluetoothDataFilterInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothDataFilterInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothDataFilterInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dataPrefix")]
    pub fn get_data_prefix(this: &BluetoothDataFilterInit) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dataPrefix")]
    pub fn set_data_prefix(this: &BluetoothDataFilterInit, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "The `val` slice must outlive any use of the dictionary on the JavaScript side. wasm-bindgen passes a raw view into wasm linear memory — if the underlying data is freed or moved before JS is done reading the field, the behaviour is undefined. Prefer the `_array` variant (takes `&Uint8Array`) for a safe alternative that copies the data."]
    #[wasm_bindgen(method, setter = "dataPrefix")]
    pub unsafe fn set_data_prefix_u8_slice(this: &BluetoothDataFilterInit, val: &mut [u8]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dataPrefix")]
    pub fn set_data_prefix_u8_array(this: &BluetoothDataFilterInit, val: &::js_sys::Uint8Array);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mask")]
    pub fn get_mask(this: &BluetoothDataFilterInit) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mask")]
    pub fn set_mask(this: &BluetoothDataFilterInit, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "The `val` slice must outlive any use of the dictionary on the JavaScript side. wasm-bindgen passes a raw view into wasm linear memory — if the underlying data is freed or moved before JS is done reading the field, the behaviour is undefined. Prefer the `_array` variant (takes `&Uint8Array`) for a safe alternative that copies the data."]
    #[wasm_bindgen(method, setter = "mask")]
    pub unsafe fn set_mask_u8_slice(this: &BluetoothDataFilterInit, val: &mut [u8]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mask")]
    pub fn set_mask_u8_array(this: &BluetoothDataFilterInit, val: &::js_sys::Uint8Array);
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothDataFilterInit {
    #[doc = "Construct a new `BluetoothDataFilterInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data_prefix()` instead."]
    pub fn data_prefix(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_data_prefix(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mask()` instead."]
    pub fn mask(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_mask(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for BluetoothDataFilterInit {
    fn default() -> Self {
        Self::new()
    }
}
