#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUQueue",
        typescript_type = "GPUQueue"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuQueue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuQueue;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUQueue", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuQueue) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "GPUQueue", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuQueue, value: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuCopyExternalImageDestInfo",
        feature = "GpuCopyExternalImageSourceInfo",
    ))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUQueue",
        js_name = "copyExternalImageToTexture"
    )]
    #[doc = "The `copyExternalImageToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyExternalImageToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageDestInfo`, `GpuCopyExternalImageSourceInfo`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_external_image_to_texture_with_u32_sequence(
        this: &GpuQueue,
        source: &GpuCopyExternalImageSourceInfo,
        destination: &GpuCopyExternalImageDestInfo,
        copy_size: &[::js_sys::Number],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuCopyExternalImageDestInfo",
        feature = "GpuCopyExternalImageSourceInfo",
        feature = "GpuExtent3dDict",
    ))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUQueue",
        js_name = "copyExternalImageToTexture"
    )]
    #[doc = "The `copyExternalImageToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyExternalImageToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageDestInfo`, `GpuCopyExternalImageSourceInfo`, `GpuExtent3dDict`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_external_image_to_texture_with_gpu_extent_3d_dict(
        this: &GpuQueue,
        source: &GpuCopyExternalImageSourceInfo,
        destination: &GpuCopyExternalImageDestInfo,
        copy_size: &GpuExtent3dDict,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUQueue", js_name = "onSubmittedWorkDone")]
    #[doc = "The `onSubmittedWorkDone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/onSubmittedWorkDone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn on_submitted_work_done(this: &GpuQueue) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandBuffer")]
    #[wasm_bindgen(method, js_class = "GPUQueue")]
    #[doc = "The `submit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/submit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn submit(this: &GpuQueue, command_buffers: &[GpuCommandBuffer]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
        data_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
        data_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
        data_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
        data_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
        data_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
        data_offset: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
        data_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
        data_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
        data_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
        data_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
        data_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
        data_offset: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source_and_u32_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
        data_offset: u32,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source_and_u32_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
        data_offset: u32,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice_and_u32_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
        data_offset: u32,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice_and_u32_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
        data_offset: u32,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array_and_u32_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
        data_offset: u32,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array_and_u32_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
        data_offset: u32,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source_and_f64_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
        data_offset: f64,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source_and_f64_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
        data_offset: f64,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice_and_f64_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
        data_offset: f64,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice_and_f64_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
        data_offset: f64,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array_and_f64_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
        data_offset: f64,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array_and_f64_and_u32(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
        data_offset: f64,
        size: u32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source_and_u32_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
        data_offset: u32,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source_and_u32_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
        data_offset: u32,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice_and_u32_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
        data_offset: u32,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice_and_u32_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
        data_offset: u32,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array_and_u32_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
        data_offset: u32,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array_and_u32_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
        data_offset: u32,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_buffer_source_and_f64_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Object,
        data_offset: f64,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_buffer_source_and_f64_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Object,
        data_offset: f64,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_slice_and_f64_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &[u8],
        data_offset: f64,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_slice_and_f64_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &[u8],
        data_offset: f64,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_u32_and_u8_array_and_f64_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: u32,
        data: &::js_sys::Uint8Array,
        data_offset: f64,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_buffer_with_f64_and_u8_array_and_f64_and_f64(
        this: &GpuQueue,
        buffer: &GpuBuffer,
        buffer_offset: f64,
        data: &::js_sys::Uint8Array,
        data_offset: f64,
        size: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeTexture")]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_texture_with_buffer_source_and_u32_sequence(
        this: &GpuQueue,
        destination: &GpuTexelCopyTextureInfo,
        data: &::js_sys::Object,
        data_layout: &GpuTexelCopyBufferLayout,
        size: &[::js_sys::Number],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeTexture")]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_texture_with_u8_slice_and_u32_sequence(
        this: &GpuQueue,
        destination: &GpuTexelCopyTextureInfo,
        data: &[u8],
        data_layout: &GpuTexelCopyBufferLayout,
        size: &[::js_sys::Number],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeTexture")]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_texture_with_u8_array_and_u32_sequence(
        this: &GpuQueue,
        destination: &GpuTexelCopyTextureInfo,
        data: &::js_sys::Uint8Array,
        data_layout: &GpuTexelCopyBufferLayout,
        size: &[::js_sys::Number],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeTexture")]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_texture_with_buffer_source_and_gpu_extent_3d_dict(
        this: &GpuQueue,
        destination: &GpuTexelCopyTextureInfo,
        data: &::js_sys::Object,
        data_layout: &GpuTexelCopyBufferLayout,
        size: &GpuExtent3dDict,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeTexture")]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_texture_with_u8_slice_and_gpu_extent_3d_dict(
        this: &GpuQueue,
        destination: &GpuTexelCopyTextureInfo,
        data: &[u8],
        data_layout: &GpuTexelCopyBufferLayout,
        size: &GpuExtent3dDict,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[wasm_bindgen(catch, method, js_class = "GPUQueue", js_name = "writeTexture")]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_texture_with_u8_array_and_gpu_extent_3d_dict(
        this: &GpuQueue,
        destination: &GpuTexelCopyTextureInfo,
        data: &::js_sys::Uint8Array,
        data_layout: &GpuTexelCopyBufferLayout,
        size: &GpuExtent3dDict,
    ) -> Result<(), JsValue>;
}
