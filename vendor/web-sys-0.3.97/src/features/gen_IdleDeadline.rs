#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IdleDeadline",
        typescript_type = "IdleDeadline"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdleDeadline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleDeadline`*"]
    pub type IdleDeadline;
    #[wasm_bindgen(method, getter, js_class = "IdleDeadline", js_name = "didTimeout")]
    #[doc = "Getter for the `didTimeout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/didTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleDeadline`*"]
    pub fn did_timeout(this: &IdleDeadline) -> bool;
    #[wasm_bindgen(method, js_class = "IdleDeadline", js_name = "timeRemaining")]
    #[doc = "The `timeRemaining()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/timeRemaining)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleDeadline`*"]
    pub fn time_remaining(this: &IdleDeadline) -> f64;
}
