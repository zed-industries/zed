#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HighlightHitResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HighlightHitResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightHitResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HighlightHitResult;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[doc = "Get the `highlight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightHitResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "highlight")]
    pub fn get_highlight(this: &HighlightHitResult) -> Option<Highlight>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[doc = "Change the `highlight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightHitResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "highlight")]
    pub fn set_highlight(this: &HighlightHitResult, val: &Highlight);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbstractRange")]
    #[doc = "Get the `ranges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`, `HighlightHitResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "ranges")]
    pub fn get_ranges(this: &HighlightHitResult) -> Option<::js_sys::Array<AbstractRange>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbstractRange")]
    #[doc = "Change the `ranges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`, `HighlightHitResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "ranges")]
    pub fn set_ranges(this: &HighlightHitResult, val: &[AbstractRange]);
}
#[cfg(web_sys_unstable_apis)]
impl HighlightHitResult {
    #[doc = "Construct a new `HighlightHitResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightHitResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[deprecated = "Use `set_highlight()` instead."]
    pub fn highlight(&mut self, val: &Highlight) -> &mut Self {
        self.set_highlight(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbstractRange")]
    #[deprecated = "Use `set_ranges()` instead."]
    pub fn ranges(&mut self, val: &[AbstractRange]) -> &mut Self {
        self.set_ranges(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HighlightHitResult {
    fn default() -> Self {
        Self::new()
    }
}
