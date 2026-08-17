#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SFrameTransformErrorEventInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransformErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SFrameTransformErrorEventInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SFrameTransformErrorEventInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &SFrameTransformErrorEventInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SFrameTransformErrorEventInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &SFrameTransformErrorEventInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SFrameTransformErrorEventInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &SFrameTransformErrorEventInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventType")]
    #[doc = "Get the `errorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`, `SFrameTransformErrorEventType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "errorType")]
    pub fn get_error_type(this: &SFrameTransformErrorEventInit) -> SFrameTransformErrorEventType;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventType")]
    #[doc = "Change the `errorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`, `SFrameTransformErrorEventType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "errorType")]
    pub fn set_error_type(this: &SFrameTransformErrorEventInit, val: SFrameTransformErrorEventType);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `frame` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "frame")]
    pub fn get_frame(this: &SFrameTransformErrorEventInit) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `frame` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "frame")]
    pub fn set_frame(this: &SFrameTransformErrorEventInit, val: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `keyID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "keyID")]
    pub fn get_key_id(this: &SFrameTransformErrorEventInit) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `keyID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "keyID")]
    pub fn set_key_id(this: &SFrameTransformErrorEventInit, val: Option<u32>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `keyID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "keyID")]
    pub fn set_key_id_opt_f64(this: &SFrameTransformErrorEventInit, val: Option<f64>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `keyID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "keyID")]
    pub fn set_key_id_opt_big_int(
        this: &SFrameTransformErrorEventInit,
        val: Option<&::js_sys::BigInt>,
    );
}
#[cfg(web_sys_unstable_apis)]
impl SFrameTransformErrorEventInit {
    #[cfg(feature = "SFrameTransformErrorEventType")]
    #[doc = "Construct a new `SFrameTransformErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEventInit`, `SFrameTransformErrorEventType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(error_type: SFrameTransformErrorEventType, frame: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_error_type(error_type);
        ret.set_frame(frame);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventType")]
    #[deprecated = "Use `set_error_type()` instead."]
    pub fn error_type(&mut self, val: SFrameTransformErrorEventType) -> &mut Self {
        self.set_error_type(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_frame()` instead."]
    pub fn frame(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_frame(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_key_id()` instead."]
    pub fn key_id(&mut self, val: Option<u32>) -> &mut Self {
        self.set_key_id(val);
        self
    }
}
