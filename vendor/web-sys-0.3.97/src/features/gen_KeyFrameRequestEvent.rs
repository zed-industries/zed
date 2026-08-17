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
        js_name = "KeyFrameRequestEvent",
        typescript_type = "KeyFrameRequestEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyFrameRequestEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type KeyFrameRequestEvent;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "KeyFrameRequestEvent", js_name = "rid")]
    #[doc = "Getter for the `rid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/rid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rid(this: &KeyFrameRequestEvent) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "KeyFrameRequestEvent")]
    #[doc = "The `new KeyFrameRequestEvent(..)` constructor, creating a new instance of `KeyFrameRequestEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/KeyFrameRequestEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(type_: &str) -> Result<KeyFrameRequestEvent, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "KeyFrameRequestEvent")]
    #[doc = "The `new KeyFrameRequestEvent(..)` constructor, creating a new instance of `KeyFrameRequestEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/KeyFrameRequestEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_rid(type_: &str, rid: &str) -> Result<KeyFrameRequestEvent, JsValue>;
}
