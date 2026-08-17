#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "WEBGL_debug_shaders" , typescript_type = "WEBGL_debug_shaders")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglDebugShaders` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDebugShaders`*"]
    pub type WebglDebugShaders;
    #[cfg(feature = "WebGlShader")]
    #[wasm_bindgen(
        method,
        js_class = "WEBGL_debug_shaders",
        js_name = "getTranslatedShaderSource"
    )]
    #[doc = "The `getTranslatedShaderSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders/getTranslatedShaderSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShader`, `WebglDebugShaders`*"]
    pub fn get_translated_shader_source(
        this: &WebglDebugShaders,
        shader: &WebGlShader,
    ) -> ::alloc::string::String;
}
