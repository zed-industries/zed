#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Presentation",
        typescript_type = "Presentation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Presentation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`*"]
    pub type Presentation;
    #[cfg(feature = "PresentationRequest")]
    #[wasm_bindgen(method, getter, js_class = "Presentation", js_name = "defaultRequest")]
    #[doc = "Getter for the `defaultRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
    pub fn default_request(this: &Presentation) -> Option<PresentationRequest>;
    #[cfg(feature = "PresentationRequest")]
    #[wasm_bindgen(method, setter, js_class = "Presentation", js_name = "defaultRequest")]
    #[doc = "Setter for the `defaultRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
    pub fn set_default_request(this: &Presentation, value: Option<&PresentationRequest>);
    #[cfg(feature = "PresentationReceiver")]
    #[wasm_bindgen(method, getter, js_class = "Presentation", js_name = "receiver")]
    #[doc = "Getter for the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/receiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`, `PresentationReceiver`*"]
    pub fn receiver(this: &Presentation) -> Option<PresentationReceiver>;
}
