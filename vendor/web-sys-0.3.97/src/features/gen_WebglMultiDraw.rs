#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WEBGL_multi_draw",
        typescript_type = "WEBGL_multi_draw"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglMultiDraw` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub type WebglMultiDraw;
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_slice_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_slice_and_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_sequence_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_slice_and_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_slice_and_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_js_i32_array_and_js_i32_array_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_js_i32_array_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_slice_and_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_js_i32_array_and_i32_sequence_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_sequence_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_slice_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_slice_and_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_js_i32_array_and_js_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_js_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_array_and_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_js_i32_array_and_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysInstancedWEBGL"
    )]
    #[doc = "The `multiDrawArraysInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_instanced_webgl_with_i32_sequence_and_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_js_i32_array_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_sequence_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &mut [i32],
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_js_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::js_sys::Int32Array,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawArraysWEBGL"
    )]
    #[doc = "The `multiDrawArraysWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_arrays_webgl_with_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        firsts_list: &::wasm_bindgen::JsValue,
        firsts_offset: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_slice_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_slice_and_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_sequence_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &mut [i32],
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_slice_and_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_slice_and_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_js_i32_array_and_js_i32_array_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_js_i32_array_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_slice_and_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_js_i32_array_and_i32_sequence_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_sequence_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &::js_sys::Int32Array,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_slice_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_slice_and_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_js_i32_array_and_js_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_js_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_array_and_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_js_i32_array_and_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsInstancedWEBGL"
    )]
    #[doc = "The `multiDrawElementsInstancedWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_instanced_webgl_with_i32_sequence_and_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        instance_counts_list: &::wasm_bindgen::JsValue,
        instance_counts_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_array_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_array_and_i32_slice(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_sequence_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &mut [i32],
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_slice_and_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_js_i32_array_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_sequence_and_js_i32_array(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::js_sys::Int32Array,
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &mut [i32],
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_js_i32_array_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::js_sys::Int32Array,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        drawcount: i32,
    );
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_multi_draw",
        js_name = "multiDrawElementsWEBGL"
    )]
    #[doc = "The `multiDrawElementsWEBGL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglMultiDraw`*"]
    pub fn multi_draw_elements_webgl_with_i32_sequence_and_i32_sequence(
        this: &WebglMultiDraw,
        mode: u32,
        counts_list: &::wasm_bindgen::JsValue,
        counts_offset: u32,
        type_: u32,
        offsets_list: &::wasm_bindgen::JsValue,
        offsets_offset: u32,
        drawcount: i32,
    );
}
