#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SFrameTransformOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransformOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SFrameTransformOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformRole")]
    #[doc = "Get the `role` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`, `SFrameTransformRole`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "role")]
    pub fn get_role(this: &SFrameTransformOptions) -> Option<SFrameTransformRole>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformRole")]
    #[doc = "Change the `role` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`, `SFrameTransformRole`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "role")]
    pub fn set_role(this: &SFrameTransformOptions, val: SFrameTransformRole);
}
#[cfg(web_sys_unstable_apis)]
impl SFrameTransformOptions {
    #[doc = "Construct a new `SFrameTransformOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformRole")]
    #[deprecated = "Use `set_role()` instead."]
    pub fn role(&mut self, val: SFrameTransformRole) -> &mut Self {
        self.set_role(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SFrameTransformOptions {
    fn default() -> Self {
        Self::new()
    }
}
