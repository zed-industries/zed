#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPURenderPipelineDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPipelineDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPipelineDescriptor;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &GpuRenderPipelineDescriptor) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &GpuRenderPipelineDescriptor, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineLayout")]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "layout")]
    pub fn get_layout(this: &GpuRenderPipelineDescriptor) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineLayout")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout(this: &GpuRenderPipelineDescriptor, val: &GpuPipelineLayout);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAutoLayoutMode")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAutoLayoutMode`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout_gpu_auto_layout_mode(
        this: &GpuRenderPipelineDescriptor,
        val: GpuAutoLayoutMode,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[doc = "Get the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthStencil")]
    pub fn get_depth_stencil(this: &GpuRenderPipelineDescriptor) -> Option<GpuDepthStencilState>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[doc = "Change the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthStencil")]
    pub fn set_depth_stencil(this: &GpuRenderPipelineDescriptor, val: &GpuDepthStencilState);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[doc = "Get the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "fragment")]
    pub fn get_fragment(this: &GpuRenderPipelineDescriptor) -> Option<GpuFragmentState>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[doc = "Change the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "fragment")]
    pub fn set_fragment(this: &GpuRenderPipelineDescriptor, val: &GpuFragmentState);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[doc = "Get the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "multisample")]
    pub fn get_multisample(this: &GpuRenderPipelineDescriptor) -> Option<GpuMultisampleState>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[doc = "Change the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "multisample")]
    pub fn set_multisample(this: &GpuRenderPipelineDescriptor, val: &GpuMultisampleState);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[doc = "Get the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "primitive")]
    pub fn get_primitive(this: &GpuRenderPipelineDescriptor) -> Option<GpuPrimitiveState>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[doc = "Change the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "primitive")]
    pub fn set_primitive(this: &GpuRenderPipelineDescriptor, val: &GpuPrimitiveState);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Get the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "vertex")]
    pub fn get_vertex(this: &GpuRenderPipelineDescriptor) -> GpuVertexState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Change the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "vertex")]
    pub fn set_vertex(this: &GpuRenderPipelineDescriptor, val: &GpuVertexState);
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPipelineDescriptor {
    #[cfg(all(feature = "GpuPipelineLayout", feature = "GpuVertexState",))]
    #[doc = "Construct a new `GpuRenderPipelineDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(layout: &GpuPipelineLayout, vertex: &GpuVertexState) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_layout(layout);
        ret.set_vertex(vertex);
        ret
    }
    #[cfg(all(feature = "GpuAutoLayoutMode", feature = "GpuVertexState",))]
    #[doc = "Construct a new `GpuRenderPipelineDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAutoLayoutMode`, `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_gpu_auto_layout_mode(
        layout: GpuAutoLayoutMode,
        vertex: &GpuVertexState,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_layout_gpu_auto_layout_mode(layout);
        ret.set_vertex(vertex);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineLayout")]
    #[deprecated = "Use `set_layout()` instead."]
    pub fn layout(&mut self, val: &GpuPipelineLayout) -> &mut Self {
        self.set_layout(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[deprecated = "Use `set_depth_stencil()` instead."]
    pub fn depth_stencil(&mut self, val: &GpuDepthStencilState) -> &mut Self {
        self.set_depth_stencil(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[deprecated = "Use `set_fragment()` instead."]
    pub fn fragment(&mut self, val: &GpuFragmentState) -> &mut Self {
        self.set_fragment(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[deprecated = "Use `set_multisample()` instead."]
    pub fn multisample(&mut self, val: &GpuMultisampleState) -> &mut Self {
        self.set_multisample(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[deprecated = "Use `set_primitive()` instead."]
    pub fn primitive(&mut self, val: &GpuPrimitiveState) -> &mut Self {
        self.set_primitive(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[deprecated = "Use `set_vertex()` instead."]
    pub fn vertex(&mut self, val: &GpuVertexState) -> &mut Self {
        self.set_vertex(val);
        self
    }
}
