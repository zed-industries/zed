#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUCopyExternalImageSourceInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCopyExternalImageSourceInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCopyExternalImageSourceInfo;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `flipY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "flipY")]
    pub fn get_flip_y(this: &GpuCopyExternalImageSourceInfo) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `flipY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "flipY")]
    pub fn set_flip_y(this: &GpuCopyExternalImageSourceInfo, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &GpuCopyExternalImageSourceInfo) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &GpuCopyExternalImageSourceInfo, val: &[::js_sys::Number]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuOrigin2dDict")]
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `GpuOrigin2dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin_gpu_origin_2d_dict(
        this: &GpuCopyExternalImageSourceInfo,
        val: &GpuOrigin2dDict,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageBitmap")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `ImageBitmap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &GpuCopyExternalImageSourceInfo) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageBitmap")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `ImageBitmap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &GpuCopyExternalImageSourceInfo, val: &ImageBitmap);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageData")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `ImageData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_image_data(this: &GpuCopyExternalImageSourceInfo, val: &ImageData);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `HtmlImageElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_html_image_element(
        this: &GpuCopyExternalImageSourceInfo,
        val: &HtmlImageElement,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_html_video_element(
        this: &GpuCopyExternalImageSourceInfo,
        val: &HtmlVideoElement,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_video_frame(this: &GpuCopyExternalImageSourceInfo, val: &VideoFrame);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `HtmlCanvasElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_html_canvas_element(
        this: &GpuCopyExternalImageSourceInfo,
        val: &HtmlCanvasElement,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `OffscreenCanvas`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_offscreen_canvas(
        this: &GpuCopyExternalImageSourceInfo,
        val: &OffscreenCanvas,
    );
}
#[cfg(web_sys_unstable_apis)]
impl GpuCopyExternalImageSourceInfo {
    #[cfg(feature = "ImageBitmap")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `ImageBitmap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(source: &ImageBitmap) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source(source);
        ret
    }
    #[cfg(feature = "ImageData")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `ImageData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_image_data(source: &ImageData) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source_image_data(source);
        ret
    }
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `HtmlImageElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_html_image_element(source: &HtmlImageElement) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source_html_image_element(source);
        ret
    }
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_html_video_element(source: &HtmlVideoElement) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source_html_video_element(source);
        ret
    }
    #[cfg(feature = "VideoFrame")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_video_frame(source: &VideoFrame) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source_video_frame(source);
        ret
    }
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `HtmlCanvasElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_html_canvas_element(source: &HtmlCanvasElement) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source_html_canvas_element(source);
        ret
    }
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "Construct a new `GpuCopyExternalImageSourceInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`, `OffscreenCanvas`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_offscreen_canvas(source: &OffscreenCanvas) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_source_offscreen_canvas(source);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_flip_y()` instead."]
    pub fn flip_y(&mut self, val: bool) -> &mut Self {
        self.set_flip_y(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: &[::js_sys::Number]) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageBitmap")]
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: &ImageBitmap) -> &mut Self {
        self.set_source(val);
        self
    }
}
