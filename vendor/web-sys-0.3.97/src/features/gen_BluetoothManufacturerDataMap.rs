#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BluetoothManufacturerDataMap",
        typescript_type = "BluetoothManufacturerDataMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothManufacturerDataMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothManufacturerDataMap;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothManufacturerDataMap",
        js_name = "size"
    )]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &BluetoothManufacturerDataMap) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothManufacturerDataMap",
        js_name = "forEach"
    )]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn for_each(
        this: &BluetoothManufacturerDataMap,
        callback: &::js_sys::Function<
            fn(::js_sys::DataView, ::js_sys::Number) -> ::js_sys::Undefined,
        >,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothManufacturerDataMap")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get(this: &BluetoothManufacturerDataMap, key: u16) -> Option<::js_sys::DataView>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothManufacturerDataMap")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has(this: &BluetoothManufacturerDataMap, key: u16) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothManufacturerDataMap")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(
        this: &BluetoothManufacturerDataMap,
    ) -> ::js_sys::Iterator<::js_sys::ArrayTuple<(::js_sys::Number, ::js_sys::DataView)>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothManufacturerDataMap")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &BluetoothManufacturerDataMap) -> ::js_sys::Iterator<::js_sys::Number>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothManufacturerDataMap")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothManufacturerDataMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn values(this: &BluetoothManufacturerDataMap) -> ::js_sys::Iterator<::js_sys::DataView>;
}
