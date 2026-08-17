#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPURenderPassDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassDescriptor;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &GpuRenderPassDescriptor) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &GpuRenderPassDescriptor, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassColorAttachment")]
    #[doc = "Get the `colorAttachments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "colorAttachments")]
    pub fn get_color_attachments(
        this: &GpuRenderPassDescriptor,
    ) -> ::js_sys::Array<::js_sys::JsOption<GpuRenderPassColorAttachment>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassColorAttachment")]
    #[doc = "Change the `colorAttachments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "colorAttachments")]
    pub fn set_color_attachments(
        this: &GpuRenderPassDescriptor,
        val: &[::js_sys::JsOption<GpuRenderPassColorAttachment>],
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[doc = "Get the `depthStencilAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthStencilAttachment")]
    pub fn get_depth_stencil_attachment(
        this: &GpuRenderPassDescriptor,
    ) -> Option<GpuRenderPassDepthStencilAttachment>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[doc = "Change the `depthStencilAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthStencilAttachment")]
    pub fn set_depth_stencil_attachment(
        this: &GpuRenderPassDescriptor,
        val: &GpuRenderPassDepthStencilAttachment,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `maxDrawCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "maxDrawCount")]
    pub fn get_max_draw_count(this: &GpuRenderPassDescriptor) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `maxDrawCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "maxDrawCount")]
    pub fn set_max_draw_count(this: &GpuRenderPassDescriptor, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `maxDrawCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "maxDrawCount")]
    pub fn set_max_draw_count_f64(this: &GpuRenderPassDescriptor, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Get the `occlusionQuerySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "occlusionQuerySet")]
    pub fn get_occlusion_query_set(this: &GpuRenderPassDescriptor) -> Option<GpuQuerySet>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Change the `occlusionQuerySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "occlusionQuerySet")]
    pub fn set_occlusion_query_set(this: &GpuRenderPassDescriptor, val: &GpuQuerySet);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[doc = "Get the `timestampWrites` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "timestampWrites")]
    pub fn get_timestamp_writes(
        this: &GpuRenderPassDescriptor,
    ) -> Option<GpuRenderPassTimestampWrites>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[doc = "Change the `timestampWrites` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "timestampWrites")]
    pub fn set_timestamp_writes(this: &GpuRenderPassDescriptor, val: &GpuRenderPassTimestampWrites);
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassDescriptor {
    #[cfg(feature = "GpuRenderPassColorAttachment")]
    #[doc = "Construct a new `GpuRenderPassDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(color_attachments: &[::js_sys::JsOption<GpuRenderPassColorAttachment>]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_color_attachments(color_attachments);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassColorAttachment")]
    #[deprecated = "Use `set_color_attachments()` instead."]
    pub fn color_attachments(
        &mut self,
        val: &[::js_sys::JsOption<GpuRenderPassColorAttachment>],
    ) -> &mut Self {
        self.set_color_attachments(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[deprecated = "Use `set_depth_stencil_attachment()` instead."]
    pub fn depth_stencil_attachment(
        &mut self,
        val: &GpuRenderPassDepthStencilAttachment,
    ) -> &mut Self {
        self.set_depth_stencil_attachment(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_max_draw_count()` instead."]
    pub fn max_draw_count(&mut self, val: u32) -> &mut Self {
        self.set_max_draw_count(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[deprecated = "Use `set_occlusion_query_set()` instead."]
    pub fn occlusion_query_set(&mut self, val: &GpuQuerySet) -> &mut Self {
        self.set_occlusion_query_set(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[deprecated = "Use `set_timestamp_writes()` instead."]
    pub fn timestamp_writes(&mut self, val: &GpuRenderPassTimestampWrites) -> &mut Self {
        self.set_timestamp_writes(val);
        self
    }
}
