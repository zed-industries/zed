#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoFrameBufferInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrameBufferInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    pub type VideoFrameBufferInit;
    #[doc = "Get the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "codedHeight")]
    pub fn get_coded_height(this: &VideoFrameBufferInit) -> u32;
    #[doc = "Change the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "codedHeight")]
    pub fn set_coded_height(this: &VideoFrameBufferInit, val: u32);
    #[doc = "Get the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "codedWidth")]
    pub fn get_coded_width(this: &VideoFrameBufferInit) -> u32;
    #[doc = "Change the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "codedWidth")]
    pub fn set_coded_width(this: &VideoFrameBufferInit, val: u32);
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "Get the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "colorSpace")]
    pub fn get_color_space(this: &VideoFrameBufferInit) -> Option<VideoColorSpaceInit>;
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "Change the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "colorSpace")]
    pub fn set_color_space(this: &VideoFrameBufferInit, val: &VideoColorSpaceInit);
    #[doc = "Get the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "displayHeight")]
    pub fn get_display_height(this: &VideoFrameBufferInit) -> Option<u32>;
    #[doc = "Change the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "displayHeight")]
    pub fn set_display_height(this: &VideoFrameBufferInit, val: u32);
    #[doc = "Get the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "displayWidth")]
    pub fn get_display_width(this: &VideoFrameBufferInit) -> Option<u32>;
    #[doc = "Change the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "displayWidth")]
    pub fn set_display_width(this: &VideoFrameBufferInit, val: u32);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &VideoFrameBufferInit) -> Option<f64>;
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &VideoFrameBufferInit, val: u32);
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration_f64(this: &VideoFrameBufferInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `flip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "flip")]
    pub fn get_flip(this: &VideoFrameBufferInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `flip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "flip")]
    pub fn set_flip(this: &VideoFrameBufferInit, val: bool);
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    #[wasm_bindgen(method, getter = "format")]
    pub fn get_format(this: &VideoFrameBufferInit) -> VideoPixelFormat;
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    #[wasm_bindgen(method, setter = "format")]
    pub fn set_format(this: &VideoFrameBufferInit, val: VideoPixelFormat);
    #[cfg(feature = "PlaneLayout")]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "layout")]
    pub fn get_layout(this: &VideoFrameBufferInit) -> Option<::js_sys::Array<PlaneLayout>>;
    #[cfg(feature = "PlaneLayout")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout(this: &VideoFrameBufferInit, val: &[PlaneLayout]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[doc = "Get the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "metadata")]
    pub fn get_metadata(this: &VideoFrameBufferInit) -> Option<VideoFrameMetadata>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[doc = "Change the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "metadata")]
    pub fn set_metadata(this: &VideoFrameBufferInit, val: &VideoFrameMetadata);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `rotation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "rotation")]
    pub fn get_rotation(this: &VideoFrameBufferInit) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `rotation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "rotation")]
    pub fn set_rotation(this: &VideoFrameBufferInit, val: f64);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &VideoFrameBufferInit) -> f64;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &VideoFrameBufferInit, val: i32);
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_f64(this: &VideoFrameBufferInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "transfer")]
    pub fn get_transfer(
        this: &VideoFrameBufferInit,
    ) -> Option<::js_sys::Array<::js_sys::ArrayBuffer>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "transfer")]
    pub fn set_transfer(this: &VideoFrameBufferInit, val: &[::js_sys::ArrayBuffer]);
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, getter = "visibleRect")]
    pub fn get_visible_rect(this: &VideoFrameBufferInit) -> Option<DomRectInit>;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameBufferInit`*"]
    #[wasm_bindgen(method, setter = "visibleRect")]
    pub fn set_visible_rect(this: &VideoFrameBufferInit, val: &DomRectInit);
}
impl VideoFrameBufferInit {
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Construct a new `VideoFrameBufferInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    pub fn new(
        coded_height: u32,
        coded_width: u32,
        format: VideoPixelFormat,
        timestamp: i32,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_coded_height(coded_height);
        ret.set_coded_width(coded_width);
        ret.set_format(format);
        ret.set_timestamp(timestamp);
        ret
    }
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Construct a new `VideoFrameBufferInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameBufferInit`, `VideoPixelFormat`*"]
    pub fn new_with_f64(
        coded_height: u32,
        coded_width: u32,
        format: VideoPixelFormat,
        timestamp: f64,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_coded_height(coded_height);
        ret.set_coded_width(coded_width);
        ret.set_format(format);
        ret.set_timestamp_f64(timestamp);
        ret
    }
    #[deprecated = "Use `set_coded_height()` instead."]
    pub fn coded_height(&mut self, val: u32) -> &mut Self {
        self.set_coded_height(val);
        self
    }
    #[deprecated = "Use `set_coded_width()` instead."]
    pub fn coded_width(&mut self, val: u32) -> &mut Self {
        self.set_coded_width(val);
        self
    }
    #[cfg(feature = "VideoColorSpaceInit")]
    #[deprecated = "Use `set_color_space()` instead."]
    pub fn color_space(&mut self, val: &VideoColorSpaceInit) -> &mut Self {
        self.set_color_space(val);
        self
    }
    #[deprecated = "Use `set_display_height()` instead."]
    pub fn display_height(&mut self, val: u32) -> &mut Self {
        self.set_display_height(val);
        self
    }
    #[deprecated = "Use `set_display_width()` instead."]
    pub fn display_width(&mut self, val: u32) -> &mut Self {
        self.set_display_width(val);
        self
    }
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: u32) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_flip()` instead."]
    pub fn flip(&mut self, val: bool) -> &mut Self {
        self.set_flip(val);
        self
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
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[deprecated = "Use `set_metadata()` instead."]
    pub fn metadata(&mut self, val: &VideoFrameMetadata) -> &mut Self {
        self.set_metadata(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_rotation()` instead."]
    pub fn rotation(&mut self, val: f64) -> &mut Self {
        self.set_rotation(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: i32) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_transfer()` instead."]
    pub fn transfer(&mut self, val: &[::js_sys::ArrayBuffer]) -> &mut Self {
        self.set_transfer(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[deprecated = "Use `set_visible_rect()` instead."]
    pub fn visible_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_visible_rect(val);
        self
    }
}
