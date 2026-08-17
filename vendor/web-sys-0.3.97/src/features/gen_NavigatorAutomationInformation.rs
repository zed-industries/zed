#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "NavigatorAutomationInformation" , typescript_type = "NavigatorAutomationInformation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigatorAutomationInformation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
    pub type NavigatorAutomationInformation;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "NavigatorAutomationInformation",
        js_name = "webdriver"
    )]
    #[doc = "Getter for the `webdriver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation/webdriver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
    pub fn webdriver(this: &NavigatorAutomationInformation) -> bool;
}
