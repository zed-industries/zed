#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUSupportedLimits",
        typescript_type = "GPUSupportedLimits"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSupportedLimits` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuSupportedLimits;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxTextureDimension1D"
    )]
    #[doc = "Getter for the `maxTextureDimension1D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension1D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_dimension_1d(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxTextureDimension2D"
    )]
    #[doc = "Getter for the `maxTextureDimension2D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_dimension_2d(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxTextureDimension3D"
    )]
    #[doc = "Getter for the `maxTextureDimension3D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_dimension_3d(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxTextureArrayLayers"
    )]
    #[doc = "Getter for the `maxTextureArrayLayers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureArrayLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_array_layers(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxBindGroups"
    )]
    #[doc = "Getter for the `maxBindGroups` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindGroups)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_bind_groups(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxBindGroupsPlusVertexBuffers"
    )]
    #[doc = "Getter for the `maxBindGroupsPlusVertexBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindGroupsPlusVertexBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_bind_groups_plus_vertex_buffers(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxBindingsPerBindGroup"
    )]
    #[doc = "Getter for the `maxBindingsPerBindGroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindingsPerBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_bindings_per_bind_group(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxDynamicUniformBuffersPerPipelineLayout"
    )]
    #[doc = "Getter for the `maxDynamicUniformBuffersPerPipelineLayout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxDynamicUniformBuffersPerPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_dynamic_uniform_buffers_per_pipeline_layout(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxDynamicStorageBuffersPerPipelineLayout"
    )]
    #[doc = "Getter for the `maxDynamicStorageBuffersPerPipelineLayout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxDynamicStorageBuffersPerPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_dynamic_storage_buffers_per_pipeline_layout(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxSampledTexturesPerShaderStage"
    )]
    #[doc = "Getter for the `maxSampledTexturesPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxSampledTexturesPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_sampled_textures_per_shader_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxSamplersPerShaderStage"
    )]
    #[doc = "Getter for the `maxSamplersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxSamplersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_samplers_per_shader_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageBuffersPerShaderStage"
    )]
    #[doc = "Getter for the `maxStorageBuffersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBuffersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_buffers_per_shader_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageBuffersInVertexStage"
    )]
    #[doc = "Getter for the `maxStorageBuffersInVertexStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBuffersInVertexStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_buffers_in_vertex_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageBuffersInFragmentStage"
    )]
    #[doc = "Getter for the `maxStorageBuffersInFragmentStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBuffersInFragmentStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_buffers_in_fragment_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageTexturesPerShaderStage"
    )]
    #[doc = "Getter for the `maxStorageTexturesPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageTexturesPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_textures_per_shader_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageTexturesInVertexStage"
    )]
    #[doc = "Getter for the `maxStorageTexturesInVertexStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageTexturesInVertexStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_textures_in_vertex_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageTexturesInFragmentStage"
    )]
    #[doc = "Getter for the `maxStorageTexturesInFragmentStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageTexturesInFragmentStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_textures_in_fragment_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxUniformBuffersPerShaderStage"
    )]
    #[doc = "Getter for the `maxUniformBuffersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxUniformBuffersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_uniform_buffers_per_shader_stage(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxUniformBufferBindingSize"
    )]
    #[doc = "Getter for the `maxUniformBufferBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxUniformBufferBindingSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_uniform_buffer_binding_size(this: &GpuSupportedLimits) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxStorageBufferBindingSize"
    )]
    #[doc = "Getter for the `maxStorageBufferBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBufferBindingSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_buffer_binding_size(this: &GpuSupportedLimits) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "minUniformBufferOffsetAlignment"
    )]
    #[doc = "Getter for the `minUniformBufferOffsetAlignment` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/minUniformBufferOffsetAlignment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min_uniform_buffer_offset_alignment(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "minStorageBufferOffsetAlignment"
    )]
    #[doc = "Getter for the `minStorageBufferOffsetAlignment` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/minStorageBufferOffsetAlignment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min_storage_buffer_offset_alignment(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxVertexBuffers"
    )]
    #[doc = "Getter for the `maxVertexBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_vertex_buffers(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxBufferSize"
    )]
    #[doc = "Getter for the `maxBufferSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBufferSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_buffer_size(this: &GpuSupportedLimits) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxVertexAttributes"
    )]
    #[doc = "Getter for the `maxVertexAttributes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_vertex_attributes(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxVertexBufferArrayStride"
    )]
    #[doc = "Getter for the `maxVertexBufferArrayStride` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexBufferArrayStride)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_vertex_buffer_array_stride(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxInterStageShaderVariables"
    )]
    #[doc = "Getter for the `maxInterStageShaderVariables` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxInterStageShaderVariables)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_inter_stage_shader_variables(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxColorAttachments"
    )]
    #[doc = "Getter for the `maxColorAttachments` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxColorAttachments)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_color_attachments(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxColorAttachmentBytesPerSample"
    )]
    #[doc = "Getter for the `maxColorAttachmentBytesPerSample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxColorAttachmentBytesPerSample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_color_attachment_bytes_per_sample(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxComputeWorkgroupStorageSize"
    )]
    #[doc = "Getter for the `maxComputeWorkgroupStorageSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupStorageSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_compute_workgroup_storage_size(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxComputeInvocationsPerWorkgroup"
    )]
    #[doc = "Getter for the `maxComputeInvocationsPerWorkgroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeInvocationsPerWorkgroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_compute_invocations_per_workgroup(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxComputeWorkgroupSizeX"
    )]
    #[doc = "Getter for the `maxComputeWorkgroupSizeX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_compute_workgroup_size_x(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxComputeWorkgroupSizeY"
    )]
    #[doc = "Getter for the `maxComputeWorkgroupSizeY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_compute_workgroup_size_y(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxComputeWorkgroupSizeZ"
    )]
    #[doc = "Getter for the `maxComputeWorkgroupSizeZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_compute_workgroup_size_z(this: &GpuSupportedLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GPUSupportedLimits",
        js_name = "maxComputeWorkgroupsPerDimension"
    )]
    #[doc = "Getter for the `maxComputeWorkgroupsPerDimension` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupsPerDimension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_compute_workgroups_per_dimension(this: &GpuSupportedLimits) -> u32;
}
