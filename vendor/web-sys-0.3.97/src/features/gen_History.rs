#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "History",
        typescript_type = "History"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `History` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub type History;
    #[wasm_bindgen(catch, method, getter, js_class = "History", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn length(this: &History) -> Result<u32, JsValue>;
    #[cfg(feature = "ScrollRestoration")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "History",
        js_name = "scrollRestoration"
    )]
    #[doc = "Getter for the `scrollRestoration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
    pub fn scroll_restoration(this: &History) -> Result<ScrollRestoration, JsValue>;
    #[cfg(feature = "ScrollRestoration")]
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "History",
        js_name = "scrollRestoration"
    )]
    #[doc = "Setter for the `scrollRestoration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
    pub fn set_scroll_restoration(this: &History, value: ScrollRestoration) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "History", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn state(this: &History) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History")]
    #[doc = "The `back()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/back)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn back(this: &History) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History")]
    #[doc = "The `forward()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/forward)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn forward(this: &History) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History")]
    #[doc = "The `go()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn go(this: &History) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History", js_name = "go")]
    #[doc = "The `go()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn go_with_delta(this: &History, delta: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History", js_name = "pushState")]
    #[doc = "The `pushState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn push_state(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History", js_name = "pushState")]
    #[doc = "The `pushState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn push_state_with_url(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History", js_name = "replaceState")]
    #[doc = "The `replaceState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn replace_state(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "History", js_name = "replaceState")]
    #[doc = "The `replaceState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
    pub fn replace_state_with_url(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), JsValue>;
}
