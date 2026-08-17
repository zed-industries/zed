#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ByteLengthQueuingStrategy",
        typescript_type = "ByteLengthQueuingStrategy"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ByteLengthQueuingStrategy` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`*"]
    pub type ByteLengthQueuingStrategy;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ByteLengthQueuingStrategy",
        js_name = "highWaterMark"
    )]
    #[doc = "Getter for the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/highWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`*"]
    pub fn high_water_mark(this: &ByteLengthQueuingStrategy) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ByteLengthQueuingStrategy",
        js_name = "size"
    )]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`*"]
    pub fn size(this: &ByteLengthQueuingStrategy) -> ::js_sys::Function;
    #[cfg(feature = "QueuingStrategyInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ByteLengthQueuingStrategy")]
    #[doc = "The `new ByteLengthQueuingStrategy(..)` constructor, creating a new instance of `ByteLengthQueuingStrategy`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/ByteLengthQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`, `QueuingStrategyInit`*"]
    pub fn new(init: &QueuingStrategyInit) -> Result<ByteLengthQueuingStrategy, JsValue>;
}
