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
        js_name = "XRReferenceSpaceEvent",
        typescript_type = "XRReferenceSpaceEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrReferenceSpaceEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrReferenceSpaceEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrReferenceSpace")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRReferenceSpaceEvent",
        js_name = "referenceSpace"
    )]
    #[doc = "Getter for the `referenceSpace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/referenceSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpace`, `XrReferenceSpaceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reference_space(this: &XrReferenceSpaceEvent) -> XrReferenceSpace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrRigidTransform")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRReferenceSpaceEvent",
        js_name = "transform"
    )]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceEvent`, `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transform(this: &XrReferenceSpaceEvent) -> Option<XrRigidTransform>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrReferenceSpaceEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "XRReferenceSpaceEvent")]
    #[doc = "The `new XrReferenceSpaceEvent(..)` constructor, creating a new instance of `XrReferenceSpaceEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/XRReferenceSpaceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceEvent`, `XrReferenceSpaceEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &XrReferenceSpaceEventInit,
    ) -> Result<XrReferenceSpaceEvent, JsValue>;
}
