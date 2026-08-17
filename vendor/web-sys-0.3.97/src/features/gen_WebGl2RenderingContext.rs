#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebGL2RenderingContext",
        typescript_type = "WebGL2RenderingContext"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGl2RenderingContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub type WebGl2RenderingContext;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGL2RenderingContext",
        js_name = "canvas"
    )]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn canvas(this: &WebGl2RenderingContext) -> Option<::js_sys::Object>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGL2RenderingContext",
        js_name = "drawingBufferWidth"
    )]
    #[doc = "Getter for the `drawingBufferWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawingBufferWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn drawing_buffer_width(this: &WebGl2RenderingContext) -> i32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGL2RenderingContext",
        js_name = "drawingBufferHeight"
    )]
    #[doc = "Getter for the `drawingBufferHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawingBufferHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn drawing_buffer_height(this: &WebGl2RenderingContext) -> i32;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "beginQuery")]
    #[doc = "The `beginQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/beginQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*"]
    pub fn begin_query(this: &WebGl2RenderingContext, target: u32, query: &WebGlQuery);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "beginTransformFeedback"
    )]
    #[doc = "The `beginTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/beginTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn begin_transform_feedback(this: &WebGl2RenderingContext, primitive_mode: u32);
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindBufferBase"
    )]
    #[doc = "The `bindBufferBase()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferBase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn bind_buffer_base(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
    );
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindBufferRange"
    )]
    #[doc = "The `bindBufferRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn bind_buffer_range_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: i32,
        size: i32,
    );
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindBufferRange"
    )]
    #[doc = "The `bindBufferRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn bind_buffer_range_with_f64_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: f64,
        size: i32,
    );
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindBufferRange"
    )]
    #[doc = "The `bindBufferRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn bind_buffer_range_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: i32,
        size: f64,
    );
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindBufferRange"
    )]
    #[doc = "The `bindBufferRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBufferRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn bind_buffer_range_with_f64_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
        buffer: Option<&WebGlBuffer>,
        offset: f64,
        size: f64,
    );
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bindSampler")]
    #[doc = "The `bindSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn bind_sampler(this: &WebGl2RenderingContext, unit: u32, sampler: Option<&WebGlSampler>);
    #[cfg(feature = "WebGlTransformFeedback")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindTransformFeedback"
    )]
    #[doc = "The `bindTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*"]
    pub fn bind_transform_feedback(
        this: &WebGl2RenderingContext,
        target: u32,
        tf: Option<&WebGlTransformFeedback>,
    );
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindVertexArray"
    )]
    #[doc = "The `bindVertexArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindVertexArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*"]
    pub fn bind_vertex_array(this: &WebGl2RenderingContext, array: Option<&WebGlVertexArrayObject>);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "blitFramebuffer"
    )]
    #[doc = "The `blitFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blitFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn blit_framebuffer(
        this: &WebGl2RenderingContext,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_i32(this: &WebGl2RenderingContext, target: u32, size: i32, usage: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_f64(this: &WebGl2RenderingContext, target: u32, size: f64, usage: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_opt_array_buffer(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: Option<&::js_sys::ArrayBuffer>,
        usage: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Object,
        usage: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &[u8],
        usage: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Uint8Array,
        usage: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Object,
        usage: u32,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &[u8],
        usage: u32,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Uint8Array,
        usage: u32,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_array_buffer_view_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Object,
        usage: u32,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &[u8],
        usage: u32,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferData")]
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_data_with_js_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_data: &::js_sys::Uint8Array,
        usage: u32,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_array_buffer(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &::js_sys::ArrayBuffer,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_array_buffer(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &::js_sys::ArrayBuffer,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &::js_sys::Object,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &::js_sys::Object,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &[u8],
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &[u8],
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: i32,
        src_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        offset: f64,
        src_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &[u8],
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &[u8],
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_array_buffer_view_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &::js_sys::Object,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &[u8],
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &[u8],
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_i32_and_js_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: i32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bufferSubData")]
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn buffer_sub_data_with_f64_and_js_u8_array_and_src_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        dst_byte_offset: f64,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfi")]
    #[doc = "The `clearBufferfi()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfi)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfi(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        depth: f32,
        stencil: i32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfv")]
    #[doc = "The `clearBufferfv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfv_with_f32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[f32],
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfv")]
    #[doc = "The `clearBufferfv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfv")]
    #[doc = "The `clearBufferfv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfv")]
    #[doc = "The `clearBufferfv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[f32],
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfv")]
    #[doc = "The `clearBufferfv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferfv")]
    #[doc = "The `clearBufferfv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferfv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferfv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferiv")]
    #[doc = "The `clearBufferiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferiv_with_i32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[i32],
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferiv")]
    #[doc = "The `clearBufferiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferiv_with_js_i32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::js_sys::Int32Array,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferiv")]
    #[doc = "The `clearBufferiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferiv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferiv")]
    #[doc = "The `clearBufferiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferiv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[i32],
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferiv")]
    #[doc = "The `clearBufferiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferiv_with_js_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::js_sys::Int32Array,
        src_offset: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearBufferiv")]
    #[doc = "The `clearBufferiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferiv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clearBufferuiv"
    )]
    #[doc = "The `clearBufferuiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferuiv_with_u32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[u32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clearBufferuiv"
    )]
    #[doc = "The `clearBufferuiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferuiv_with_js_u32_array(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::js_sys::Uint32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clearBufferuiv"
    )]
    #[doc = "The `clearBufferuiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferuiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clearBufferuiv"
    )]
    #[doc = "The `clearBufferuiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferuiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &[u32],
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clearBufferuiv"
    )]
    #[doc = "The `clearBufferuiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferuiv_with_js_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::js_sys::Uint32Array,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clearBufferuiv"
    )]
    #[doc = "The `clearBufferuiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearBufferuiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_bufferuiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        buffer: u32,
        drawbuffer: i32,
        values: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clientWaitSync"
    )]
    #[doc = "The `clientWaitSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clientWaitSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn client_wait_sync_with_u32(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: u32,
    ) -> u32;
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "clientWaitSync"
    )]
    #[doc = "The `clientWaitSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clientWaitSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn client_wait_sync_with_f64(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: f64,
    ) -> u32;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        offset: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Object,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &[u8],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_js_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage2D"
    )]
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_2d_with_js_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        image_size: i32,
        offset: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Object,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &[u8],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_js_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &[u8],
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexImage3D"
    )]
    #[doc = "The `compressedTexImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_image_3d_with_js_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        image_size: i32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        image_size: i32,
        offset: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Object,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &mut [u8],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_js_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage2D"
    )]
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_2d_with_js_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_i32_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        image_size: i32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_i32_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        image_size: i32,
        offset: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Object,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &mut [u8],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_array_buffer_view_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_js_u8_array_and_u32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_array_buffer_view_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &mut [u8],
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "compressedTexSubImage3D"
    )]
    #[doc = "The `compressedTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compressedTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn compressed_tex_sub_image_3d_with_js_u8_array_and_u32_and_src_length_override(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
        src_length_override: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_i32_and_i32_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: i32,
        size: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_f64_and_i32_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: i32,
        size: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_i32_and_f64_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: f64,
        size: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_f64_and_f64_and_i32(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: f64,
        size: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_i32_and_i32_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: i32,
        size: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_f64_and_i32_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: i32,
        size: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_i32_and_f64_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: i32,
        write_offset: f64,
        size: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyBufferSubData"
    )]
    #[doc = "The `copyBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_buffer_sub_data_with_f64_and_f64_and_f64(
        this: &WebGl2RenderingContext,
        read_target: u32,
        write_target: u32,
        read_offset: f64,
        write_offset: f64,
        size: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyTexSubImage3D"
    )]
    #[doc = "The `copyTexSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyTexSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_tex_sub_image_3d(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "createQuery")]
    #[doc = "The `createQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*"]
    pub fn create_query(this: &WebGl2RenderingContext) -> Option<WebGlQuery>;
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "createSampler")]
    #[doc = "The `createSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn create_sampler(this: &WebGl2RenderingContext) -> Option<WebGlSampler>;
    #[cfg(feature = "WebGlTransformFeedback")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "createTransformFeedback"
    )]
    #[doc = "The `createTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*"]
    pub fn create_transform_feedback(
        this: &WebGl2RenderingContext,
    ) -> Option<WebGlTransformFeedback>;
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "createVertexArray"
    )]
    #[doc = "The `createVertexArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createVertexArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*"]
    pub fn create_vertex_array(this: &WebGl2RenderingContext) -> Option<WebGlVertexArrayObject>;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteQuery")]
    #[doc = "The `deleteQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*"]
    pub fn delete_query(this: &WebGl2RenderingContext, query: Option<&WebGlQuery>);
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteSampler")]
    #[doc = "The `deleteSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn delete_sampler(this: &WebGl2RenderingContext, sampler: Option<&WebGlSampler>);
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteSync")]
    #[doc = "The `deleteSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn delete_sync(this: &WebGl2RenderingContext, sync: Option<&WebGlSync>);
    #[cfg(feature = "WebGlTransformFeedback")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "deleteTransformFeedback"
    )]
    #[doc = "The `deleteTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*"]
    pub fn delete_transform_feedback(
        this: &WebGl2RenderingContext,
        tf: Option<&WebGlTransformFeedback>,
    );
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "deleteVertexArray"
    )]
    #[doc = "The `deleteVertexArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteVertexArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*"]
    pub fn delete_vertex_array(
        this: &WebGl2RenderingContext,
        vertex_array: Option<&WebGlVertexArrayObject>,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "drawArraysInstanced"
    )]
    #[doc = "The `drawArraysInstanced()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawArraysInstanced)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_arrays_instanced(
        this: &WebGl2RenderingContext,
        mode: u32,
        first: i32,
        count: i32,
        instance_count: i32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "drawBuffers")]
    #[doc = "The `drawBuffers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_buffers(this: &WebGl2RenderingContext, buffers: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "drawElementsInstanced"
    )]
    #[doc = "The `drawElementsInstanced()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElementsInstanced)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_elements_instanced_with_i32(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
        instance_count: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "drawElementsInstanced"
    )]
    #[doc = "The `drawElementsInstanced()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElementsInstanced)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_elements_instanced_with_f64(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
        instance_count: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "drawRangeElements"
    )]
    #[doc = "The `drawRangeElements()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawRangeElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_range_elements_with_i32(
        this: &WebGl2RenderingContext,
        mode: u32,
        start: u32,
        end: u32,
        count: i32,
        type_: u32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "drawRangeElements"
    )]
    #[doc = "The `drawRangeElements()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawRangeElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_range_elements_with_f64(
        this: &WebGl2RenderingContext,
        mode: u32,
        start: u32,
        end: u32,
        count: i32,
        type_: u32,
        offset: f64,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "endQuery")]
    #[doc = "The `endQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/endQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn end_query(this: &WebGl2RenderingContext, target: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "endTransformFeedback"
    )]
    #[doc = "The `endTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/endTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn end_transform_feedback(this: &WebGl2RenderingContext);
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "fenceSync")]
    #[doc = "The `fenceSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/fenceSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn fence_sync(
        this: &WebGl2RenderingContext,
        condition: u32,
        flags: u32,
    ) -> Option<WebGlSync>;
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "framebufferTextureLayer"
    )]
    #[doc = "The `framebufferTextureLayer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/framebufferTextureLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*"]
    pub fn framebuffer_texture_layer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
        layer: i32,
    );
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getActiveUniformBlockName"
    )]
    #[doc = "The `getActiveUniformBlockName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniformBlockName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_active_uniform_block_name(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_index: u32,
    ) -> Option<::alloc::string::String>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getActiveUniformBlockParameter"
    )]
    #[doc = "The `getActiveUniformBlockParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniformBlockParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_active_uniform_block_parameter(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getActiveUniforms"
    )]
    #[doc = "The `getActiveUniforms()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniforms)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_active_uniforms(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_indices: &::wasm_bindgen::JsValue,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Object,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Object,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &mut [u8],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &mut [u8],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Uint8Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_array_buffer_view_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_array_buffer_view_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &mut [u8],
        dst_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &mut [u8],
        dst_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_js_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Uint8Array,
        dst_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_js_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Uint8Array,
        dst_offset: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_array_buffer_view_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_array_buffer_view_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &mut [u8],
        dst_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_u8_array_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &mut [u8],
        dst_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_i32_and_js_u8_array_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: i32,
        dst_data: &::js_sys::Uint8Array,
        dst_offset: u32,
        length: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferSubData"
    )]
    #[doc = "The `getBufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_sub_data_with_f64_and_js_u8_array_and_dst_offset_and_length(
        this: &WebGl2RenderingContext,
        target: u32,
        src_byte_offset: f64,
        dst_data: &::js_sys::Uint8Array,
        dst_offset: u32,
        length: u32,
    );
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getFragDataLocation"
    )]
    #[doc = "The `getFragDataLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getFragDataLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_frag_data_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> i32;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getIndexedParameter"
    )]
    #[doc = "The `getIndexedParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getIndexedParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_indexed_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        index: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getInternalformatParameter"
    )]
    #[doc = "The `getInternalformatParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getInternalformatParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_internalformat_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        internalformat: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "getQuery")]
    #[doc = "The `getQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_query(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getQueryParameter"
    )]
    #[doc = "The `getQueryParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getQueryParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*"]
    pub fn get_query_parameter(
        this: &WebGl2RenderingContext,
        query: &WebGlQuery,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getSamplerParameter"
    )]
    #[doc = "The `getSamplerParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getSamplerParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn get_sampler_parameter(
        this: &WebGl2RenderingContext,
        sampler: &WebGlSampler,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getSyncParameter"
    )]
    #[doc = "The `getSyncParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getSyncParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn get_sync_parameter(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getTransformFeedbackVarying"
    )]
    #[doc = "The `getTransformFeedbackVarying()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getTransformFeedbackVarying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlActiveInfo`, `WebGlProgram`*"]
    pub fn get_transform_feedback_varying(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getUniformBlockIndex"
    )]
    #[doc = "The `getUniformBlockIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniformBlockIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_uniform_block_index(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_name: &str,
    ) -> u32;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getUniformIndices"
    )]
    #[doc = "The `getUniformIndices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniformIndices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_uniform_indices(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_names: &::wasm_bindgen::JsValue,
    ) -> Option<::js_sys::Array>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "invalidateFramebuffer"
    )]
    #[doc = "The `invalidateFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/invalidateFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn invalidate_framebuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "invalidateSubFramebuffer"
    )]
    #[doc = "The `invalidateSubFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/invalidateSubFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn invalidate_sub_framebuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachments: &::wasm_bindgen::JsValue,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isQuery")]
    #[doc = "The `isQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlQuery`*"]
    pub fn is_query(this: &WebGl2RenderingContext, query: Option<&WebGlQuery>) -> bool;
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isSampler")]
    #[doc = "The `isSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn is_sampler(this: &WebGl2RenderingContext, sampler: Option<&WebGlSampler>) -> bool;
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isSync")]
    #[doc = "The `isSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn is_sync(this: &WebGl2RenderingContext, sync: Option<&WebGlSync>) -> bool;
    #[cfg(feature = "WebGlTransformFeedback")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "isTransformFeedback"
    )]
    #[doc = "The `isTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTransformFeedback`*"]
    pub fn is_transform_feedback(
        this: &WebGl2RenderingContext,
        tf: Option<&WebGlTransformFeedback>,
    ) -> bool;
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isVertexArray")]
    #[doc = "The `isVertexArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isVertexArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlVertexArrayObject`*"]
    pub fn is_vertex_array(
        this: &WebGl2RenderingContext,
        vertex_array: Option<&WebGlVertexArrayObject>,
    ) -> bool;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "pauseTransformFeedback"
    )]
    #[doc = "The `pauseTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/pauseTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn pause_transform_feedback(this: &WebGl2RenderingContext);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "readBuffer")]
    #[doc = "The `readBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_buffer(this: &WebGl2RenderingContext, src: u32);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_opt_u8_array(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: Option<&mut [u8]>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_opt_js_u8_array(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: Option<&::js_sys::Uint8Array>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_i32(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        offset: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_f64(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        offset: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_array_buffer_view_and_dst_offset(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: &::js_sys::Object,
        dst_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: &mut [u8],
        dst_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "readPixels"
    )]
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn read_pixels_with_js_u8_array_and_dst_offset(
        this: &WebGl2RenderingContext,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        dst_data: &::js_sys::Uint8Array,
        dst_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "renderbufferStorageMultisample"
    )]
    #[doc = "The `renderbufferStorageMultisample()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/renderbufferStorageMultisample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn renderbuffer_storage_multisample(
        this: &WebGl2RenderingContext,
        target: u32,
        samples: i32,
        internalformat: u32,
        width: i32,
        height: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "resumeTransformFeedback"
    )]
    #[doc = "The `resumeTransformFeedback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/resumeTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn resume_transform_feedback(this: &WebGl2RenderingContext);
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "samplerParameterf"
    )]
    #[doc = "The `samplerParameterf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/samplerParameterf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn sampler_parameterf(
        this: &WebGl2RenderingContext,
        sampler: &WebGlSampler,
        pname: u32,
        param: f32,
    );
    #[cfg(feature = "WebGlSampler")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "samplerParameteri"
    )]
    #[doc = "The `samplerParameteri()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/samplerParameteri)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSampler`*"]
    pub fn sampler_parameteri(
        this: &WebGl2RenderingContext,
        sampler: &WebGlSampler,
        pname: u32,
        param: i32,
    );
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Uint8Array>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_video_frame(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &VideoFrame,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_u32_and_u32_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_video_frame(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &VideoFrame,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &[u8],
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage2D"
    )]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_video_frame(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &VideoFrame,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_opt_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: Option<&[u8]>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_opt_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Uint8Array>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &[u8],
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texImage3D"
    )]
    #[doc = "The `texImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_image_3d_with_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "texStorage2D")]
    #[doc = "The `texStorage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texStorage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_storage_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        levels: i32,
        internalformat: u32,
        width: i32,
        height: i32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "texStorage3D")]
    #[doc = "The `texStorage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texStorage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_storage_3d(
        this: &WebGl2RenderingContext,
        target: u32,
        levels: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        depth: i32,
    );
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Uint8Array>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_video_frame(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &VideoFrame,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_video_frame(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &VideoFrame,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Object,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        src_data: &[u8],
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage2D"
    )]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        src_data: &::js_sys::Uint8Array,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_i32(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        pbo_offset: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_f64(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        pbo_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_html_canvas_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &HtmlCanvasElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_html_image_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &HtmlImageElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_html_video_element(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &HtmlVideoElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_video_frame(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &VideoFrame,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_image_bitmap(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &ImageBitmap,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_image_data(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        source: &ImageData,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_opt_array_buffer_view(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_opt_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&[u8]>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_opt_js_u8_array(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Uint8Array>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_opt_array_buffer_view_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Object>,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_opt_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&[u8]>,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "texSubImage3D"
    )]
    #[doc = "The `texSubImage3D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texSubImage3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_sub_image_3d_with_opt_js_u8_array_and_src_offset(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        format: u32,
        type_: u32,
        src_data: Option<&::js_sys::Uint8Array>,
        src_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "transformFeedbackVaryings"
    )]
    #[doc = "The `transformFeedbackVaryings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/transformFeedbackVaryings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn transform_feedback_varyings(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        varyings: &::wasm_bindgen::JsValue,
        buffer_mode: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1fv")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_js_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_js_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_js_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1iv")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform1ui()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1ui)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_js_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_js_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_js_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform1uiv")]
    #[doc = "The `uniform1uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2fv")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_js_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_js_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_js_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2iv")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform2ui()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2ui)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
        v1: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_js_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_js_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_js_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform2uiv")]
    #[doc = "The `uniform2uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3fv")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_js_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_js_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_js_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3iv")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform3ui()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3ui)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
        v1: u32,
        v2: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_js_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_js_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_js_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform3uiv")]
    #[doc = "The `uniform3uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4fv")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_js_i32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_js_i32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[i32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_js_i32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Int32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4iv")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4iv_with_i32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform4ui()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4ui)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4ui(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_js_u32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_js_u32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_u32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &[u32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_js_u32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::js_sys::Uint32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "uniform4uiv")]
    #[doc = "The `uniform4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4uiv_with_u32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformBlockBinding"
    )]
    #[doc = "The `uniformBlockBinding()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformBlockBinding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn uniform_block_binding(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        uniform_block_index: u32,
        uniform_block_binding: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2fv"
    )]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x3fv"
    )]
    #[doc = "The `uniformMatrix2x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix2x4fv"
    )]
    #[doc = "The `uniformMatrix2x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix2x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix2x4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3fv"
    )]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x2fv"
    )]
    #[doc = "The `uniformMatrix3x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix3x4fv"
    )]
    #[doc = "The `uniformMatrix3x4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix3x4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix3x4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4fv"
    )]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x2fv"
    )]
    #[doc = "The `uniformMatrix4x2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x2fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_js_f32_array_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_f32_sequence_and_src_offset(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_js_f32_array_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::js_sys::Float32Array,
        src_offset: u32,
        src_length: u32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "uniformMatrix4x3fv"
    )]
    #[doc = "The `uniformMatrix4x3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix4x3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform_matrix4x3fv_with_f32_sequence_and_src_offset_and_src_length(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
        src_offset: u32,
        src_length: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribDivisor"
    )]
    #[doc = "The `vertexAttribDivisor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribDivisor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_divisor(this: &WebGl2RenderingContext, index: u32, divisor: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4i"
    )]
    #[doc = "The `vertexAttribI4i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4i(
        this: &WebGl2RenderingContext,
        index: u32,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4iv"
    )]
    #[doc = "The `vertexAttribI4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4iv_with_i32_array(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &mut [i32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4iv"
    )]
    #[doc = "The `vertexAttribI4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4iv_with_js_i32_array(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &::js_sys::Int32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4iv"
    )]
    #[doc = "The `vertexAttribI4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4iv_with_i32_sequence(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4ui"
    )]
    #[doc = "The `vertexAttribI4ui()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4ui)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4ui(
        this: &WebGl2RenderingContext,
        index: u32,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4uiv"
    )]
    #[doc = "The `vertexAttribI4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4uiv_with_u32_array(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &mut [u32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4uiv"
    )]
    #[doc = "The `vertexAttribI4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4uiv_with_js_u32_array(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &::js_sys::Uint32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribI4uiv"
    )]
    #[doc = "The `vertexAttribI4uiv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribI4uiv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i4uiv_with_u32_sequence(
        this: &WebGl2RenderingContext,
        index: u32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribIPointer"
    )]
    #[doc = "The `vertexAttribIPointer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribIPointer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i_pointer_with_i32(
        this: &WebGl2RenderingContext,
        index: u32,
        size: i32,
        type_: u32,
        stride: i32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribIPointer"
    )]
    #[doc = "The `vertexAttribIPointer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribIPointer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_i_pointer_with_f64(
        this: &WebGl2RenderingContext,
        index: u32,
        size: i32,
        type_: u32,
        stride: i32,
        offset: f64,
    );
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "waitSync")]
    #[doc = "The `waitSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/waitSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn wait_sync_with_i32(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: i32,
    );
    #[cfg(feature = "WebGlSync")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "waitSync")]
    #[doc = "The `waitSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/waitSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlSync`*"]
    pub fn wait_sync_with_f64(
        this: &WebGl2RenderingContext,
        sync: &WebGlSync,
        flags: u32,
        timeout: f64,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "activeTexture")]
    #[doc = "The `activeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/activeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn active_texture(this: &WebGl2RenderingContext, texture: u32);
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "attachShader")]
    #[doc = "The `attachShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/attachShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlShader`*"]
    pub fn attach_shader(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindAttribLocation"
    )]
    #[doc = "The `bindAttribLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindAttribLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn bind_attrib_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
        name: &str,
    );
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bindBuffer")]
    #[doc = "The `bindBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn bind_buffer(this: &WebGl2RenderingContext, target: u32, buffer: Option<&WebGlBuffer>);
    #[cfg(feature = "WebGlFramebuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindFramebuffer"
    )]
    #[doc = "The `bindFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*"]
    pub fn bind_framebuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        framebuffer: Option<&WebGlFramebuffer>,
    );
    #[cfg(feature = "WebGlRenderbuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "bindRenderbuffer"
    )]
    #[doc = "The `bindRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*"]
    pub fn bind_renderbuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "bindTexture")]
    #[doc = "The `bindTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/bindTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*"]
    pub fn bind_texture(this: &WebGl2RenderingContext, target: u32, texture: Option<&WebGlTexture>);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "blendColor")]
    #[doc = "The `blendColor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn blend_color(this: &WebGl2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "blendEquation")]
    #[doc = "The `blendEquation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendEquation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn blend_equation(this: &WebGl2RenderingContext, mode: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "blendEquationSeparate"
    )]
    #[doc = "The `blendEquationSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendEquationSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn blend_equation_separate(this: &WebGl2RenderingContext, mode_rgb: u32, mode_alpha: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "blendFunc")]
    #[doc = "The `blendFunc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendFunc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn blend_func(this: &WebGl2RenderingContext, sfactor: u32, dfactor: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "blendFuncSeparate"
    )]
    #[doc = "The `blendFuncSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/blendFuncSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn blend_func_separate(
        this: &WebGl2RenderingContext,
        src_rgb: u32,
        dst_rgb: u32,
        src_alpha: u32,
        dst_alpha: u32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "checkFramebufferStatus"
    )]
    #[doc = "The `checkFramebufferStatus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/checkFramebufferStatus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn check_framebuffer_status(this: &WebGl2RenderingContext, target: u32) -> u32;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear(this: &WebGl2RenderingContext, mask: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearColor")]
    #[doc = "The `clearColor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_color(this: &WebGl2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearDepth")]
    #[doc = "The `clearDepth()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearDepth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_depth(this: &WebGl2RenderingContext, depth: f32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "clearStencil")]
    #[doc = "The `clearStencil()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/clearStencil)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn clear_stencil(this: &WebGl2RenderingContext, s: i32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "colorMask")]
    #[doc = "The `colorMask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/colorMask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn color_mask(
        this: &WebGl2RenderingContext,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "compileShader")]
    #[doc = "The `compileShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/compileShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn compile_shader(this: &WebGl2RenderingContext, shader: &WebGlShader);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyTexImage2D"
    )]
    #[doc = "The `copyTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_tex_image_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        internalformat: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "copyTexSubImage2D"
    )]
    #[doc = "The `copyTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/copyTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn copy_tex_sub_image_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    );
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "createBuffer")]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn create_buffer(this: &WebGl2RenderingContext) -> Option<WebGlBuffer>;
    #[cfg(feature = "WebGlFramebuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "createFramebuffer"
    )]
    #[doc = "The `createFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*"]
    pub fn create_framebuffer(this: &WebGl2RenderingContext) -> Option<WebGlFramebuffer>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "createProgram")]
    #[doc = "The `createProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn create_program(this: &WebGl2RenderingContext) -> Option<WebGlProgram>;
    #[cfg(feature = "WebGlRenderbuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "createRenderbuffer"
    )]
    #[doc = "The `createRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*"]
    pub fn create_renderbuffer(this: &WebGl2RenderingContext) -> Option<WebGlRenderbuffer>;
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "createShader")]
    #[doc = "The `createShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn create_shader(this: &WebGl2RenderingContext, type_: u32) -> Option<WebGlShader>;
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "createTexture")]
    #[doc = "The `createTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/createTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*"]
    pub fn create_texture(this: &WebGl2RenderingContext) -> Option<WebGlTexture>;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "cullFace")]
    #[doc = "The `cullFace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/cullFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn cull_face(this: &WebGl2RenderingContext, mode: u32);
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteBuffer")]
    #[doc = "The `deleteBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn delete_buffer(this: &WebGl2RenderingContext, buffer: Option<&WebGlBuffer>);
    #[cfg(feature = "WebGlFramebuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "deleteFramebuffer"
    )]
    #[doc = "The `deleteFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*"]
    pub fn delete_framebuffer(
        this: &WebGl2RenderingContext,
        framebuffer: Option<&WebGlFramebuffer>,
    );
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteProgram")]
    #[doc = "The `deleteProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn delete_program(this: &WebGl2RenderingContext, program: Option<&WebGlProgram>);
    #[cfg(feature = "WebGlRenderbuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "deleteRenderbuffer"
    )]
    #[doc = "The `deleteRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*"]
    pub fn delete_renderbuffer(
        this: &WebGl2RenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteShader")]
    #[doc = "The `deleteShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn delete_shader(this: &WebGl2RenderingContext, shader: Option<&WebGlShader>);
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "deleteTexture")]
    #[doc = "The `deleteTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/deleteTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*"]
    pub fn delete_texture(this: &WebGl2RenderingContext, texture: Option<&WebGlTexture>);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "depthFunc")]
    #[doc = "The `depthFunc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/depthFunc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn depth_func(this: &WebGl2RenderingContext, func: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "depthMask")]
    #[doc = "The `depthMask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/depthMask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn depth_mask(this: &WebGl2RenderingContext, flag: bool);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "depthRange")]
    #[doc = "The `depthRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/depthRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn depth_range(this: &WebGl2RenderingContext, z_near: f32, z_far: f32);
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "detachShader")]
    #[doc = "The `detachShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/detachShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlShader`*"]
    pub fn detach_shader(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        shader: &WebGlShader,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `disable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/disable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn disable(this: &WebGl2RenderingContext, cap: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "disableVertexAttribArray"
    )]
    #[doc = "The `disableVertexAttribArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/disableVertexAttribArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn disable_vertex_attrib_array(this: &WebGl2RenderingContext, index: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "drawArrays")]
    #[doc = "The `drawArrays()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawArrays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_arrays(this: &WebGl2RenderingContext, mode: u32, first: i32, count: i32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "drawElements")]
    #[doc = "The `drawElements()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_elements_with_i32(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "drawElements")]
    #[doc = "The `drawElements()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/drawElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn draw_elements_with_f64(
        this: &WebGl2RenderingContext,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `enable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/enable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn enable(this: &WebGl2RenderingContext, cap: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "enableVertexAttribArray"
    )]
    #[doc = "The `enableVertexAttribArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/enableVertexAttribArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn enable_vertex_attrib_array(this: &WebGl2RenderingContext, index: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn finish(this: &WebGl2RenderingContext);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn flush(this: &WebGl2RenderingContext);
    #[cfg(feature = "WebGlRenderbuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "framebufferRenderbuffer"
    )]
    #[doc = "The `framebufferRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/framebufferRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*"]
    pub fn framebuffer_renderbuffer(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        renderbuffertarget: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    );
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "framebufferTexture2D"
    )]
    #[doc = "The `framebufferTexture2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/framebufferTexture2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*"]
    pub fn framebuffer_texture_2d(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "frontFace")]
    #[doc = "The `frontFace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/frontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn front_face(this: &WebGl2RenderingContext, mode: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "generateMipmap"
    )]
    #[doc = "The `generateMipmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/generateMipmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn generate_mipmap(this: &WebGl2RenderingContext, target: u32);
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getActiveAttrib"
    )]
    #[doc = "The `getActiveAttrib()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveAttrib)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlActiveInfo`, `WebGlProgram`*"]
    pub fn get_active_attrib(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getActiveUniform"
    )]
    #[doc = "The `getActiveUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getActiveUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlActiveInfo`, `WebGlProgram`*"]
    pub fn get_active_uniform(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getAttachedShaders"
    )]
    #[doc = "The `getAttachedShaders()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getAttachedShaders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_attached_shaders(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> Option<::js_sys::Array>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getAttribLocation"
    )]
    #[doc = "The `getAttribLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getAttribLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_attrib_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> i32;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getBufferParameter"
    )]
    #[doc = "The `getBufferParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getBufferParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_buffer_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlContextAttributes")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getContextAttributes"
    )]
    #[doc = "The `getContextAttributes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getContextAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlContextAttributes`*"]
    pub fn get_context_attributes(this: &WebGl2RenderingContext) -> Option<WebGlContextAttributes>;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "getError")]
    #[doc = "The `getError()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_error(this: &WebGl2RenderingContext) -> u32;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getExtension"
    )]
    #[doc = "The `getExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_extension(
        this: &WebGl2RenderingContext,
        name: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getFramebufferAttachmentParameter"
    )]
    #[doc = "The `getFramebufferAttachmentParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getFramebufferAttachmentParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_framebuffer_attachment_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        attachment: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getParameter"
    )]
    #[doc = "The `getParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_parameter(
        this: &WebGl2RenderingContext,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getProgramInfoLog"
    )]
    #[doc = "The `getProgramInfoLog()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getProgramInfoLog)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_program_info_log(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> Option<::alloc::string::String>;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getProgramParameter"
    )]
    #[doc = "The `getProgramParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getProgramParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn get_program_parameter(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getRenderbufferParameter"
    )]
    #[doc = "The `getRenderbufferParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getRenderbufferParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_renderbuffer_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getShaderInfoLog"
    )]
    #[doc = "The `getShaderInfoLog()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderInfoLog)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn get_shader_info_log(
        this: &WebGl2RenderingContext,
        shader: &WebGlShader,
    ) -> Option<::alloc::string::String>;
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getShaderParameter"
    )]
    #[doc = "The `getShaderParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn get_shader_parameter(
        this: &WebGl2RenderingContext,
        shader: &WebGlShader,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlShaderPrecisionFormat")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getShaderPrecisionFormat"
    )]
    #[doc = "The `getShaderPrecisionFormat()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderPrecisionFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShaderPrecisionFormat`*"]
    pub fn get_shader_precision_format(
        this: &WebGl2RenderingContext,
        shadertype: u32,
        precisiontype: u32,
    ) -> Option<WebGlShaderPrecisionFormat>;
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getShaderSource"
    )]
    #[doc = "The `getShaderSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getShaderSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn get_shader_source(
        this: &WebGl2RenderingContext,
        shader: &WebGlShader,
    ) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getSupportedExtensions"
    )]
    #[doc = "The `getSupportedExtensions()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getSupportedExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_supported_extensions(this: &WebGl2RenderingContext) -> Option<::js_sys::Array>;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getTexParameter"
    )]
    #[doc = "The `getTexParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getTexParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_tex_parameter(
        this: &WebGl2RenderingContext,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "getUniform")]
    #[doc = "The `getUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlUniformLocation`*"]
    pub fn get_uniform(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        location: &WebGlUniformLocation,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getUniformLocation"
    )]
    #[doc = "The `getUniformLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getUniformLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`, `WebGlUniformLocation`*"]
    pub fn get_uniform_location(
        this: &WebGl2RenderingContext,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getVertexAttrib"
    )]
    #[doc = "The `getVertexAttrib()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getVertexAttrib)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_vertex_attrib(
        this: &WebGl2RenderingContext,
        index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "getVertexAttribOffset"
    )]
    #[doc = "The `getVertexAttribOffset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/getVertexAttribOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn get_vertex_attrib_offset(this: &WebGl2RenderingContext, index: u32, pname: u32) -> f64;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `hint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/hint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn hint(this: &WebGl2RenderingContext, target: u32, mode: u32);
    #[cfg(feature = "WebGlBuffer")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isBuffer")]
    #[doc = "The `isBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlBuffer`*"]
    pub fn is_buffer(this: &WebGl2RenderingContext, buffer: Option<&WebGlBuffer>) -> bool;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isContextLost")]
    #[doc = "The `isContextLost()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isContextLost)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn is_context_lost(this: &WebGl2RenderingContext) -> bool;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isEnabled")]
    #[doc = "The `isEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn is_enabled(this: &WebGl2RenderingContext, cap: u32) -> bool;
    #[cfg(feature = "WebGlFramebuffer")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isFramebuffer")]
    #[doc = "The `isFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlFramebuffer`*"]
    pub fn is_framebuffer(
        this: &WebGl2RenderingContext,
        framebuffer: Option<&WebGlFramebuffer>,
    ) -> bool;
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isProgram")]
    #[doc = "The `isProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn is_program(this: &WebGl2RenderingContext, program: Option<&WebGlProgram>) -> bool;
    #[cfg(feature = "WebGlRenderbuffer")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "isRenderbuffer"
    )]
    #[doc = "The `isRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlRenderbuffer`*"]
    pub fn is_renderbuffer(
        this: &WebGl2RenderingContext,
        renderbuffer: Option<&WebGlRenderbuffer>,
    ) -> bool;
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isShader")]
    #[doc = "The `isShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn is_shader(this: &WebGl2RenderingContext, shader: Option<&WebGlShader>) -> bool;
    #[cfg(feature = "WebGlTexture")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "isTexture")]
    #[doc = "The `isTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/isTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlTexture`*"]
    pub fn is_texture(this: &WebGl2RenderingContext, texture: Option<&WebGlTexture>) -> bool;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "lineWidth")]
    #[doc = "The `lineWidth()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn line_width(this: &WebGl2RenderingContext, width: f32);
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "linkProgram")]
    #[doc = "The `linkProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/linkProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn link_program(this: &WebGl2RenderingContext, program: &WebGlProgram);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "makeXRCompatible"
    )]
    #[doc = "The `makeXRCompatible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/makeXRCompatible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn make_xr_compatible(
        this: &WebGl2RenderingContext,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "pixelStorei")]
    #[doc = "The `pixelStorei()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/pixelStorei)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn pixel_storei(this: &WebGl2RenderingContext, pname: u32, param: i32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "polygonOffset")]
    #[doc = "The `polygonOffset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/polygonOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn polygon_offset(this: &WebGl2RenderingContext, factor: f32, units: f32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "renderbufferStorage"
    )]
    #[doc = "The `renderbufferStorage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/renderbufferStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn renderbuffer_storage(
        this: &WebGl2RenderingContext,
        target: u32,
        internalformat: u32,
        width: i32,
        height: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "sampleCoverage"
    )]
    #[doc = "The `sampleCoverage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/sampleCoverage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn sample_coverage(this: &WebGl2RenderingContext, value: f32, invert: bool);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `scissor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/scissor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn scissor(this: &WebGl2RenderingContext, x: i32, y: i32, width: i32, height: i32);
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "shaderSource")]
    #[doc = "The `shaderSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/shaderSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlShader`*"]
    pub fn shader_source(this: &WebGl2RenderingContext, shader: &WebGlShader, source: &str);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "stencilFunc")]
    #[doc = "The `stencilFunc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilFunc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn stencil_func(this: &WebGl2RenderingContext, func: u32, ref_: i32, mask: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "stencilFuncSeparate"
    )]
    #[doc = "The `stencilFuncSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilFuncSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn stencil_func_separate(
        this: &WebGl2RenderingContext,
        face: u32,
        func: u32,
        ref_: i32,
        mask: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "stencilMask")]
    #[doc = "The `stencilMask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilMask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn stencil_mask(this: &WebGl2RenderingContext, mask: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "stencilMaskSeparate"
    )]
    #[doc = "The `stencilMaskSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilMaskSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn stencil_mask_separate(this: &WebGl2RenderingContext, face: u32, mask: u32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "stencilOp")]
    #[doc = "The `stencilOp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilOp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn stencil_op(this: &WebGl2RenderingContext, fail: u32, zfail: u32, zpass: u32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "stencilOpSeparate"
    )]
    #[doc = "The `stencilOpSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/stencilOpSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn stencil_op_separate(
        this: &WebGl2RenderingContext,
        face: u32,
        fail: u32,
        zfail: u32,
        zpass: u32,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "texParameterf")]
    #[doc = "The `texParameterf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texParameterf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_parameterf(this: &WebGl2RenderingContext, target: u32, pname: u32, param: f32);
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "texParameteri")]
    #[doc = "The `texParameteri()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/texParameteri)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn tex_parameteri(this: &WebGl2RenderingContext, target: u32, pname: u32, param: i32);
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform1f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform1i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform1i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform1i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform2f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform2i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform2i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform2i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform3f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform3i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform3i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform3i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform4f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4f(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );
    #[cfg(feature = "WebGlUniformLocation")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `uniform4i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform4i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlUniformLocation`*"]
    pub fn uniform4i(
        this: &WebGl2RenderingContext,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    );
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext", js_name = "useProgram")]
    #[doc = "The `useProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/useProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn use_program(this: &WebGl2RenderingContext, program: Option<&WebGlProgram>);
    #[cfg(feature = "WebGlProgram")]
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "validateProgram"
    )]
    #[doc = "The `validateProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/validateProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `WebGlProgram`*"]
    pub fn validate_program(this: &WebGl2RenderingContext, program: &WebGlProgram);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib1f"
    )]
    #[doc = "The `vertexAttrib1f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib1f(this: &WebGl2RenderingContext, indx: u32, x: f32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib1fv"
    )]
    #[doc = "The `vertexAttrib1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib1fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib1fv"
    )]
    #[doc = "The `vertexAttrib1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib1fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib1fv"
    )]
    #[doc = "The `vertexAttrib1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib1fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib2f"
    )]
    #[doc = "The `vertexAttrib2f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib2f(this: &WebGl2RenderingContext, indx: u32, x: f32, y: f32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib2fv"
    )]
    #[doc = "The `vertexAttrib2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib2fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib2fv"
    )]
    #[doc = "The `vertexAttrib2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib2fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib2fv"
    )]
    #[doc = "The `vertexAttrib2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib2fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib3f"
    )]
    #[doc = "The `vertexAttrib3f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib3f(this: &WebGl2RenderingContext, indx: u32, x: f32, y: f32, z: f32);
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib3fv"
    )]
    #[doc = "The `vertexAttrib3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib3fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib3fv"
    )]
    #[doc = "The `vertexAttrib3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib3fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib3fv"
    )]
    #[doc = "The `vertexAttrib3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib3fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib4f"
    )]
    #[doc = "The `vertexAttrib4f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib4f(
        this: &WebGl2RenderingContext,
        indx: u32,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib4fv"
    )]
    #[doc = "The `vertexAttrib4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib4fv_with_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &[f32],
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib4fv"
    )]
    #[doc = "The `vertexAttrib4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib4fv_with_js_f32_array(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::js_sys::Float32Array,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttrib4fv"
    )]
    #[doc = "The `vertexAttrib4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttrib4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib4fv_with_f32_sequence(
        this: &WebGl2RenderingContext,
        indx: u32,
        values: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribPointer"
    )]
    #[doc = "The `vertexAttribPointer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribPointer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_pointer_with_i32(
        this: &WebGl2RenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WebGL2RenderingContext",
        js_name = "vertexAttribPointer"
    )]
    #[doc = "The `vertexAttribPointer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribPointer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn vertex_attrib_pointer_with_f64(
        this: &WebGl2RenderingContext,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: f64,
    );
    #[wasm_bindgen(method, js_class = "WebGL2RenderingContext")]
    #[doc = "The `viewport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/viewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub fn viewport(this: &WebGl2RenderingContext, x: i32, y: i32, width: i32, height: i32);
}
impl WebGl2RenderingContext {
    #[doc = "The `WebGL2RenderingContext.READ_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const READ_BUFFER: u32 = 3074u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_ROW_LENGTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_ROW_LENGTH: u32 = 3314u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_SKIP_ROWS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_SKIP_ROWS: u32 = 3315u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_SKIP_PIXELS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_SKIP_PIXELS: u32 = 3316u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PACK_ROW_LENGTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PACK_ROW_LENGTH: u32 = 3330u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PACK_SKIP_ROWS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PACK_SKIP_ROWS: u32 = 3331u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PACK_SKIP_PIXELS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PACK_SKIP_PIXELS: u32 = 3332u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR: u32 = 6144u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH: u32 = 6145u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL: u32 = 6146u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RED: u32 = 6403u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB8: u32 = 32849u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA8: u32 = 32856u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB10_A2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB10_A2: u32 = 32857u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_BINDING_3D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_BINDING_3D: u32 = 32874u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_SKIP_IMAGES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_SKIP_IMAGES: u32 = 32877u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_IMAGE_HEIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_IMAGE_HEIGHT: u32 = 32878u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_3D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_3D: u32 = 32879u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_WRAP_R` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_WRAP_R: u32 = 32882u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_3D_TEXTURE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_3D_TEXTURE_SIZE: u32 = 32883u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_2_10_10_10_REV` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_2_10_10_10_REV: u32 = 33640u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_ELEMENTS_VERTICES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_ELEMENTS_VERTICES: u32 = 33000u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_ELEMENTS_INDICES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_ELEMENTS_INDICES: u32 = 33001u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_MIN_LOD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_MIN_LOD: u32 = 33082u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_MAX_LOD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_MAX_LOD: u32 = 33083u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_BASE_LEVEL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_BASE_LEVEL: u32 = 33084u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_MAX_LEVEL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_MAX_LEVEL: u32 = 33085u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MIN: u32 = 32775u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX: u32 = 32776u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_COMPONENT24` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_COMPONENT24: u32 = 33190u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_TEXTURE_LOD_BIAS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_TEXTURE_LOD_BIAS: u32 = 34045u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_COMPARE_MODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_COMPARE_MODE: u32 = 34892u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_COMPARE_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_COMPARE_FUNC: u32 = 34893u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CURRENT_QUERY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CURRENT_QUERY: u32 = 34917u64 as u32;
    #[doc = "The `WebGL2RenderingContext.QUERY_RESULT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const QUERY_RESULT: u32 = 34918u64 as u32;
    #[doc = "The `WebGL2RenderingContext.QUERY_RESULT_AVAILABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const QUERY_RESULT_AVAILABLE: u32 = 34919u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STREAM_READ` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STREAM_READ: u32 = 35041u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STREAM_COPY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STREAM_COPY: u32 = 35042u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STATIC_READ` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STATIC_READ: u32 = 35045u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STATIC_COPY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STATIC_COPY: u32 = 35046u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DYNAMIC_READ` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DYNAMIC_READ: u32 = 35049u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DYNAMIC_COPY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DYNAMIC_COPY: u32 = 35050u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_DRAW_BUFFERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_DRAW_BUFFERS: u32 = 34852u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER0` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER0: u32 = 34853u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER1: u32 = 34854u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER2: u32 = 34855u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER3: u32 = 34856u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER4: u32 = 34857u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER5: u32 = 34858u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER6` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER6: u32 = 34859u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER7` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER7: u32 = 34860u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER8: u32 = 34861u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER9` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER9: u32 = 34862u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER10` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER10: u32 = 34863u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER11` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER11: u32 = 34864u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER12` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER12: u32 = 34865u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER13` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER13: u32 = 34866u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER14` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER14: u32 = 34867u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_BUFFER15` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_BUFFER15: u32 = 34868u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_FRAGMENT_UNIFORM_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 35657u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VERTEX_UNIFORM_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 35658u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_3D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_3D: u32 = 35679u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_2D_SHADOW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_2D_SHADOW: u32 = 35682u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAGMENT_SHADER_DERIVATIVE_HINT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 35723u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PIXEL_PACK_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PIXEL_PACK_BUFFER: u32 = 35051u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PIXEL_UNPACK_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PIXEL_UNPACK_BUFFER: u32 = 35052u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PIXEL_PACK_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PIXEL_PACK_BUFFER_BINDING: u32 = 35053u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PIXEL_UNPACK_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PIXEL_UNPACK_BUFFER_BINDING: u32 = 35055u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT2x3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT2X3: u32 = 35685u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT2x4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT2X4: u32 = 35686u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT3x2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT3X2: u32 = 35687u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT3x4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT3X4: u32 = 35688u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT4x2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT4X2: u32 = 35689u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT4x3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT4X3: u32 = 35690u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SRGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SRGB: u32 = 35904u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SRGB8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SRGB8: u32 = 35905u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SRGB8_ALPHA8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SRGB8_ALPHA8: u32 = 35907u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COMPARE_REF_TO_TEXTURE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COMPARE_REF_TO_TEXTURE: u32 = 34894u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA32F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA32F: u32 = 34836u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB32F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB32F: u32 = 34837u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA16F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA16F: u32 = 34842u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB16F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB16F: u32 = 34843u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_INTEGER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 35069u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_ARRAY_TEXTURE_LAYERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_ARRAY_TEXTURE_LAYERS: u32 = 35071u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MIN_PROGRAM_TEXEL_OFFSET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MIN_PROGRAM_TEXEL_OFFSET: u32 = 35076u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_PROGRAM_TEXEL_OFFSET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_PROGRAM_TEXEL_OFFSET: u32 = 35077u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VARYING_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VARYING_COMPONENTS: u32 = 35659u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_2D_ARRAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_2D_ARRAY: u32 = 35866u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_BINDING_2D_ARRAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_BINDING_2D_ARRAY: u32 = 35869u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R11F_G11F_B10F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R11F_G11F_B10F: u32 = 35898u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_10F_11F_11F_REV` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_10F_11F_11F_REV: u32 = 35899u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB9_E5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB9_E5: u32 = 35901u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_5_9_9_9_REV` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_5_9_9_9_REV: u32 = 35902u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_MODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 35967u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 35968u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_VARYINGS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_VARYINGS: u32 = 35971u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_START` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_BUFFER_START: u32 = 35972u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 35973u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 35976u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RASTERIZER_DISCARD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RASTERIZER_DISCARD: u32 = 35977u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 35978u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 35979u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INTERLEAVED_ATTRIBS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INTERLEAVED_ATTRIBS: u32 = 35980u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SEPARATE_ATTRIBS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SEPARATE_ATTRIBS: u32 = 35981u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_BUFFER: u32 = 35982u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 35983u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA32UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA32UI: u32 = 36208u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB32UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB32UI: u32 = 36209u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA16UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA16UI: u32 = 36214u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB16UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB16UI: u32 = 36215u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA8UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA8UI: u32 = 36220u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB8UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB8UI: u32 = 36221u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA32I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA32I: u32 = 36226u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB32I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB32I: u32 = 36227u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA16I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA16I: u32 = 36232u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB16I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB16I: u32 = 36233u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA8I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA8I: u32 = 36238u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB8I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB8I: u32 = 36239u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RED_INTEGER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RED_INTEGER: u32 = 36244u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB_INTEGER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB_INTEGER: u32 = 36248u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA_INTEGER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA_INTEGER: u32 = 36249u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_2D_ARRAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_2D_ARRAY: u32 = 36289u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_2D_ARRAY_SHADOW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_2D_ARRAY_SHADOW: u32 = 36292u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_CUBE_SHADOW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_CUBE_SHADOW: u32 = 36293u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_VEC2: u32 = 36294u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_VEC3: u32 = 36295u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_VEC4: u32 = 36296u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_SAMPLER_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_SAMPLER_2D: u32 = 36298u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_SAMPLER_3D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_SAMPLER_3D: u32 = 36299u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_SAMPLER_CUBE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_SAMPLER_CUBE: u32 = 36300u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_SAMPLER_2D_ARRAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_SAMPLER_2D_ARRAY: u32 = 36303u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_SAMPLER_2D: u32 = 36306u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_3D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_SAMPLER_3D: u32 = 36307u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_CUBE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_SAMPLER_CUBE: u32 = 36308u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_SAMPLER_2D_ARRAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 36311u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_COMPONENT32F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_COMPONENT32F: u32 = 36012u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH32F_STENCIL8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH32F_STENCIL8: u32 = 36013u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_32_UNSIGNED_INT_24_8_REV` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 36269u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 33296u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 33297u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_RED_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 33298u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_GREEN_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 33299u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_BLUE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 33300u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 33301u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 33302u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 33303u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_DEFAULT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_DEFAULT: u32 = 33304u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT_24_8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT_24_8: u32 = 34042u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH24_STENCIL8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH24_STENCIL8: u32 = 35056u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_NORMALIZED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_NORMALIZED: u32 = 35863u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_FRAMEBUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;
    #[doc = "The `WebGL2RenderingContext.READ_FRAMEBUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const READ_FRAMEBUFFER: u32 = 36008u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DRAW_FRAMEBUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DRAW_FRAMEBUFFER: u32 = 36009u64 as u32;
    #[doc = "The `WebGL2RenderingContext.READ_FRAMEBUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const READ_FRAMEBUFFER_BINDING: u32 = 36010u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_SAMPLES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_SAMPLES: u32 = 36011u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 36052u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_COLOR_ATTACHMENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_COLOR_ATTACHMENTS: u32 = 36063u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT1: u32 = 36065u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT2: u32 = 36066u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT3: u32 = 36067u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT4: u32 = 36068u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT5: u32 = 36069u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT6` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT6: u32 = 36070u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT7` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT7: u32 = 36071u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT8: u32 = 36072u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT9` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT9: u32 = 36073u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT10` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT10: u32 = 36074u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT11` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT11: u32 = 36075u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT12` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT12: u32 = 36076u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT13` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT13: u32 = 36077u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT14` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT14: u32 = 36078u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT15` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT15: u32 = 36079u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_MULTISAMPLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 36182u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_SAMPLES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_SAMPLES: u32 = 36183u64 as u32;
    #[doc = "The `WebGL2RenderingContext.HALF_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const HALF_FLOAT: u32 = 5131u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG: u32 = 33319u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG_INTEGER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG_INTEGER: u32 = 33320u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R8: u32 = 33321u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG8: u32 = 33323u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R16F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R16F: u32 = 33325u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R32F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R32F: u32 = 33326u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG16F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG16F: u32 = 33327u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG32F` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG32F: u32 = 33328u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R8I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R8I: u32 = 33329u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R8UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R8UI: u32 = 33330u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R16I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R16I: u32 = 33331u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R16UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R16UI: u32 = 33332u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R32I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R32I: u32 = 33333u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R32UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R32UI: u32 = 33334u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG8I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG8I: u32 = 33335u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG8UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG8UI: u32 = 33336u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG16I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG16I: u32 = 33337u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG16UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG16UI: u32 = 33338u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG32I` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG32I: u32 = 33339u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG32UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG32UI: u32 = 33340u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ARRAY_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ARRAY_BINDING: u32 = 34229u64 as u32;
    #[doc = "The `WebGL2RenderingContext.R8_SNORM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const R8_SNORM: u32 = 36756u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RG8_SNORM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RG8_SNORM: u32 = 36757u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB8_SNORM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB8_SNORM: u32 = 36758u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA8_SNORM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA8_SNORM: u32 = 36759u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SIGNED_NORMALIZED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SIGNED_NORMALIZED: u32 = 36764u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COPY_READ_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COPY_READ_BUFFER: u32 = 36662u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COPY_WRITE_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COPY_WRITE_BUFFER: u32 = 36663u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COPY_READ_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COPY_READ_BUFFER_BINDING: u32 = 36662u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COPY_WRITE_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COPY_WRITE_BUFFER_BINDING: u32 = 36663u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BUFFER: u32 = 35345u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BUFFER_BINDING: u32 = 35368u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BUFFER_START` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BUFFER_START: u32 = 35369u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BUFFER_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BUFFER_SIZE: u32 = 35370u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VERTEX_UNIFORM_BLOCKS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VERTEX_UNIFORM_BLOCKS: u32 = 35371u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_FRAGMENT_UNIFORM_BLOCKS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 35373u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_COMBINED_UNIFORM_BLOCKS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_COMBINED_UNIFORM_BLOCKS: u32 = 35374u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_UNIFORM_BUFFER_BINDINGS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_UNIFORM_BUFFER_BINDINGS: u32 = 35375u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_UNIFORM_BLOCK_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_UNIFORM_BLOCK_SIZE: u32 = 35376u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 35377u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 35379u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BUFFER_OFFSET_ALIGNMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 35380u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ACTIVE_UNIFORM_BLOCKS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ACTIVE_UNIFORM_BLOCKS: u32 = 35382u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_TYPE: u32 = 35383u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_SIZE: u32 = 35384u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_INDEX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_INDEX: u32 = 35386u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_OFFSET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_OFFSET: u32 = 35387u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_ARRAY_STRIDE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_ARRAY_STRIDE: u32 = 35388u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_MATRIX_STRIDE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_MATRIX_STRIDE: u32 = 35389u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_IS_ROW_MAJOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_IS_ROW_MAJOR: u32 = 35390u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_BINDING: u32 = 35391u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_DATA_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_DATA_SIZE: u32 = 35392u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_ACTIVE_UNIFORMS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 35394u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 35395u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 35396u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 35398u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INVALID_INDEX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INVALID_INDEX: u32 = 4294967295u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VERTEX_OUTPUT_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 37154u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_FRAGMENT_INPUT_COMPONENTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 37157u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_SERVER_WAIT_TIMEOUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_SERVER_WAIT_TIMEOUT: u32 = 37137u64 as u32;
    #[doc = "The `WebGL2RenderingContext.OBJECT_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const OBJECT_TYPE: u32 = 37138u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SYNC_CONDITION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SYNC_CONDITION: u32 = 37139u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SYNC_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SYNC_STATUS: u32 = 37140u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SYNC_FLAGS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SYNC_FLAGS: u32 = 37141u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SYNC_FENCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SYNC_FENCE: u32 = 37142u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SYNC_GPU_COMMANDS_COMPLETE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SYNC_GPU_COMMANDS_COMPLETE: u32 = 37143u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNALED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNALED: u32 = 37144u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SIGNALED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SIGNALED: u32 = 37145u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ALREADY_SIGNALED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ALREADY_SIGNALED: u32 = 37146u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TIMEOUT_EXPIRED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TIMEOUT_EXPIRED: u32 = 37147u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CONDITION_SATISFIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CONDITION_SATISFIED: u32 = 37148u64 as u32;
    #[doc = "The `WebGL2RenderingContext.WAIT_FAILED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const WAIT_FAILED: u32 = 37149u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SYNC_FLUSH_COMMANDS_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SYNC_FLUSH_COMMANDS_BIT: u32 = 1u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_DIVISOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 35070u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ANY_SAMPLES_PASSED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ANY_SAMPLES_PASSED: u32 = 35887u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ANY_SAMPLES_PASSED_CONSERVATIVE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 36202u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_BINDING: u32 = 35097u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB10_A2UI` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB10_A2UI: u32 = 36975u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_2_10_10_10_REV` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_2_10_10_10_REV: u32 = 36255u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK: u32 = 36386u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_PAUSED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_PAUSED: u32 = 36387u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_ACTIVE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_ACTIVE: u32 = 36388u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRANSFORM_FEEDBACK_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRANSFORM_FEEDBACK_BINDING: u32 = 36389u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_IMMUTABLE_FORMAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_IMMUTABLE_FORMAT: u32 = 37167u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_ELEMENT_INDEX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_ELEMENT_INDEX: u32 = 36203u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_IMMUTABLE_LEVELS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_IMMUTABLE_LEVELS: u32 = 33503u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TIMEOUT_IGNORED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TIMEOUT_IGNORED: f64 = -1i64 as f64;
    #[doc = "The `WebGL2RenderingContext.MAX_CLIENT_WAIT_TIMEOUT_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: u32 = 37447u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_BUFFER_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_BUFFER_BIT: u32 = 256u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BUFFER_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BUFFER_BIT: u32 = 1024u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_BUFFER_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_BUFFER_BIT: u32 = 16384u64 as u32;
    #[doc = "The `WebGL2RenderingContext.POINTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const POINTS: u32 = 0u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINES: u32 = 1u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINE_LOOP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINE_LOOP: u32 = 2u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINE_STRIP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINE_STRIP: u32 = 3u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRIANGLES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRIANGLES: u32 = 4u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRIANGLE_STRIP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRIANGLE_STRIP: u32 = 5u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TRIANGLE_FAN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TRIANGLE_FAN: u32 = 6u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ZERO` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ZERO: u32 = 0i64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE: u32 = 1u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SRC_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SRC_COLOR: u32 = 768u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE_MINUS_SRC_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE_MINUS_SRC_COLOR: u32 = 769u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SRC_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SRC_ALPHA: u32 = 770u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE_MINUS_SRC_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE_MINUS_SRC_ALPHA: u32 = 771u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DST_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DST_ALPHA: u32 = 772u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE_MINUS_DST_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE_MINUS_DST_ALPHA: u32 = 773u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DST_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DST_COLOR: u32 = 774u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE_MINUS_DST_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE_MINUS_DST_COLOR: u32 = 775u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SRC_ALPHA_SATURATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SRC_ALPHA_SATURATE: u32 = 776u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FUNC_ADD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FUNC_ADD: u32 = 32774u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_EQUATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_EQUATION: u32 = 32777u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_EQUATION_RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_EQUATION_RGB: u32 = 32777u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_EQUATION_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_EQUATION_ALPHA: u32 = 34877u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FUNC_SUBTRACT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FUNC_SUBTRACT: u32 = 32778u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FUNC_REVERSE_SUBTRACT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FUNC_REVERSE_SUBTRACT: u32 = 32779u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_DST_RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_DST_RGB: u32 = 32968u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_SRC_RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_SRC_RGB: u32 = 32969u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_DST_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_DST_ALPHA: u32 = 32970u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_SRC_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_SRC_ALPHA: u32 = 32971u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CONSTANT_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CONSTANT_COLOR: u32 = 32769u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE_MINUS_CONSTANT_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CONSTANT_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CONSTANT_ALPHA: u32 = 32771u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ONE_MINUS_CONSTANT_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND_COLOR: u32 = 32773u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ARRAY_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ARRAY_BUFFER: u32 = 34962u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ELEMENT_ARRAY_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ARRAY_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ARRAY_BUFFER_BINDING: u32 = 34964u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ELEMENT_ARRAY_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STREAM_DRAW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STREAM_DRAW: u32 = 35040u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STATIC_DRAW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STATIC_DRAW: u32 = 35044u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DYNAMIC_DRAW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DYNAMIC_DRAW: u32 = 35048u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BUFFER_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BUFFER_SIZE: u32 = 34660u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BUFFER_USAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BUFFER_USAGE: u32 = 34661u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CURRENT_VERTEX_ATTRIB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CURRENT_VERTEX_ATTRIB: u32 = 34342u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRONT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRONT: u32 = 1028u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BACK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BACK: u32 = 1029u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRONT_AND_BACK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRONT_AND_BACK: u32 = 1032u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CULL_FACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CULL_FACE: u32 = 2884u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLEND` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLEND: u32 = 3042u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DITHER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DITHER: u32 = 3024u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_TEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_TEST: u32 = 2960u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_TEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_TEST: u32 = 2929u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SCISSOR_TEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SCISSOR_TEST: u32 = 3089u64 as u32;
    #[doc = "The `WebGL2RenderingContext.POLYGON_OFFSET_FILL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const POLYGON_OFFSET_FILL: u32 = 32823u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLE_ALPHA_TO_COVERAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLE_COVERAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLE_COVERAGE: u32 = 32928u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NO_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NO_ERROR: u32 = 0i64 as u32;
    #[doc = "The `WebGL2RenderingContext.INVALID_ENUM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INVALID_ENUM: u32 = 1280u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INVALID_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INVALID_VALUE: u32 = 1281u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INVALID_OPERATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INVALID_OPERATION: u32 = 1282u64 as u32;
    #[doc = "The `WebGL2RenderingContext.OUT_OF_MEMORY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const OUT_OF_MEMORY: u32 = 1285u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CW: u32 = 2304u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CCW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CCW: u32 = 2305u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINE_WIDTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINE_WIDTH: u32 = 2849u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ALIASED_POINT_SIZE_RANGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ALIASED_POINT_SIZE_RANGE: u32 = 33901u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ALIASED_LINE_WIDTH_RANGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ALIASED_LINE_WIDTH_RANGE: u32 = 33902u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CULL_FACE_MODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CULL_FACE_MODE: u32 = 2885u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRONT_FACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRONT_FACE: u32 = 2886u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_RANGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_RANGE: u32 = 2928u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_WRITEMASK: u32 = 2930u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_CLEAR_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_CLEAR_VALUE: u32 = 2931u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_FUNC: u32 = 2932u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_CLEAR_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_CLEAR_VALUE: u32 = 2961u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_FUNC: u32 = 2962u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_FAIL: u32 = 2964u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_PASS_DEPTH_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_PASS_DEPTH_FAIL: u32 = 2965u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_PASS_DEPTH_PASS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_PASS_DEPTH_PASS: u32 = 2966u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_REF` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_REF: u32 = 2967u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_VALUE_MASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_VALUE_MASK: u32 = 2963u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_WRITEMASK: u32 = 2968u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_FUNC: u32 = 34816u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_FAIL: u32 = 34817u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_PASS_DEPTH_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_PASS_DEPTH_PASS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_REF` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_REF: u32 = 36003u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_VALUE_MASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_VALUE_MASK: u32 = 36004u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BACK_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BACK_WRITEMASK: u32 = 36005u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VIEWPORT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VIEWPORT: u32 = 2978u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SCISSOR_BOX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SCISSOR_BOX: u32 = 3088u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_CLEAR_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_CLEAR_VALUE: u32 = 3106u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_WRITEMASK: u32 = 3107u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_ALIGNMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_ALIGNMENT: u32 = 3317u64 as u32;
    #[doc = "The `WebGL2RenderingContext.PACK_ALIGNMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const PACK_ALIGNMENT: u32 = 3333u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_TEXTURE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_TEXTURE_SIZE: u32 = 3379u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VIEWPORT_DIMS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VIEWPORT_DIMS: u32 = 3386u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SUBPIXEL_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SUBPIXEL_BITS: u32 = 3408u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RED_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RED_BITS: u32 = 3410u64 as u32;
    #[doc = "The `WebGL2RenderingContext.GREEN_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const GREEN_BITS: u32 = 3411u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BLUE_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BLUE_BITS: u32 = 3412u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ALPHA_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ALPHA_BITS: u32 = 3413u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_BITS: u32 = 3414u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_BITS: u32 = 3415u64 as u32;
    #[doc = "The `WebGL2RenderingContext.POLYGON_OFFSET_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const POLYGON_OFFSET_UNITS: u32 = 10752u64 as u32;
    #[doc = "The `WebGL2RenderingContext.POLYGON_OFFSET_FACTOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const POLYGON_OFFSET_FACTOR: u32 = 32824u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_BINDING_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_BINDING_2D: u32 = 32873u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLE_BUFFERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLE_BUFFERS: u32 = 32936u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLES: u32 = 32937u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLE_COVERAGE_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLE_COVERAGE_VALUE: u32 = 32938u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLE_COVERAGE_INVERT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLE_COVERAGE_INVERT: u32 = 32939u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COMPRESSED_TEXTURE_FORMATS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COMPRESSED_TEXTURE_FORMATS: u32 = 34467u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DONT_CARE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DONT_CARE: u32 = 4352u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FASTEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FASTEST: u32 = 4353u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NICEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NICEST: u32 = 4354u64 as u32;
    #[doc = "The `WebGL2RenderingContext.GENERATE_MIPMAP_HINT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const GENERATE_MIPMAP_HINT: u32 = 33170u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BYTE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BYTE: u32 = 5120u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_BYTE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_BYTE: u32 = 5121u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SHORT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SHORT: u32 = 5122u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_SHORT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_SHORT: u32 = 5123u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT: u32 = 5124u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_INT: u32 = 5125u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT: u32 = 5126u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_COMPONENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_COMPONENT: u32 = 6402u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ALPHA: u32 = 6406u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB: u32 = 6407u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA: u32 = 6408u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LUMINANCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LUMINANCE: u32 = 6409u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LUMINANCE_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LUMINANCE_ALPHA: u32 = 6410u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_SHORT_4_4_4_4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 32819u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_SHORT_5_5_5_1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 32820u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNSIGNED_SHORT_5_6_5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNSIGNED_SHORT_5_6_5: u32 = 33635u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAGMENT_SHADER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAGMENT_SHADER: u32 = 35632u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_SHADER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_SHADER: u32 = 35633u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VERTEX_ATTRIBS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VERTEX_ATTRIBS: u32 = 34921u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VERTEX_UNIFORM_VECTORS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VARYING_VECTORS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VARYING_VECTORS: u32 = 36348u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_COMBINED_TEXTURE_IMAGE_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_VERTEX_TEXTURE_IMAGE_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_TEXTURE_IMAGE_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 34930u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_FRAGMENT_UNIFORM_VECTORS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SHADER_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SHADER_TYPE: u32 = 35663u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DELETE_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DELETE_STATUS: u32 = 35712u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINK_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINK_STATUS: u32 = 35714u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VALIDATE_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VALIDATE_STATUS: u32 = 35715u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ATTACHED_SHADERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ATTACHED_SHADERS: u32 = 35717u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ACTIVE_UNIFORMS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ACTIVE_UNIFORMS: u32 = 35718u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ACTIVE_ATTRIBUTES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ACTIVE_ATTRIBUTES: u32 = 35721u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SHADING_LANGUAGE_VERSION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SHADING_LANGUAGE_VERSION: u32 = 35724u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CURRENT_PROGRAM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CURRENT_PROGRAM: u32 = 35725u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NEVER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NEVER: u32 = 512u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LESS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LESS: u32 = 513u64 as u32;
    #[doc = "The `WebGL2RenderingContext.EQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const EQUAL: u32 = 514u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LEQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LEQUAL: u32 = 515u64 as u32;
    #[doc = "The `WebGL2RenderingContext.GREATER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const GREATER: u32 = 516u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NOTEQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NOTEQUAL: u32 = 517u64 as u32;
    #[doc = "The `WebGL2RenderingContext.GEQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const GEQUAL: u32 = 518u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ALWAYS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ALWAYS: u32 = 519u64 as u32;
    #[doc = "The `WebGL2RenderingContext.KEEP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const KEEP: u32 = 7680u64 as u32;
    #[doc = "The `WebGL2RenderingContext.REPLACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const REPLACE: u32 = 7681u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INCR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INCR: u32 = 7682u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DECR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DECR: u32 = 7683u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INVERT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INVERT: u32 = 5386u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INCR_WRAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INCR_WRAP: u32 = 34055u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DECR_WRAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DECR_WRAP: u32 = 34056u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VENDOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VENDOR: u32 = 7936u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERER: u32 = 7937u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERSION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERSION: u32 = 7938u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NEAREST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NEAREST: u32 = 9728u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINEAR: u32 = 9729u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NEAREST_MIPMAP_NEAREST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINEAR_MIPMAP_NEAREST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NEAREST_MIPMAP_LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LINEAR_MIPMAP_LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_MAG_FILTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_MAG_FILTER: u32 = 10240u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_MIN_FILTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_MIN_FILTER: u32 = 10241u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_WRAP_S` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_WRAP_S: u32 = 10242u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_WRAP_T` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_WRAP_T: u32 = 10243u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_2D: u32 = 3553u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE: u32 = 5890u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP: u32 = 34067u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_BINDING_CUBE_MAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_BINDING_CUBE_MAP: u32 = 34068u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_POSITIVE_X` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_X` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Y` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Y` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Z` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Z` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_CUBE_MAP_TEXTURE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE0` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE0: u32 = 33984u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE1: u32 = 33985u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE2: u32 = 33986u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE3: u32 = 33987u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE4: u32 = 33988u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE5: u32 = 33989u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE6` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE6: u32 = 33990u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE7` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE7: u32 = 33991u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE8: u32 = 33992u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE9` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE9: u32 = 33993u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE10` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE10: u32 = 33994u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE11` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE11: u32 = 33995u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE12` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE12: u32 = 33996u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE13` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE13: u32 = 33997u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE14` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE14: u32 = 33998u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE15` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE15: u32 = 33999u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE16` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE16: u32 = 34000u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE17` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE17: u32 = 34001u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE18` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE18: u32 = 34002u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE19` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE19: u32 = 34003u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE20` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE20: u32 = 34004u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE21` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE21: u32 = 34005u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE22` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE22: u32 = 34006u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE23` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE23: u32 = 34007u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE24` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE24: u32 = 34008u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE25` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE25: u32 = 34009u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE26` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE26: u32 = 34010u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE27` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE27: u32 = 34011u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE28` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE28: u32 = 34012u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE29` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE29: u32 = 34013u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE30` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE30: u32 = 34014u64 as u32;
    #[doc = "The `WebGL2RenderingContext.TEXTURE31` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const TEXTURE31: u32 = 34015u64 as u32;
    #[doc = "The `WebGL2RenderingContext.ACTIVE_TEXTURE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const ACTIVE_TEXTURE: u32 = 34016u64 as u32;
    #[doc = "The `WebGL2RenderingContext.REPEAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const REPEAT: u32 = 10497u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CLAMP_TO_EDGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CLAMP_TO_EDGE: u32 = 33071u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MIRRORED_REPEAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MIRRORED_REPEAT: u32 = 33648u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_VEC2: u32 = 35664u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_VEC3: u32 = 35665u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_VEC4: u32 = 35666u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_VEC2: u32 = 35667u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_VEC3: u32 = 35668u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INT_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INT_VEC4: u32 = 35669u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BOOL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BOOL: u32 = 35670u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BOOL_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BOOL_VEC2: u32 = 35671u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BOOL_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BOOL_VEC3: u32 = 35672u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BOOL_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BOOL_VEC4: u32 = 35673u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT2: u32 = 35674u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT3: u32 = 35675u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FLOAT_MAT4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FLOAT_MAT4: u32 = 35676u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_2D: u32 = 35678u64 as u32;
    #[doc = "The `WebGL2RenderingContext.SAMPLER_CUBE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const SAMPLER_CUBE: u32 = 35680u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_ENABLED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_STRIDE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_NORMALIZED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_POINTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373u64 as u32;
    #[doc = "The `WebGL2RenderingContext.VERTEX_ATTRIB_ARRAY_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975u64 as u32;
    #[doc = "The `WebGL2RenderingContext.IMPLEMENTATION_COLOR_READ_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738u64 as u32;
    #[doc = "The `WebGL2RenderingContext.IMPLEMENTATION_COLOR_READ_FORMAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COMPILE_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COMPILE_STATUS: u32 = 35713u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LOW_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LOW_FLOAT: u32 = 36336u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MEDIUM_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MEDIUM_FLOAT: u32 = 36337u64 as u32;
    #[doc = "The `WebGL2RenderingContext.HIGH_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const HIGH_FLOAT: u32 = 36338u64 as u32;
    #[doc = "The `WebGL2RenderingContext.LOW_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const LOW_INT: u32 = 36339u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MEDIUM_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MEDIUM_INT: u32 = 36340u64 as u32;
    #[doc = "The `WebGL2RenderingContext.HIGH_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const HIGH_INT: u32 = 36341u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER: u32 = 36160u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER: u32 = 36161u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGBA4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGBA4: u32 = 32854u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB5_A1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB5_A1: u32 = 32855u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RGB565` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RGB565: u32 = 36194u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_COMPONENT16` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_COMPONENT16: u32 = 33189u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_INDEX8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_INDEX8: u32 = 36168u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_STENCIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_STENCIL: u32 = 34041u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_WIDTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_WIDTH: u32 = 36162u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_HEIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_HEIGHT: u32 = 36163u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_INTERNAL_FORMAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_RED_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_RED_SIZE: u32 = 36176u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_GREEN_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_GREEN_SIZE: u32 = 36177u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_BLUE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_BLUE_SIZE: u32 = 36178u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_ALPHA_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_ALPHA_SIZE: u32 = 36179u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_DEPTH_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_DEPTH_SIZE: u32 = 36180u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_STENCIL_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_STENCIL_SIZE: u32 = 36181u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_NAME` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051u64 as u32;
    #[doc = "The `WebGL2RenderingContext.COLOR_ATTACHMENT0` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const COLOR_ATTACHMENT0: u32 = 36064u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_ATTACHMENT: u32 = 36096u64 as u32;
    #[doc = "The `WebGL2RenderingContext.STENCIL_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const STENCIL_ATTACHMENT: u32 = 36128u64 as u32;
    #[doc = "The `WebGL2RenderingContext.DEPTH_STENCIL_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const DEPTH_STENCIL_ATTACHMENT: u32 = 33306u64 as u32;
    #[doc = "The `WebGL2RenderingContext.NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const NONE: u32 = 0i64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_COMPLETE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_COMPLETE: u32 = 36053u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_INCOMPLETE_DIMENSIONS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_UNSUPPORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_UNSUPPORTED: u32 = 36061u64 as u32;
    #[doc = "The `WebGL2RenderingContext.FRAMEBUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;
    #[doc = "The `WebGL2RenderingContext.RENDERBUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const RENDERBUFFER_BINDING: u32 = 36007u64 as u32;
    #[doc = "The `WebGL2RenderingContext.MAX_RENDERBUFFER_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const MAX_RENDERBUFFER_SIZE: u32 = 34024u64 as u32;
    #[doc = "The `WebGL2RenderingContext.INVALID_FRAMEBUFFER_OPERATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_FLIP_Y_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_FLIP_Y_WEBGL: u32 = 37440u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_PREMULTIPLY_ALPHA_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441u64 as u32;
    #[doc = "The `WebGL2RenderingContext.CONTEXT_LOST_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const CONTEXT_LOST_WEBGL: u32 = 37442u64 as u32;
    #[doc = "The `WebGL2RenderingContext.UNPACK_COLORSPACE_CONVERSION_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 37443u64 as u32;
    #[doc = "The `WebGL2RenderingContext.BROWSER_DEFAULT_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`*"]
    pub const BROWSER_DEFAULT_WEBGL: u32 = 37444u64 as u32;
}
