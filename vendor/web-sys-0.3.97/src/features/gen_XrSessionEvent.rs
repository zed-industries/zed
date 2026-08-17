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
        js_name = "XRSessionEvent",
        typescript_type = "XRSessionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrSessionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrSessionEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[wasm_bindgen(method, getter, js_class = "XRSessionEvent", js_name = "session")]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSession`, `XrSessionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn session(this: &XrSessionEvent) -> XrSession;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "XRSessionEvent")]
    #[doc = "The `new XrSessionEvent(..)` constructor, creating a new instance of `XrSessionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent/XRSessionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionEvent`, `XrSessionEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &XrSessionEventInit,
    ) -> Result<XrSessionEvent, JsValue>;
}
