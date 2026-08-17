#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoFrameCopyToOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrameCopyToOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
    pub type VideoFrameCopyToOptions;
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`, `VideoPixelFormat`*"]
    #[wasm_bindgen(method, getter = "format")]
    pub fn get_format(this: &VideoFrameCopyToOptions) -> Option<VideoPixelFormat>;
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`, `VideoPixelFormat`*"]
    #[wasm_bindgen(method, setter = "format")]
    pub fn set_format(this: &VideoFrameCopyToOptions, val: VideoPixelFormat);
    #[cfg(feature = "PlaneLayout")]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrameCopyToOptions`*"]
    #[wasm_bindgen(method, getter = "layout")]
    pub fn get_layout(this: &VideoFrameCopyToOptions) -> Option<::js_sys::Array<PlaneLayout>>;
    #[cfg(feature = "PlaneLayout")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrameCopyToOptions`*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout(this: &VideoFrameCopyToOptions, val: &[PlaneLayout]);
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `rect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameCopyToOptions`*"]
    #[wasm_bindgen(method, getter = "rect")]
    pub fn get_rect(this: &VideoFrameCopyToOptions) -> Option<DomRectInit>;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `rect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameCopyToOptions`*"]
    #[wasm_bindgen(method, setter = "rect")]
    pub fn set_rect(this: &VideoFrameCopyToOptions, val: &DomRectInit);
}
impl VideoFrameCopyToOptions {
    #[doc = "Construct a new `VideoFrameCopyToOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameCopyToOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "VideoPixelFormat")]
    #[deprecated = "Use `set_format()` instead."]
    pub fn format(&mut self, val: VideoPixelFormat) -> &mut Self {
        self.set_format(val);
        self
    }
    #[cfg(feature = "PlaneLayout")]
    #[deprecated = "Use `set_layout()` instead."]
    pub fn layout(&mut self, val: &[PlaneLayout]) -> &mut Self {
        self.set_layout(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[deprecated = "Use `set_rect()` instead."]
    pub fn rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_rect(val);
        self
    }
}
impl Default for VideoFrameCopyToOptions {
    fn default() -> Self {
        Self::new()
    }
}
