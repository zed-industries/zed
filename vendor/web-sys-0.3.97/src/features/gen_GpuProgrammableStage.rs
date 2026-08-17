#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUProgrammableStage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuProgrammableStage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuProgrammableStage;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `constants` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "constants")]
    pub fn get_constants(this: &GpuProgrammableStage)
        -> Option<::js_sys::Object<::js_sys::Number>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `constants` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "constants")]
    pub fn set_constants(this: &GpuProgrammableStage, val: &::js_sys::Object<::js_sys::Number>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "entryPoint")]
    pub fn get_entry_point(this: &GpuProgrammableStage) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "entryPoint")]
    pub fn set_entry_point(this: &GpuProgrammableStage, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Get the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "module")]
    pub fn get_module(this: &GpuProgrammableStage) -> GpuShaderModule;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "module")]
    pub fn set_module(this: &GpuProgrammableStage, val: &GpuShaderModule);
}
#[cfg(web_sys_unstable_apis)]
impl GpuProgrammableStage {
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Construct a new `GpuProgrammableStage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuProgrammableStage`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(module: &GpuShaderModule) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_module(module);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_constants()` instead."]
    pub fn constants(&mut self, val: &::js_sys::Object<::js_sys::Number>) -> &mut Self {
        self.set_constants(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_entry_point()` instead."]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        self.set_entry_point(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[deprecated = "Use `set_module()` instead."]
    pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self {
        self.set_module(val);
        self
    }
}
