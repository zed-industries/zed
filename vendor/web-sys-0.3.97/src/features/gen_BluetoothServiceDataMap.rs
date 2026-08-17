#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BluetoothServiceDataMap",
        typescript_type = "BluetoothServiceDataMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothServiceDataMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothServiceDataMap;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "BluetoothServiceDataMap", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &BluetoothServiceDataMap) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothServiceDataMap",
        js_name = "forEach"
    )]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn for_each(
        this: &BluetoothServiceDataMap,
        callback: &::js_sys::Function<
            fn(::js_sys::DataView, ::js_sys::JsString) -> ::js_sys::Undefined,
        >,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothServiceDataMap")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get(this: &BluetoothServiceDataMap, key: &str) -> Option<::js_sys::DataView>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothServiceDataMap")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has(this: &BluetoothServiceDataMap, key: &str) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothServiceDataMap")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(
        this: &BluetoothServiceDataMap,
    ) -> ::js_sys::Iterator<::js_sys::ArrayTuple<(::js_sys::JsString, ::js_sys::DataView)>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothServiceDataMap")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &BluetoothServiceDataMap) -> ::js_sys::Iterator<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothServiceDataMap")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn values(this: &BluetoothServiceDataMap) -> ::js_sys::Iterator<::js_sys::DataView>;
}
