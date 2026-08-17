#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ReadableWritablePair")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableWritablePair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`*"]
    pub type ReadableWritablePair;
    #[cfg(feature = "ReadableStream")]
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    #[wasm_bindgen(method, getter = "readable")]
    pub fn get_readable(this: &ReadableWritablePair) -> ReadableStream;
    #[cfg(feature = "ReadableStream")]
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    #[wasm_bindgen(method, setter = "readable")]
    pub fn set_readable(this: &ReadableWritablePair, val: &ReadableStream);
    #[cfg(feature = "WritableStream")]
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &ReadableWritablePair) -> WritableStream;
    #[cfg(feature = "WritableStream")]
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    #[wasm_bindgen(method, setter = "writable")]
    pub fn set_writable(this: &ReadableWritablePair, val: &WritableStream);
}
impl ReadableWritablePair {
    #[cfg(all(feature = "ReadableStream", feature = "WritableStream",))]
    #[doc = "Construct a new `ReadableWritablePair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `WritableStream`*"]
    pub fn new(readable: &ReadableStream, writable: &WritableStream) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_readable(readable);
        ret.set_writable(writable);
        ret
    }
    #[cfg(feature = "ReadableStream")]
    #[deprecated = "Use `set_readable()` instead."]
    pub fn readable(&mut self, val: &ReadableStream) -> &mut Self {
        self.set_readable(val);
        self
    }
    #[cfg(feature = "WritableStream")]
    #[deprecated = "Use `set_writable()` instead."]
    pub fn writable(&mut self, val: &WritableStream) -> &mut Self {
        self.set_writable(val);
        self
    }
}
