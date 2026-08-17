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
        js_name = "SVGFEPointLightElement",
        typescript_type = "SVGFEPointLightElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfePointLightElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfePointLightElement`*"]
    pub type SvgfePointLightElement;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEPointLightElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfePointLightElement`*"]
    pub fn x(this: &SvgfePointLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEPointLightElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfePointLightElement`*"]
    pub fn y(this: &SvgfePointLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEPointLightElement", js_name = "z")]
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfePointLightElement`*"]
    pub fn z(this: &SvgfePointLightElement) -> SvgAnimatedNumber;
}
