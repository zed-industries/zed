#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUShaderModuleCompilationHint"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuShaderModuleCompilationHint` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuShaderModuleCompilationHint;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "entryPoint")]
    pub fn get_entry_point(this: &GpuShaderModuleCompilationHint) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "entryPoint")]
    pub fn set_entry_point(this: &GpuShaderModuleCompilationHint, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineLayout")]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "layout")]
    pub fn get_layout(this: &GpuShaderModuleCompilationHint) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineLayout")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineLayout`, `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout(this: &GpuShaderModuleCompilationHint, val: &GpuPipelineLayout);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAutoLayoutMode")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAutoLayoutMode`, `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout_gpu_auto_layout_mode(
        this: &GpuShaderModuleCompilationHint,
        val: GpuAutoLayoutMode,
    );
}
#[cfg(web_sys_unstable_apis)]
impl GpuShaderModuleCompilationHint {
    #[doc = "Construct a new `GpuShaderModuleCompilationHint`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(entry_point: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_entry_point(entry_point);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_entry_point()` instead."]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        self.set_entry_point(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineLayout")]
    #[deprecated = "Use `set_layout()` instead."]
    pub fn layout(&mut self, val: &GpuPipelineLayout) -> &mut Self {
        self.set_layout(val);
        self
    }
}
