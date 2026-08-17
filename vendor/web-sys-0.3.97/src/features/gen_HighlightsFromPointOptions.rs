#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HighlightsFromPointOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HighlightsFromPointOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightsFromPointOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HighlightsFromPointOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ShadowRoot")]
    #[doc = "Get the `shadowRoots` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightsFromPointOptions`, `ShadowRoot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "shadowRoots")]
    pub fn get_shadow_roots(
        this: &HighlightsFromPointOptions,
    ) -> Option<::js_sys::Array<ShadowRoot>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ShadowRoot")]
    #[doc = "Change the `shadowRoots` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightsFromPointOptions`, `ShadowRoot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "shadowRoots")]
    pub fn set_shadow_roots(this: &HighlightsFromPointOptions, val: &[ShadowRoot]);
}
#[cfg(web_sys_unstable_apis)]
impl HighlightsFromPointOptions {
    #[doc = "Construct a new `HighlightsFromPointOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightsFromPointOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ShadowRoot")]
    #[deprecated = "Use `set_shadow_roots()` instead."]
    pub fn shadow_roots(&mut self, val: &[ShadowRoot]) -> &mut Self {
        self.set_shadow_roots(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HighlightsFromPointOptions {
    fn default() -> Self {
        Self::new()
    }
}
