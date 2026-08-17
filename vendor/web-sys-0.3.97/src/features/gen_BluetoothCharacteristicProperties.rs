#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BluetoothCharacteristicProperties",
        typescript_type = "BluetoothCharacteristicProperties"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothCharacteristicProperties` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothCharacteristicProperties;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "broadcast"
    )]
    #[doc = "Getter for the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/broadcast)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn broadcast(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "read"
    )]
    #[doc = "Getter for the `read` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn read(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "writeWithoutResponse"
    )]
    #[doc = "Getter for the `writeWithoutResponse` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/writeWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_without_response(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "write"
    )]
    #[doc = "Getter for the `write` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "notify"
    )]
    #[doc = "Getter for the `notify` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/notify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn notify(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "indicate"
    )]
    #[doc = "Getter for the `indicate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/indicate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn indicate(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "authenticatedSignedWrites"
    )]
    #[doc = "Getter for the `authenticatedSignedWrites` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/authenticatedSignedWrites)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn authenticated_signed_writes(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "reliableWrite"
    )]
    #[doc = "Getter for the `reliableWrite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/reliableWrite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reliable_write(this: &BluetoothCharacteristicProperties) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothCharacteristicProperties",
        js_name = "writableAuxiliaries"
    )]
    #[doc = "Getter for the `writableAuxiliaries` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothCharacteristicProperties/writableAuxiliaries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable_auxiliaries(this: &BluetoothCharacteristicProperties) -> bool;
}
