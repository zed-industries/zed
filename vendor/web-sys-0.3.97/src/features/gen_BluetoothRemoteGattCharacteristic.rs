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
        js_name = "BluetoothRemoteGATTCharacteristic",
        typescript_type = "BluetoothRemoteGATTCharacteristic"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothRemoteGattCharacteristic` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothRemoteGattCharacteristic;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "service"
    )]
    #[doc = "Getter for the `service` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/service)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn service(this: &BluetoothRemoteGattCharacteristic) -> BluetoothRemoteGattService;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "uuid"
    )]
    #[doc = "Getter for the `uuid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/uuid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn uuid(this: &BluetoothRemoteGattCharacteristic) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothCharacteristicProperties")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "properties"
    )]
    #[doc = "Getter for the `properties` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/properties)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`, `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn properties(
        this: &BluetoothRemoteGattCharacteristic,
    ) -> BluetoothCharacteristicProperties;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "value"
    )]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn value(this: &BluetoothRemoteGattCharacteristic) -> Option<::js_sys::DataView>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "oncharacteristicvaluechanged"
    )]
    #[doc = "Getter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn oncharacteristicvaluechanged(
        this: &BluetoothRemoteGattCharacteristic,
    ) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "oncharacteristicvaluechanged"
    )]
    #[doc = "Setter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_oncharacteristicvaluechanged(
        this: &BluetoothRemoteGattCharacteristic,
        value: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattDescriptor")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "getDescriptor"
    )]
    #[doc = "The `getDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptor_with_str(
        this: &BluetoothRemoteGattCharacteristic,
        descriptor: &str,
    ) -> ::js_sys::Promise<BluetoothRemoteGattDescriptor>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattDescriptor")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "getDescriptor"
    )]
    #[doc = "The `getDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptor_with_u32(
        this: &BluetoothRemoteGattCharacteristic,
        descriptor: u32,
    ) -> ::js_sys::Promise<BluetoothRemoteGattDescriptor>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattDescriptor")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "getDescriptors"
    )]
    #[doc = "The `getDescriptors()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptors(
        this: &BluetoothRemoteGattCharacteristic,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattDescriptor>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattDescriptor")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "getDescriptors"
    )]
    #[doc = "The `getDescriptors()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptors_with_str(
        this: &BluetoothRemoteGattCharacteristic,
        descriptor: &str,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattDescriptor>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattDescriptor")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "getDescriptors"
    )]
    #[doc = "The `getDescriptors()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_descriptors_with_u32(
        this: &BluetoothRemoteGattCharacteristic,
        descriptor: u32,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattDescriptor>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "readValue"
    )]
    #[doc = "The `readValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/readValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn read_value(
        this: &BluetoothRemoteGattCharacteristic,
    ) -> ::js_sys::Promise<::js_sys::DataView>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "startNotifications"
    )]
    #[doc = "The `startNotifications()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/startNotifications)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn start_notifications(
        this: &BluetoothRemoteGattCharacteristic,
    ) -> ::js_sys::Promise<BluetoothRemoteGattCharacteristic>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "stopNotifications"
    )]
    #[doc = "The `stopNotifications()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/stopNotifications)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stop_notifications(
        this: &BluetoothRemoteGattCharacteristic,
    ) -> ::js_sys::Promise<BluetoothRemoteGattCharacteristic>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValue"
    )]
    #[doc = "The `writeValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_with_buffer_source(
        this: &BluetoothRemoteGattCharacteristic,
        value: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValue"
    )]
    #[doc = "The `writeValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_with_u8_slice(
        this: &BluetoothRemoteGattCharacteristic,
        value: &mut [u8],
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValue"
    )]
    #[doc = "The `writeValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_with_u8_array(
        this: &BluetoothRemoteGattCharacteristic,
        value: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValueWithResponse"
    )]
    #[doc = "The `writeValueWithResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_with_response_with_buffer_source(
        this: &BluetoothRemoteGattCharacteristic,
        value: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValueWithResponse"
    )]
    #[doc = "The `writeValueWithResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_with_response_with_u8_slice(
        this: &BluetoothRemoteGattCharacteristic,
        value: &mut [u8],
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValueWithResponse"
    )]
    #[doc = "The `writeValueWithResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_with_response_with_u8_array(
        this: &BluetoothRemoteGattCharacteristic,
        value: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValueWithoutResponse"
    )]
    #[doc = "The `writeValueWithoutResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_without_response_with_buffer_source(
        this: &BluetoothRemoteGattCharacteristic,
        value: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValueWithoutResponse"
    )]
    #[doc = "The `writeValueWithoutResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_without_response_with_u8_slice(
        this: &BluetoothRemoteGattCharacteristic,
        value: &mut [u8],
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "BluetoothRemoteGATTCharacteristic",
        js_name = "writeValueWithoutResponse"
    )]
    #[doc = "The `writeValueWithoutResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_value_without_response_with_u8_array(
        this: &BluetoothRemoteGattCharacteristic,
        value: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
}
