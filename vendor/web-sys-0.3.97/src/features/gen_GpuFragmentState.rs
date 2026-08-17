#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUFragmentState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuFragmentState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuFragmentState;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `constants` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "constants")]
    pub fn get_constants(this: &GpuFragmentState) -> Option<::js_sys::Object<::js_sys::Number>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `constants` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "constants")]
    pub fn set_constants(this: &GpuFragmentState, val: &::js_sys::Object<::js_sys::Number>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "entryPoint")]
    pub fn get_entry_point(this: &GpuFragmentState) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "entryPoint")]
    pub fn set_entry_point(this: &GpuFragmentState, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Get the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "module")]
    pub fn get_module(this: &GpuFragmentState) -> GpuShaderModule;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "module")]
    pub fn set_module(this: &GpuFragmentState, val: &GpuShaderModule);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuColorTargetState")]
    #[doc = "Get the `targets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "targets")]
    pub fn get_targets(
        this: &GpuFragmentState,
    ) -> ::js_sys::Array<::js_sys::JsOption<GpuColorTargetState>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuColorTargetState")]
    #[doc = "Change the `targets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "targets")]
    pub fn set_targets(this: &GpuFragmentState, val: &[::js_sys::JsOption<GpuColorTargetState>]);
}
#[cfg(web_sys_unstable_apis)]
impl GpuFragmentState {
    #[cfg(all(feature = "GpuColorTargetState", feature = "GpuShaderModule",))]
    #[doc = "Construct a new `GpuFragmentState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        module: &GpuShaderModule,
        targets: &[::js_sys::JsOption<GpuColorTargetState>],
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_module(module);
        ret.set_targets(targets);
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
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuColorTargetState")]
    #[deprecated = "Use `set_targets()` instead."]
    pub fn targets(&mut self, val: &[::js_sys::JsOption<GpuColorTargetState>]) -> &mut Self {
        self.set_targets(val);
        self
    }
}
