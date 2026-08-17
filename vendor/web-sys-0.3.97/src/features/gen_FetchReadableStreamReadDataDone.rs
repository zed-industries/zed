#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FetchReadableStreamReadDataDone"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchReadableStreamReadDataDone` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    pub type FetchReadableStreamReadDataDone;
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    #[wasm_bindgen(method, getter = "done")]
    pub fn get_done(this: &FetchReadableStreamReadDataDone) -> Option<bool>;
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    #[wasm_bindgen(method, setter = "done")]
    pub fn set_done(this: &FetchReadableStreamReadDataDone, val: bool);
}
impl FetchReadableStreamReadDataDone {
    #[doc = "Construct a new `FetchReadableStreamReadDataDone`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_done()` instead."]
    pub fn done(&mut self, val: bool) -> &mut Self {
        self.set_done(val);
        self
    }
}
impl Default for FetchReadableStreamReadDataDone {
    fn default() -> Self {
        Self::new()
    }
}
