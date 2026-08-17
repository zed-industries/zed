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
        js_name = "HIDDevice",
        typescript_type = "HIDDevice"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidDevice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidDevice;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDDevice", js_name = "oninputreport")]
    #[doc = "Getter for the `oninputreport` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/oninputreport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn oninputreport(this: &HidDevice) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "HIDDevice", js_name = "oninputreport")]
    #[doc = "Setter for the `oninputreport` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/oninputreport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_oninputreport(this: &HidDevice, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDDevice", js_name = "opened")]
    #[doc = "Getter for the `opened` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/opened)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn opened(this: &HidDevice) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDDevice", js_name = "vendorId")]
    #[doc = "Getter for the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/vendorId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vendor_id(this: &HidDevice) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDDevice", js_name = "productId")]
    #[doc = "Getter for the `productId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/productId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_id(this: &HidDevice) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDDevice", js_name = "productName")]
    #[doc = "Getter for the `productName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/productName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_name(this: &HidDevice) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidCollectionInfo")]
    #[wasm_bindgen(method, getter, js_class = "HIDDevice", js_name = "collections")]
    #[doc = "Getter for the `collections` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/collections)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn collections(this: &HidDevice) -> ::js_sys::Array<HidCollectionInfo>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HIDDevice")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &HidDevice) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HIDDevice")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn open(this: &HidDevice) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HIDDevice", js_name = "receiveFeatureReport")]
    #[doc = "The `receiveFeatureReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/receiveFeatureReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn receive_feature_report(
        this: &HidDevice,
        report_id: u8,
    ) -> ::js_sys::Promise<::js_sys::DataView>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "HIDDevice", js_name = "sendFeatureReport")]
    #[doc = "The `sendFeatureReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendFeatureReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_feature_report_with_buffer_source(
        this: &HidDevice,
        report_id: u8,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "HIDDevice", js_name = "sendFeatureReport")]
    #[doc = "The `sendFeatureReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendFeatureReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_feature_report_with_u8_slice(
        this: &HidDevice,
        report_id: u8,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "HIDDevice", js_name = "sendFeatureReport")]
    #[doc = "The `sendFeatureReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendFeatureReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_feature_report_with_u8_array(
        this: &HidDevice,
        report_id: u8,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "HIDDevice", js_name = "sendReport")]
    #[doc = "The `sendReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_report_with_buffer_source(
        this: &HidDevice,
        report_id: u8,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "HIDDevice", js_name = "sendReport")]
    #[doc = "The `sendReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_report_with_u8_slice(
        this: &HidDevice,
        report_id: u8,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "HIDDevice", js_name = "sendReport")]
    #[doc = "The `sendReport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDDevice/sendReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_report_with_u8_array(
        this: &HidDevice,
        report_id: u8,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<::js_sys::Undefined>, JsValue>;
}
