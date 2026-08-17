#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "BluetoothAdvertisingEvent",
        typescript_type = "BluetoothAdvertisingEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothAdvertisingEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothAdvertisingEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "device"
    )]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(this: &BluetoothAdvertisingEvent) -> BluetoothDevice;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "uuids"
    )]
    #[doc = "Getter for the `uuids` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/uuids)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn uuids(this: &BluetoothAdvertisingEvent) -> ::js_sys::Array<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "name"
    )]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(this: &BluetoothAdvertisingEvent) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "appearance"
    )]
    #[doc = "Getter for the `appearance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/appearance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn appearance(this: &BluetoothAdvertisingEvent) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "txPower"
    )]
    #[doc = "Getter for the `txPower` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/txPower)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn tx_power(this: &BluetoothAdvertisingEvent) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "rssi"
    )]
    #[doc = "Getter for the `rssi` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/rssi)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rssi(this: &BluetoothAdvertisingEvent) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "manufacturerData"
    )]
    #[doc = "Getter for the `manufacturerData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/manufacturerData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn manufacturer_data(this: &BluetoothAdvertisingEvent) -> BluetoothManufacturerDataMap;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothServiceDataMap")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothAdvertisingEvent",
        js_name = "serviceData"
    )]
    #[doc = "Getter for the `serviceData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/serviceData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn service_data(this: &BluetoothAdvertisingEvent) -> BluetoothServiceDataMap;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothAdvertisingEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "BluetoothAdvertisingEvent")]
    #[doc = "The `new BluetoothAdvertisingEvent(..)` constructor, creating a new instance of `BluetoothAdvertisingEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/BluetoothAdvertisingEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        init: &BluetoothAdvertisingEventInit,
    ) -> Result<BluetoothAdvertisingEvent, JsValue>;
}
