#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ValueEvent",
        typescript_type = "ValueEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ValueEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValueEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValueEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ValueEvent;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ValueEvent", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValueEvent/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValueEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn value(this: &ValueEvent) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "ValueEvent")]
    #[doc = "The `new ValueEvent(..)` constructor, creating a new instance of `ValueEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValueEvent/ValueEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValueEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(type_: &str) -> Result<ValueEvent, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ValueEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ValueEvent")]
    #[doc = "The `new ValueEvent(..)` constructor, creating a new instance of `ValueEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValueEvent/ValueEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValueEvent`, `ValueEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_init_dict(
        type_: &str,
        init_dict: &ValueEventInit,
    ) -> Result<ValueEvent, JsValue>;
}
