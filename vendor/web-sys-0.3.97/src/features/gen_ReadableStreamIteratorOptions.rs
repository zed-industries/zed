#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableStreamIteratorOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamIteratorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    pub type ReadableStreamIteratorOptions;
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    #[wasm_bindgen(method, getter = "preventCancel")]
    pub fn get_prevent_cancel(this: &ReadableStreamIteratorOptions) -> Option<bool>;
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    #[wasm_bindgen(method, setter = "preventCancel")]
    pub fn set_prevent_cancel(this: &ReadableStreamIteratorOptions, val: bool);
}
impl ReadableStreamIteratorOptions {
    #[doc = "Construct a new `ReadableStreamIteratorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_prevent_cancel()` instead."]
    pub fn prevent_cancel(&mut self, val: bool) -> &mut Self {
        self.set_prevent_cancel(val);
        self
    }
}
impl Default for ReadableStreamIteratorOptions {
    fn default() -> Self {
        Self::new()
    }
}
