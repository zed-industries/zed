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
        js_name = "XRInputSourceEvent",
        typescript_type = "XRInputSourceEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrInputSourceEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrInputSourceEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrFrame")]
    #[wasm_bindgen(method, getter, js_class = "XRInputSourceEvent", js_name = "frame")]
    #[doc = "Getter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrInputSourceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn frame(this: &XrInputSourceEvent) -> XrFrame;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSource")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRInputSourceEvent",
        js_name = "inputSource"
    )]
    #[doc = "Getter for the `inputSource` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/inputSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrInputSourceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn input_source(this: &XrInputSourceEvent) -> XrInputSource;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSourceEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "XRInputSourceEvent")]
    #[doc = "The `new XrInputSourceEvent(..)` constructor, creating a new instance of `XrInputSourceEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/XRInputSourceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourceEvent`, `XrInputSourceEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &XrInputSourceEventInit,
    ) -> Result<XrInputSourceEvent, JsValue>;
}
