#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "NavigatorUAData",
        typescript_type = "NavigatorUAData"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigatorUaData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type NavigatorUaData;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[wasm_bindgen(method, getter, js_class = "NavigatorUAData", js_name = "brands")]
    #[doc = "Getter for the `brands` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/brands)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`, `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn brands(this: &NavigatorUaData) -> ::js_sys::Array<NavigatorUaBrandVersion>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "NavigatorUAData", js_name = "mobile")]
    #[doc = "Getter for the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/mobile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mobile(this: &NavigatorUaData) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "NavigatorUAData", js_name = "platform")]
    #[doc = "Getter for the `platform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/platform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn platform(this: &NavigatorUaData) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UaDataValues")]
    #[wasm_bindgen(method, js_class = "NavigatorUAData", js_name = "getHighEntropyValues")]
    #[doc = "The `getHighEntropyValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/getHighEntropyValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`, `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_high_entropy_values(
        this: &NavigatorUaData,
        hints: &[::js_sys::JsString],
    ) -> ::js_sys::Promise<UaDataValues>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UaLowEntropyJson")]
    #[wasm_bindgen(method, js_class = "NavigatorUAData", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`, `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn to_json(this: &NavigatorUaData) -> UaLowEntropyJson;
}
