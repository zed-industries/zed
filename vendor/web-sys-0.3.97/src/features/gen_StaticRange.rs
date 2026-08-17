#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AbstractRange",
        extends = "::js_sys::Object",
        js_name = "StaticRange",
        typescript_type = "StaticRange"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StaticRange` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRange`*"]
    pub type StaticRange;
    #[cfg(feature = "StaticRangeInit")]
    #[wasm_bindgen(catch, constructor, js_class = "StaticRange")]
    #[doc = "The `new StaticRange(..)` constructor, creating a new instance of `StaticRange`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StaticRange/StaticRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRange`, `StaticRangeInit`*"]
    pub fn new(init: &StaticRangeInit) -> Result<StaticRange, JsValue>;
}
