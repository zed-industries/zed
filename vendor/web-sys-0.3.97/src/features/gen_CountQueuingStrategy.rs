#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CountQueuingStrategy",
        typescript_type = "CountQueuingStrategy"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CountQueuingStrategy` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`*"]
    pub type CountQueuingStrategy;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CountQueuingStrategy",
        js_name = "highWaterMark"
    )]
    #[doc = "Getter for the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/highWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`*"]
    pub fn high_water_mark(this: &CountQueuingStrategy) -> f64;
    #[wasm_bindgen(method, getter, js_class = "CountQueuingStrategy", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`*"]
    pub fn size(this: &CountQueuingStrategy) -> ::js_sys::Function;
    #[cfg(feature = "QueuingStrategyInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CountQueuingStrategy")]
    #[doc = "The `new CountQueuingStrategy(..)` constructor, creating a new instance of `CountQueuingStrategy`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/CountQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`, `QueuingStrategyInit`*"]
    pub fn new(init: &QueuingStrategyInit) -> Result<CountQueuingStrategy, JsValue>;
}
