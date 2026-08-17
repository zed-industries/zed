#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "NetworkInformation",
        typescript_type = "NetworkInformation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkInformation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`*"]
    pub type NetworkInformation;
    #[cfg(feature = "ConnectionType")]
    #[wasm_bindgen(method, getter, js_class = "NetworkInformation", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnectionType`, `NetworkInformation`*"]
    pub fn type_(this: &NetworkInformation) -> ConnectionType;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NetworkInformation",
        js_name = "ontypechange"
    )]
    #[doc = "Getter for the `ontypechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`*"]
    pub fn ontypechange(this: &NetworkInformation) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "NetworkInformation",
        js_name = "ontypechange"
    )]
    #[doc = "Setter for the `ontypechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`*"]
    pub fn set_ontypechange(this: &NetworkInformation, value: Option<&::js_sys::Function>);
}
