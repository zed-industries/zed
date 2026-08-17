#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PresentationReceiver",
        typescript_type = "PresentationReceiver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationReceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    pub type PresentationReceiver;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "PresentationReceiver",
        js_name = "connectionList"
    )]
    #[doc = "Getter for the `connectionList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    pub fn connection_list(this: &PresentationReceiver) -> Result<::js_sys::Promise, JsValue>;
}
