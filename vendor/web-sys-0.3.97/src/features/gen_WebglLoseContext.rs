#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "WEBGL_lose_context" , typescript_type = "WEBGL_lose_context")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglLoseContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    pub type WebglLoseContext;
    #[wasm_bindgen(method, js_class = "WEBGL_lose_context", js_name = "loseContext")]
    #[doc = "The `loseContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/loseContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    pub fn lose_context(this: &WebglLoseContext);
    #[wasm_bindgen(method, js_class = "WEBGL_lose_context", js_name = "restoreContext")]
    #[doc = "The `restoreContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/restoreContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    pub fn restore_context(this: &WebglLoseContext);
}
