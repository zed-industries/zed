#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Client",
        extends = "::js_sys::Object",
        js_name = "WindowClient",
        typescript_type = "WindowClient"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WindowClient` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
    pub type WindowClient;
    #[cfg(feature = "VisibilityState")]
    #[wasm_bindgen(method, getter, js_class = "WindowClient", js_name = "visibilityState")]
    #[doc = "Getter for the `visibilityState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/visibilityState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisibilityState`, `WindowClient`*"]
    pub fn visibility_state(this: &WindowClient) -> VisibilityState;
    #[wasm_bindgen(method, getter, js_class = "WindowClient", js_name = "focused")]
    #[doc = "Getter for the `focused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
    pub fn focused(this: &WindowClient) -> bool;
    #[wasm_bindgen(catch, method, js_class = "WindowClient")]
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
    pub fn focus(this: &WindowClient) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WindowClient")]
    #[doc = "The `navigate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/navigate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
    pub fn navigate(this: &WindowClient, url: &str) -> Result<::js_sys::Promise, JsValue>;
}
