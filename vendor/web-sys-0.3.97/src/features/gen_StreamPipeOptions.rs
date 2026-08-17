#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "StreamPipeOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StreamPipeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub type StreamPipeOptions;
    #[doc = "Get the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    #[wasm_bindgen(method, getter = "preventAbort")]
    pub fn get_prevent_abort(this: &StreamPipeOptions) -> Option<bool>;
    #[doc = "Change the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    #[wasm_bindgen(method, setter = "preventAbort")]
    pub fn set_prevent_abort(this: &StreamPipeOptions, val: bool);
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    #[wasm_bindgen(method, getter = "preventCancel")]
    pub fn get_prevent_cancel(this: &StreamPipeOptions) -> Option<bool>;
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    #[wasm_bindgen(method, setter = "preventCancel")]
    pub fn set_prevent_cancel(this: &StreamPipeOptions, val: bool);
    #[doc = "Get the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    #[wasm_bindgen(method, getter = "preventClose")]
    pub fn get_prevent_close(this: &StreamPipeOptions) -> Option<bool>;
    #[doc = "Change the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    #[wasm_bindgen(method, setter = "preventClose")]
    pub fn set_prevent_close(this: &StreamPipeOptions, val: bool);
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    #[wasm_bindgen(method, getter = "signal")]
    pub fn get_signal(this: &StreamPipeOptions) -> Option<AbortSignal>;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    #[wasm_bindgen(method, setter = "signal")]
    pub fn set_signal(this: &StreamPipeOptions, val: &AbortSignal);
}
impl StreamPipeOptions {
    #[doc = "Construct a new `StreamPipeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_prevent_abort()` instead."]
    pub fn prevent_abort(&mut self, val: bool) -> &mut Self {
        self.set_prevent_abort(val);
        self
    }
    #[deprecated = "Use `set_prevent_cancel()` instead."]
    pub fn prevent_cancel(&mut self, val: bool) -> &mut Self {
        self.set_prevent_cancel(val);
        self
    }
    #[deprecated = "Use `set_prevent_close()` instead."]
    pub fn prevent_close(&mut self, val: bool) -> &mut Self {
        self.set_prevent_close(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal(val);
        self
    }
}
impl Default for StreamPipeOptions {
    fn default() -> Self {
        Self::new()
    }
}
