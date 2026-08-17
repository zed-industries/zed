#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Worker",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ChromeWorker",
        typescript_type = "ChromeWorker"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChromeWorker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChromeWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeWorker`*"]
    pub type ChromeWorker;
    #[wasm_bindgen(catch, constructor, js_class = "ChromeWorker")]
    #[doc = "The `new ChromeWorker(..)` constructor, creating a new instance of `ChromeWorker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChromeWorker/ChromeWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeWorker`*"]
    pub fn new(script_url: &str) -> Result<ChromeWorker, JsValue>;
}
