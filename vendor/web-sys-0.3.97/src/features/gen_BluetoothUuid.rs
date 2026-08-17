#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BluetoothUUID",
        typescript_type = "BluetoothUUID"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothUuid` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothUuid;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "canonicalUUID"
    )]
    #[doc = "The `canonicalUUID()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/canonicalUUID_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn canonical_uuid(alias: u32) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "getCharacteristic"
    )]
    #[doc = "The `getCharacteristic()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getCharacteristic_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristic_with_str(name: &str) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "getCharacteristic"
    )]
    #[doc = "The `getCharacteristic()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getCharacteristic_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristic_with_u32(name: u32) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "getDescriptor"
    )]
    #[doc = "The `getDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getDescriptor_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptor_with_str(name: &str) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "getDescriptor"
    )]
    #[doc = "The `getDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getDescriptor_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptor_with_u32(name: u32) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "getService"
    )]
    #[doc = "The `getService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getService_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_service_with_str(name: &str) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "BluetoothUuid",
        js_class = "BluetoothUUID",
        js_name = "getService"
    )]
    #[doc = "The `getService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothUUID/getService_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothUuid`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_service_with_u32(name: u32) -> ::alloc::string::String;
}
