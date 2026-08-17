#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WebGLContextAttributes")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextAttributes` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub type WebGlContextAttributes;
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "alpha")]
    pub fn set_alpha(this: &WebGlContextAttributes, val: bool);
    #[doc = "Get the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "antialias")]
    pub fn get_antialias(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "antialias")]
    pub fn set_antialias(this: &WebGlContextAttributes, val: bool);
    #[doc = "Get the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "depth")]
    pub fn get_depth(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "depth")]
    pub fn set_depth(this: &WebGlContextAttributes, val: bool);
    #[doc = "Get the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "failIfMajorPerformanceCaveat")]
    pub fn get_fail_if_major_performance_caveat(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "failIfMajorPerformanceCaveat")]
    pub fn set_fail_if_major_performance_caveat(this: &WebGlContextAttributes, val: bool);
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Get the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    #[wasm_bindgen(method, getter = "powerPreference")]
    pub fn get_power_preference(this: &WebGlContextAttributes) -> Option<WebGlPowerPreference>;
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    #[wasm_bindgen(method, setter = "powerPreference")]
    pub fn set_power_preference(this: &WebGlContextAttributes, val: WebGlPowerPreference);
    #[doc = "Get the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "premultipliedAlpha")]
    pub fn get_premultiplied_alpha(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "premultipliedAlpha")]
    pub fn set_premultiplied_alpha(this: &WebGlContextAttributes, val: bool);
    #[doc = "Get the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "preserveDrawingBuffer")]
    pub fn get_preserve_drawing_buffer(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "preserveDrawingBuffer")]
    pub fn set_preserve_drawing_buffer(this: &WebGlContextAttributes, val: bool);
    #[doc = "Get the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, getter = "stencil")]
    pub fn get_stencil(this: &WebGlContextAttributes) -> Option<bool>;
    #[doc = "Change the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[wasm_bindgen(method, setter = "stencil")]
    pub fn set_stencil(this: &WebGlContextAttributes, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "xrCompatible")]
    pub fn get_xr_compatible(this: &WebGlContextAttributes) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "xrCompatible")]
    pub fn set_xr_compatible(this: &WebGlContextAttributes, val: bool);
}
impl WebGlContextAttributes {
    #[doc = "Construct a new `WebGlContextAttributes`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_alpha()` instead."]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[deprecated = "Use `set_antialias()` instead."]
    pub fn antialias(&mut self, val: bool) -> &mut Self {
        self.set_antialias(val);
        self
    }
    #[deprecated = "Use `set_depth()` instead."]
    pub fn depth(&mut self, val: bool) -> &mut Self {
        self.set_depth(val);
        self
    }
    #[deprecated = "Use `set_fail_if_major_performance_caveat()` instead."]
    pub fn fail_if_major_performance_caveat(&mut self, val: bool) -> &mut Self {
        self.set_fail_if_major_performance_caveat(val);
        self
    }
    #[cfg(feature = "WebGlPowerPreference")]
    #[deprecated = "Use `set_power_preference()` instead."]
    pub fn power_preference(&mut self, val: WebGlPowerPreference) -> &mut Self {
        self.set_power_preference(val);
        self
    }
    #[deprecated = "Use `set_premultiplied_alpha()` instead."]
    pub fn premultiplied_alpha(&mut self, val: bool) -> &mut Self {
        self.set_premultiplied_alpha(val);
        self
    }
    #[deprecated = "Use `set_preserve_drawing_buffer()` instead."]
    pub fn preserve_drawing_buffer(&mut self, val: bool) -> &mut Self {
        self.set_preserve_drawing_buffer(val);
        self
    }
    #[deprecated = "Use `set_stencil()` instead."]
    pub fn stencil(&mut self, val: bool) -> &mut Self {
        self.set_stencil(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_xr_compatible()` instead."]
    pub fn xr_compatible(&mut self, val: bool) -> &mut Self {
        self.set_xr_compatible(val);
        self
    }
}
impl Default for WebGlContextAttributes {
    fn default() -> Self {
        Self::new()
    }
}
