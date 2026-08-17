#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TogglePopoverOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TogglePopoverOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TogglePopoverOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type TogglePopoverOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlElement")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `TogglePopoverOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &TogglePopoverOptions) -> Option<HtmlElement>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `TogglePopoverOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &TogglePopoverOptions, val: &HtmlElement);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TogglePopoverOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "force")]
    pub fn get_force(this: &TogglePopoverOptions) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TogglePopoverOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "force")]
    pub fn set_force(this: &TogglePopoverOptions, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl TogglePopoverOptions {
    #[doc = "Construct a new `TogglePopoverOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TogglePopoverOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlElement")]
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: &HtmlElement) -> &mut Self {
        self.set_source(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_force()` instead."]
    pub fn force(&mut self, val: bool) -> &mut Self {
        self.set_force(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for TogglePopoverOptions {
    fn default() -> Self {
        Self::new()
    }
}
