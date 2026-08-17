#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGFEDistantLightElement",
        typescript_type = "SVGFEDistantLightElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDistantLightElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDistantLightElement`*"]
    pub type SvgfeDistantLightElement;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDistantLightElement",
        js_name = "azimuth"
    )]
    #[doc = "Getter for the `azimuth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/azimuth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDistantLightElement`*"]
    pub fn azimuth(this: &SvgfeDistantLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDistantLightElement",
        js_name = "elevation"
    )]
    #[doc = "Getter for the `elevation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/elevation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDistantLightElement`*"]
    pub fn elevation(this: &SvgfeDistantLightElement) -> SvgAnimatedNumber;
}
