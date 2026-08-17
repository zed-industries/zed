#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "PresentationConnectionList",
        typescript_type = "PresentationConnectionList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    pub type PresentationConnectionList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnectionList",
        js_name = "connections"
    )]
    #[doc = "Getter for the `connections` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/connections)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    pub fn connections(this: &PresentationConnectionList) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnectionList",
        js_name = "onconnectionavailable"
    )]
    #[doc = "Getter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    pub fn onconnectionavailable(this: &PresentationConnectionList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationConnectionList",
        js_name = "onconnectionavailable"
    )]
    #[doc = "Setter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    pub fn set_onconnectionavailable(
        this: &PresentationConnectionList,
        value: Option<&::js_sys::Function>,
    );
}
