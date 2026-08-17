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
        js_name = "XRInputSourcesChangeEvent",
        typescript_type = "XRInputSourcesChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrInputSourcesChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrInputSourcesChangeEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRInputSourcesChangeEvent",
        js_name = "session"
    )]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn session(this: &XrInputSourcesChangeEvent) -> XrSession;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSource")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRInputSourcesChangeEvent",
        js_name = "added"
    )]
    #[doc = "Getter for the `added` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/added)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrInputSourcesChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn added(this: &XrInputSourcesChangeEvent) -> ::js_sys::Array<XrInputSource>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSource")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRInputSourcesChangeEvent",
        js_name = "removed"
    )]
    #[doc = "Getter for the `removed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/removed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrInputSourcesChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn removed(this: &XrInputSourcesChangeEvent) -> ::js_sys::Array<XrInputSource>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSourcesChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "XRInputSourcesChangeEvent")]
    #[doc = "The `new XrInputSourcesChangeEvent(..)` constructor, creating a new instance of `XrInputSourcesChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/XRInputSourcesChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`, `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &XrInputSourcesChangeEventInit,
    ) -> Result<XrInputSourcesChangeEvent, JsValue>;
}
