#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "PromiseNativeHandler" , typescript_type = "PromiseNativeHandler")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseNativeHandler` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseNativeHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseNativeHandler`*"]
    pub type PromiseNativeHandler;
}
