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
        js_name = "BluetoothRemoteGATTService",
        typescript_type = "BluetoothRemoteGATTService"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothRemoteGattService` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothRemoteGattService;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "device"
    )]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDevice`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(this: &BluetoothRemoteGattService) -> BluetoothDevice;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "uuid"
    )]
    #[doc = "Getter for the `uuid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/uuid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn uuid(this: &BluetoothRemoteGattService) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "isPrimary"
    )]
    #[doc = "Getter for the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/isPrimary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_primary(this: &BluetoothRemoteGattService) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "oncharacteristicvaluechanged"
    )]
    #[doc = "Getter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn oncharacteristicvaluechanged(
        this: &BluetoothRemoteGattService,
    ) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "oncharacteristicvaluechanged"
    )]
    #[doc = "Setter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_oncharacteristicvaluechanged(
        this: &BluetoothRemoteGattService,
        value: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "onserviceadded"
    )]
    #[doc = "Getter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onserviceadded(this: &BluetoothRemoteGattService) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "onserviceadded"
    )]
    #[doc = "Setter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onserviceadded(
        this: &BluetoothRemoteGattService,
        value: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "onservicechanged"
    )]
    #[doc = "Getter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onservicechanged(this: &BluetoothRemoteGattService) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "onservicechanged"
    )]
    #[doc = "Setter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onservicechanged(
        this: &BluetoothRemoteGattService,
        value: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "onserviceremoved"
    )]
    #[doc = "Getter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onserviceremoved(this: &BluetoothRemoteGattService) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BluetoothRemoteGATTService",
        js_name = "onserviceremoved"
    )]
    #[doc = "Setter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onserviceremoved(
        this: &BluetoothRemoteGattService,
        value: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattCharacteristic")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getCharacteristic"
    )]
    #[doc = "The `getCharacteristic()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristic_with_str(
        this: &BluetoothRemoteGattService,
        characteristic: &str,
    ) -> ::js_sys::Promise<BluetoothRemoteGattCharacteristic>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattCharacteristic")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getCharacteristic"
    )]
    #[doc = "The `getCharacteristic()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristic_with_u32(
        this: &BluetoothRemoteGattService,
        characteristic: u32,
    ) -> ::js_sys::Promise<BluetoothRemoteGattCharacteristic>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattCharacteristic")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getCharacteristics"
    )]
    #[doc = "The `getCharacteristics()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristics(
        this: &BluetoothRemoteGattService,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattCharacteristic>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattCharacteristic")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getCharacteristics"
    )]
    #[doc = "The `getCharacteristics()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristics_with_str(
        this: &BluetoothRemoteGattService,
        characteristic: &str,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattCharacteristic>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattCharacteristic")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getCharacteristics"
    )]
    #[doc = "The `getCharacteristics()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_characteristics_with_u32(
        this: &BluetoothRemoteGattService,
        characteristic: u32,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattCharacteristic>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getIncludedService"
    )]
    #[doc = "The `getIncludedService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_included_service_with_str(
        this: &BluetoothRemoteGattService,
        service: &str,
    ) -> ::js_sys::Promise<BluetoothRemoteGattService>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getIncludedService"
    )]
    #[doc = "The `getIncludedService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_included_service_with_u32(
        this: &BluetoothRemoteGattService,
        service: u32,
    ) -> ::js_sys::Promise<BluetoothRemoteGattService>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getIncludedServices"
    )]
    #[doc = "The `getIncludedServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_included_services(
        this: &BluetoothRemoteGattService,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattService>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getIncludedServices"
    )]
    #[doc = "The `getIncludedServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_included_services_with_str(
        this: &BluetoothRemoteGattService,
        service: &str,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattService>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTService",
        js_name = "getIncludedServices"
    )]
    #[doc = "The `getIncludedServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_included_services_with_u32(
        this: &BluetoothRemoteGattService,
        service: u32,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattService>>;
}
