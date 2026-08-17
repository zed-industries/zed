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
        js_name = "SVGFESpotLightElement",
        typescript_type = "SVGFESpotLightElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeSpotLightElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeSpotLightElement`*"]
    pub type SvgfeSpotLightElement;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFESpotLightElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn x(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFESpotLightElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn y(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFESpotLightElement", js_name = "z")]
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn z(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFESpotLightElement",
        js_name = "pointsAtX"
    )]
    #[doc = "Getter for the `pointsAtX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn points_at_x(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFESpotLightElement",
        js_name = "pointsAtY"
    )]
    #[doc = "Getter for the `pointsAtY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn points_at_y(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFESpotLightElement",
        js_name = "pointsAtZ"
    )]
    #[doc = "Getter for the `pointsAtZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn points_at_z(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFESpotLightElement",
        js_name = "specularExponent"
    )]
    #[doc = "Getter for the `specularExponent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/specularExponent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn specular_exponent(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFESpotLightElement",
        js_name = "limitingConeAngle"
    )]
    #[doc = "Getter for the `limitingConeAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/limitingConeAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn limiting_cone_angle(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
}
