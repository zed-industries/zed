#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FuzzingFunctions",
        typescript_type = "FuzzingFunctions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FuzzingFunctions` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub type FuzzingFunctions;
    #[wasm_bindgen(
        static_method_of = "FuzzingFunctions",
        js_class = "FuzzingFunctions",
        js_name = "cycleCollect"
    )]
    #[doc = "The `cycleCollect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/cycleCollect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub fn cycle_collect();
    #[wasm_bindgen(
        catch,
        static_method_of = "FuzzingFunctions",
        js_class = "FuzzingFunctions",
        js_name = "enableAccessibility"
    )]
    #[doc = "The `enableAccessibility()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/enableAccessibility_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub fn enable_accessibility() -> Result<(), JsValue>;
    #[wasm_bindgen(
        static_method_of = "FuzzingFunctions",
        js_class = "FuzzingFunctions",
        js_name = "garbageCollect"
    )]
    #[doc = "The `garbageCollect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/garbageCollect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
    pub fn garbage_collect();
}
