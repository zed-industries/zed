#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Animation",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "CSSTransition",
        typescript_type = "CSSTransition"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssTransition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssTransition`*"]
    pub type CssTransition;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSTransition",
        js_name = "transitionProperty"
    )]
    #[doc = "Getter for the `transitionProperty` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition/transitionProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssTransition`*"]
    pub fn transition_property(this: &CssTransition) -> ::alloc::string::String;
}
