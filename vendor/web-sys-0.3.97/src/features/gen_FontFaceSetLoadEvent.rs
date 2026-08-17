#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "FontFaceSetLoadEvent",
        typescript_type = "FontFaceSetLoadEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetLoadEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
    pub type FontFaceSetLoadEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "FontFaceSetLoadEvent",
        js_name = "fontfaces"
    )]
    #[doc = "Getter for the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/fontfaces)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
    pub fn fontfaces(this: &FontFaceSetLoadEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "FontFaceSetLoadEvent")]
    #[doc = "The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
    pub fn new(type_: &str) -> Result<FontFaceSetLoadEvent, JsValue>;
    #[cfg(feature = "FontFaceSetLoadEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFaceSetLoadEvent")]
    #[doc = "The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`, `FontFaceSetLoadEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &FontFaceSetLoadEventInit,
    ) -> Result<FontFaceSetLoadEvent, JsValue>;
}
