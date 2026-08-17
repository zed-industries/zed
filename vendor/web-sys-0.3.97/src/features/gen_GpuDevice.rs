#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "GPUDevice",
        typescript_type = "GPUDevice"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuDevice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuDevice;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedFeatures")]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "features")]
    #[doc = "Getter for the `features` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/features)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn features(this: &GpuDevice) -> GpuSupportedFeatures;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedLimits")]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "limits")]
    #[doc = "Getter for the `limits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/limits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn limits(this: &GpuDevice) -> GpuSupportedLimits;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAdapterInfo")]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "adapterInfo")]
    #[doc = "Getter for the `adapterInfo` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/adapterInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn adapter_info(this: &GpuDevice) -> GpuAdapterInfo;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQueue")]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "queue")]
    #[doc = "Getter for the `queue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/queue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn queue(this: &GpuDevice) -> GpuQueue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDeviceLostInfo")]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "lost")]
    #[doc = "Getter for the `lost` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/lost)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuDeviceLostInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn lost(this: &GpuDevice) -> ::js_sys::Promise<GpuDeviceLostInfo>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "onuncapturederror")]
    #[doc = "Getter for the `onuncapturederror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onuncapturederror(this: &GpuDevice) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "GPUDevice", js_name = "onuncapturederror")]
    #[doc = "Setter for the `onuncapturederror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onuncapturederror(this: &GpuDevice, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUDevice", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuDevice) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "GPUDevice", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuDevice, value: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBindGroup", feature = "GpuBindGroupDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createBindGroup")]
    #[doc = "The `createBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuBindGroupDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_bind_group(this: &GpuDevice, descriptor: &GpuBindGroupDescriptor)
        -> GpuBindGroup;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuBindGroupLayout",
        feature = "GpuBindGroupLayoutDescriptor",
    ))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUDevice",
        js_name = "createBindGroupLayout"
    )]
    #[doc = "The `createBindGroupLayout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroupLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuBindGroupLayoutDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_bind_group_layout(
        this: &GpuDevice,
        descriptor: &GpuBindGroupLayoutDescriptor,
    ) -> Result<GpuBindGroupLayout, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuBufferDescriptor",))]
    #[wasm_bindgen(catch, method, js_class = "GPUDevice", js_name = "createBuffer")]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_buffer(
        this: &GpuDevice,
        descriptor: &GpuBufferDescriptor,
    ) -> Result<GpuBuffer, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandEncoder")]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createCommandEncoder")]
    #[doc = "The `createCommandEncoder()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_command_encoder(this: &GpuDevice) -> GpuCommandEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuCommandEncoder", feature = "GpuCommandEncoderDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createCommandEncoder")]
    #[doc = "The `createCommandEncoder()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuCommandEncoderDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_command_encoder_with_descriptor(
        this: &GpuDevice,
        descriptor: &GpuCommandEncoderDescriptor,
    ) -> GpuCommandEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePipeline",
        feature = "GpuComputePipelineDescriptor",
    ))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createComputePipeline")]
    #[doc = "The `createComputePipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipeline`, `GpuComputePipelineDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_compute_pipeline(
        this: &GpuDevice,
        descriptor: &GpuComputePipelineDescriptor,
    ) -> GpuComputePipeline;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePipeline",
        feature = "GpuComputePipelineDescriptor",
    ))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createComputePipelineAsync")]
    #[doc = "The `createComputePipelineAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipelineAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipeline`, `GpuComputePipelineDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_compute_pipeline_async(
        this: &GpuDevice,
        descriptor: &GpuComputePipelineDescriptor,
    ) -> ::js_sys::Promise<GpuComputePipeline>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuPipelineLayout", feature = "GpuPipelineLayoutDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createPipelineLayout")]
    #[doc = "The `createPipelineLayout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuPipelineLayout`, `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_pipeline_layout(
        this: &GpuDevice,
        descriptor: &GpuPipelineLayoutDescriptor,
    ) -> GpuPipelineLayout;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuQuerySet", feature = "GpuQuerySetDescriptor",))]
    #[wasm_bindgen(catch, method, js_class = "GPUDevice", js_name = "createQuerySet")]
    #[doc = "The `createQuerySet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createQuerySet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuQuerySet`, `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_query_set(
        this: &GpuDevice,
        descriptor: &GpuQuerySetDescriptor,
    ) -> Result<GpuQuerySet, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuRenderBundleEncoder",
        feature = "GpuRenderBundleEncoderDescriptor",
    ))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUDevice",
        js_name = "createRenderBundleEncoder"
    )]
    #[doc = "The `createRenderBundleEncoder()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderBundleEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderBundleEncoder`, `GpuRenderBundleEncoderDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_render_bundle_encoder(
        this: &GpuDevice,
        descriptor: &GpuRenderBundleEncoderDescriptor,
    ) -> Result<GpuRenderBundleEncoder, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPipeline", feature = "GpuRenderPipelineDescriptor",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUDevice",
        js_name = "createRenderPipeline"
    )]
    #[doc = "The `createRenderPipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderPipeline`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_render_pipeline(
        this: &GpuDevice,
        descriptor: &GpuRenderPipelineDescriptor,
    ) -> Result<GpuRenderPipeline, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPipeline", feature = "GpuRenderPipelineDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createRenderPipelineAsync")]
    #[doc = "The `createRenderPipelineAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipelineAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderPipeline`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_render_pipeline_async(
        this: &GpuDevice,
        descriptor: &GpuRenderPipelineDescriptor,
    ) -> ::js_sys::Promise<GpuRenderPipeline>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSampler")]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createSampler")]
    #[doc = "The `createSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_sampler(this: &GpuDevice) -> GpuSampler;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuSampler", feature = "GpuSamplerDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createSampler")]
    #[doc = "The `createSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_sampler_with_descriptor(
        this: &GpuDevice,
        descriptor: &GpuSamplerDescriptor,
    ) -> GpuSampler;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuShaderModule", feature = "GpuShaderModuleDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "createShaderModule")]
    #[doc = "The `createShaderModule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createShaderModule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuShaderModule`, `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_shader_module(
        this: &GpuDevice,
        descriptor: &GpuShaderModuleDescriptor,
    ) -> GpuShaderModule;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuTexture", feature = "GpuTextureDescriptor",))]
    #[wasm_bindgen(catch, method, js_class = "GPUDevice", js_name = "createTexture")]
    #[doc = "The `createTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuTexture`, `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_texture(
        this: &GpuDevice,
        descriptor: &GpuTextureDescriptor,
    ) -> Result<GpuTexture, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUDevice")]
    #[doc = "The `destroy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/destroy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn destroy(this: &GpuDevice);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExternalTexture",
        feature = "GpuExternalTextureDescriptor",
    ))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUDevice",
        js_name = "importExternalTexture"
    )]
    #[doc = "The `importExternalTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/importExternalTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuExternalTexture`, `GpuExternalTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn import_external_texture(
        this: &GpuDevice,
        descriptor: &GpuExternalTextureDescriptor,
    ) -> Result<GpuExternalTexture, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuError")]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "popErrorScope")]
    #[doc = "The `popErrorScope()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/popErrorScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_error_scope(this: &GpuDevice) -> ::js_sys::Promise<::js_sys::JsOption<GpuError>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuErrorFilter")]
    #[wasm_bindgen(method, js_class = "GPUDevice", js_name = "pushErrorScope")]
    #[doc = "The `pushErrorScope()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/pushErrorScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuErrorFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_error_scope(this: &GpuDevice, filter: GpuErrorFilter);
}
