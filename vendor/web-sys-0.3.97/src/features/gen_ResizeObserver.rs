#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ResizeObserver",
        typescript_type = "ResizeObserver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserver`*"]
    pub type ResizeObserver;
    #[wasm_bindgen(catch, constructor, js_class = "ResizeObserver")]
    #[doc = "The `new ResizeObserver(..)` constructor, creating a new instance of `ResizeObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/ResizeObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserver`*"]
    pub fn new(callback: &::js_sys::Function) -> Result<ResizeObserver, JsValue>;
    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserver`*"]
    pub fn disconnect(this: &ResizeObserver);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserver`*"]
    pub fn observe(this: &ResizeObserver, target: &Element);
    #[cfg(all(feature = "Element", feature = "ResizeObserverOptions",))]
    #[wasm_bindgen(method, js_class = "ResizeObserver", js_name = "observe")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserver`, `ResizeObserverOptions`*"]
    pub fn observe_with_options(
        this: &ResizeObserver,
        target: &Element,
        options: &ResizeObserverOptions,
    );
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "ResizeObserver")]
    #[doc = "The `unobserve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/unobserve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserver`*"]
    pub fn unobserve(this: &ResizeObserver, target: &Element);
}
