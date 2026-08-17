#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoColorSpaceInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoColorSpaceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    pub type VideoColorSpaceInit;
    #[doc = "Get the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[wasm_bindgen(method, getter = "fullRange")]
    pub fn get_full_range(this: &VideoColorSpaceInit) -> Option<bool>;
    #[doc = "Change the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[wasm_bindgen(method, setter = "fullRange")]
    pub fn set_full_range(this: &VideoColorSpaceInit, val: Option<bool>);
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Get the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[wasm_bindgen(method, getter = "matrix")]
    pub fn get_matrix(this: &VideoColorSpaceInit) -> Option<VideoMatrixCoefficients>;
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Change the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[wasm_bindgen(method, setter = "matrix")]
    pub fn set_matrix(this: &VideoColorSpaceInit, val: Option<VideoMatrixCoefficients>);
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Get the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[wasm_bindgen(method, getter = "primaries")]
    pub fn get_primaries(this: &VideoColorSpaceInit) -> Option<VideoColorPrimaries>;
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Change the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[wasm_bindgen(method, setter = "primaries")]
    pub fn set_primaries(this: &VideoColorSpaceInit, val: Option<VideoColorPrimaries>);
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Get the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[wasm_bindgen(method, getter = "transfer")]
    pub fn get_transfer(this: &VideoColorSpaceInit) -> Option<VideoTransferCharacteristics>;
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Change the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[wasm_bindgen(method, setter = "transfer")]
    pub fn set_transfer(this: &VideoColorSpaceInit, val: Option<VideoTransferCharacteristics>);
}
impl VideoColorSpaceInit {
    #[doc = "Construct a new `VideoColorSpaceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_full_range()` instead."]
    pub fn full_range(&mut self, val: Option<bool>) -> &mut Self {
        self.set_full_range(val);
        self
    }
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[deprecated = "Use `set_matrix()` instead."]
    pub fn matrix(&mut self, val: Option<VideoMatrixCoefficients>) -> &mut Self {
        self.set_matrix(val);
        self
    }
    #[cfg(feature = "VideoColorPrimaries")]
    #[deprecated = "Use `set_primaries()` instead."]
    pub fn primaries(&mut self, val: Option<VideoColorPrimaries>) -> &mut Self {
        self.set_primaries(val);
        self
    }
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[deprecated = "Use `set_transfer()` instead."]
    pub fn transfer(&mut self, val: Option<VideoTransferCharacteristics>) -> &mut Self {
        self.set_transfer(val);
        self
    }
}
impl Default for VideoColorSpaceInit {
    fn default() -> Self {
        Self::new()
    }
}
