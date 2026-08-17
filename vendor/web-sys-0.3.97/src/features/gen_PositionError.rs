#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "PositionError" , typescript_type = "PositionError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PositionError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub type PositionError;
    #[wasm_bindgen(method, getter, js_class = "PositionError", js_name = "code")]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub fn code(this: &PositionError) -> u16;
    #[wasm_bindgen(method, getter, js_class = "PositionError", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub fn message(this: &PositionError) -> ::alloc::string::String;
}
impl PositionError {
    #[doc = "The `PositionError.PERMISSION_DENIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub const PERMISSION_DENIED: u16 = 1u64 as u16;
    #[doc = "The `PositionError.POSITION_UNAVAILABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub const POSITION_UNAVAILABLE: u16 = 2u64 as u16;
    #[doc = "The `PositionError.TIMEOUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub const TIMEOUT: u16 = 3u64 as u16;
}
