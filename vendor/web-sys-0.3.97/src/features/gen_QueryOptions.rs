#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "QueryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueryOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type QueryOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `postscriptNames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueryOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "postscriptNames")]
    pub fn get_postscript_names(this: &QueryOptions)
        -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `postscriptNames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueryOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "postscriptNames")]
    pub fn set_postscript_names(this: &QueryOptions, val: &[::js_sys::JsString]);
}
#[cfg(web_sys_unstable_apis)]
impl QueryOptions {
    #[doc = "Construct a new `QueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueryOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_postscript_names()` instead."]
    pub fn postscript_names(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_postscript_names(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for QueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
