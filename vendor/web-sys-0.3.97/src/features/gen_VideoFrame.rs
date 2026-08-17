#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VideoFrame",
        typescript_type = "VideoFrame"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub type VideoFrame;
    #[cfg(feature = "VideoPixelFormat")]
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "format")]
    #[doc = "Getter for the `format` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/format)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoPixelFormat`*"]
    pub fn format(this: &VideoFrame) -> Option<VideoPixelFormat>;
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "codedWidth")]
    #[doc = "Getter for the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn coded_width(this: &VideoFrame) -> u32;
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "codedHeight")]
    #[doc = "Getter for the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn coded_height(this: &VideoFrame) -> u32;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "codedRect")]
    #[doc = "Getter for the `codedRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `VideoFrame`*"]
    pub fn coded_rect(this: &VideoFrame) -> Option<DomRectReadOnly>;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "visibleRect")]
    #[doc = "Getter for the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/visibleRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `VideoFrame`*"]
    pub fn visible_rect(this: &VideoFrame) -> Option<DomRectReadOnly>;
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "displayWidth")]
    #[doc = "Getter for the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/displayWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn display_width(this: &VideoFrame) -> u32;
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "displayHeight")]
    #[doc = "Getter for the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/displayHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn display_height(this: &VideoFrame) -> u32;
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn duration(this: &VideoFrame) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn timestamp(this: &VideoFrame) -> f64;
    #[cfg(feature = "VideoColorSpace")]
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "colorSpace")]
    #[doc = "Getter for the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/colorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoFrame`*"]
    pub fn color_space(this: &VideoFrame) -> VideoColorSpace;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "rotation")]
    #[doc = "Getter for the `rotation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/rotation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rotation(this: &VideoFrame) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "VideoFrame", js_name = "flip")]
    #[doc = "Getter for the `flip` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/flip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flip(this: &VideoFrame) -> bool;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `VideoFrame`*"]
    pub fn new_with_html_image_element(image: &HtmlImageElement) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `VideoFrame`*"]
    pub fn new_with_svg_image_element(image: &SvgImageElement) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VideoFrame`*"]
    pub fn new_with_html_canvas_element(image: &HtmlCanvasElement) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoFrame`*"]
    pub fn new_with_html_video_element(image: &HtmlVideoElement) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `VideoFrame`*"]
    pub fn new_with_image_bitmap(image: &ImageBitmap) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `VideoFrame`*"]
    pub fn new_with_offscreen_canvas(image: &OffscreenCanvas) -> Result<VideoFrame, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn new_with_video_frame(image: &VideoFrame) -> Result<VideoFrame, JsValue>;
    #[cfg(all(feature = "HtmlImageElement", feature = "VideoFrameInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_html_image_element_and_video_frame_init(
        image: &HtmlImageElement,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(all(feature = "SvgImageElement", feature = "VideoFrameInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_svg_image_element_and_video_frame_init(
        image: &SvgImageElement,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(all(feature = "HtmlCanvasElement", feature = "VideoFrameInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_html_canvas_element_and_video_frame_init(
        image: &HtmlCanvasElement,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(all(feature = "HtmlVideoElement", feature = "VideoFrameInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_html_video_element_and_video_frame_init(
        image: &HtmlVideoElement,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(all(feature = "ImageBitmap", feature = "VideoFrameInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_image_bitmap_and_video_frame_init(
        image: &ImageBitmap,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(all(feature = "OffscreenCanvas", feature = "VideoFrameInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_offscreen_canvas_and_video_frame_init(
        image: &OffscreenCanvas,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "VideoFrameInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameInit`*"]
    pub fn new_with_video_frame_and_video_frame_init(
        image: &VideoFrame,
        init: &VideoFrameInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "VideoFrameBufferInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameBufferInit`*"]
    pub fn new_with_buffer_source_and_video_frame_buffer_init(
        data: &::js_sys::Object,
        init: &VideoFrameBufferInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "VideoFrameBufferInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameBufferInit`*"]
    pub fn new_with_u8_slice_and_video_frame_buffer_init(
        data: &mut [u8],
        init: &VideoFrameBufferInit,
    ) -> Result<VideoFrame, JsValue>;
    #[cfg(feature = "VideoFrameBufferInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoFrame")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameBufferInit`*"]
    pub fn new_with_u8_array_and_video_frame_buffer_init(
        data: &::js_sys::Uint8Array,
        init: &VideoFrameBufferInit,
    ) -> Result<VideoFrame, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "VideoFrame", js_name = "allocationSize")]
    #[doc = "The `allocationSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/allocationSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn allocation_size(this: &VideoFrame) -> Result<u32, JsValue>;
    #[cfg(feature = "VideoFrameCopyToOptions")]
    #[wasm_bindgen(catch, method, js_class = "VideoFrame", js_name = "allocationSize")]
    #[doc = "The `allocationSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/allocationSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameCopyToOptions`*"]
    pub fn allocation_size_with_options(
        this: &VideoFrame,
        options: &VideoFrameCopyToOptions,
    ) -> Result<u32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "VideoFrame")]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn clone(this: &VideoFrame) -> Result<VideoFrame, JsValue>;
    #[wasm_bindgen(method, js_class = "VideoFrame")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    pub fn close(this: &VideoFrame);
    #[cfg(feature = "PlaneLayout")]
    #[wasm_bindgen(method, js_class = "VideoFrame", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrame`*"]
    pub fn copy_to_with_buffer_source(
        this: &VideoFrame,
        destination: &::js_sys::Object,
    ) -> ::js_sys::Promise<::js_sys::Array<PlaneLayout>>;
    #[cfg(feature = "PlaneLayout")]
    #[wasm_bindgen(method, js_class = "VideoFrame", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrame`*"]
    pub fn copy_to_with_u8_slice(
        this: &VideoFrame,
        destination: &mut [u8],
    ) -> ::js_sys::Promise<::js_sys::Array<PlaneLayout>>;
    #[cfg(feature = "PlaneLayout")]
    #[wasm_bindgen(method, js_class = "VideoFrame", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrame`*"]
    pub fn copy_to_with_u8_array(
        this: &VideoFrame,
        destination: &::js_sys::Uint8Array,
    ) -> ::js_sys::Promise<::js_sys::Array<PlaneLayout>>;
    #[cfg(all(feature = "PlaneLayout", feature = "VideoFrameCopyToOptions",))]
    #[wasm_bindgen(method, js_class = "VideoFrame", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrame`, `VideoFrameCopyToOptions`*"]
    pub fn copy_to_with_buffer_source_and_options(
        this: &VideoFrame,
        destination: &::js_sys::Object,
        options: &VideoFrameCopyToOptions,
    ) -> ::js_sys::Promise<::js_sys::Array<PlaneLayout>>;
    #[cfg(all(feature = "PlaneLayout", feature = "VideoFrameCopyToOptions",))]
    #[wasm_bindgen(method, js_class = "VideoFrame", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrame`, `VideoFrameCopyToOptions`*"]
    pub fn copy_to_with_u8_slice_and_options(
        this: &VideoFrame,
        destination: &mut [u8],
        options: &VideoFrameCopyToOptions,
    ) -> ::js_sys::Promise<::js_sys::Array<PlaneLayout>>;
    #[cfg(all(feature = "PlaneLayout", feature = "VideoFrameCopyToOptions",))]
    #[wasm_bindgen(method, js_class = "VideoFrame", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`, `VideoFrame`, `VideoFrameCopyToOptions`*"]
    pub fn copy_to_with_u8_array_and_options(
        this: &VideoFrame,
        destination: &::js_sys::Uint8Array,
        options: &VideoFrameCopyToOptions,
    ) -> ::js_sys::Promise<::js_sys::Array<PlaneLayout>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[wasm_bindgen(method, js_class = "VideoFrame")]
    #[doc = "The `metadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/metadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn metadata(this: &VideoFrame) -> VideoFrameMetadata;
}
