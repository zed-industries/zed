#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Bluetooth",
        typescript_type = "Bluetooth"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Bluetooth` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Bluetooth;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Bluetooth",
        js_name = "onavailabilitychanged"
    )]
    #[doc = "Getter for the `onavailabilitychanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onavailabilitychanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onavailabilitychanged(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Bluetooth",
        js_name = "onavailabilitychanged"
    )]
    #[doc = "Setter for the `onavailabilitychanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onavailabilitychanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onavailabilitychanged(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(method, getter, js_class = "Bluetooth", js_name = "referringDevice")]
    #[doc = "Getter for the `referringDevice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/referringDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn referring_device(this: &Bluetooth) -> Option<BluetoothDevice>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Bluetooth",
        js_name = "onadvertisementreceived"
    )]
    #[doc = "Getter for the `onadvertisementreceived` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onadvertisementreceived)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onadvertisementreceived(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Bluetooth",
        js_name = "onadvertisementreceived"
    )]
    #[doc = "Setter for the `onadvertisementreceived` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onadvertisementreceived)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onadvertisementreceived(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Bluetooth",
        js_name = "ongattserverdisconnected"
    )]
    #[doc = "Getter for the `ongattserverdisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/ongattserverdisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongattserverdisconnected(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Bluetooth",
        js_name = "ongattserverdisconnected"
    )]
    #[doc = "Setter for the `ongattserverdisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/ongattserverdisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongattserverdisconnected(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Bluetooth",
        js_name = "oncharacteristicvaluechanged"
    )]
    #[doc = "Getter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn oncharacteristicvaluechanged(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Bluetooth",
        js_name = "oncharacteristicvaluechanged"
    )]
    #[doc = "Setter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_oncharacteristicvaluechanged(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Bluetooth", js_name = "onserviceadded")]
    #[doc = "Getter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onserviceadded(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Bluetooth", js_name = "onserviceadded")]
    #[doc = "Setter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onserviceadded(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Bluetooth", js_name = "onservicechanged")]
    #[doc = "Getter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onservicechanged(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Bluetooth", js_name = "onservicechanged")]
    #[doc = "Setter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onservicechanged(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Bluetooth", js_name = "onserviceremoved")]
    #[doc = "Getter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onserviceremoved(this: &Bluetooth) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Bluetooth", js_name = "onserviceremoved")]
    #[doc = "Setter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onserviceremoved(this: &Bluetooth, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "Bluetooth", js_name = "getAvailability")]
    #[doc = "The `getAvailability()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/getAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_availability(this: &Bluetooth) -> ::js_sys::Promise<::js_sys::Boolean>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(method, js_class = "Bluetooth", js_name = "getDevices")]
    #[doc = "The `getDevices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/getDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_devices(this: &Bluetooth) -> ::js_sys::Promise<::js_sys::Array<BluetoothDevice>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "BluetoothDevice", feature = "RequestDeviceOptions",))]
    #[wasm_bindgen(method, js_class = "Bluetooth", js_name = "requestDevice")]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `BluetoothDevice`, `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device(
        this: &Bluetooth,
        options: &RequestDeviceOptions,
    ) -> ::js_sys::Promise<BluetoothDevice>;
}
