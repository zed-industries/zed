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
        js_name = "SVGSymbolElement",
        typescript_type = "SVGSymbolElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgSymbolElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
    pub type SvgSymbolElement;
    #[cfg(feature = "SvgAnimatedRect")]
    #[wasm_bindgen(method, getter, js_class = "SVGSymbolElement", js_name = "viewBox")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgSymbolElement`*"]
    pub fn view_box(this: &SvgSymbolElement) -> SvgAnimatedRect;
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGSymbolElement",
        js_name = "preserveAspectRatio"
    )]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgSymbolElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgSymbolElement) -> SvgAnimatedPreserveAspectRatio;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGSymbolElement",
        js_name = "requiredFeatures"
    )]
    #[doc = "Getter for the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    pub fn required_features(this: &SvgSymbolElement) -> SvgStringList;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGSymbolElement",
        js_name = "requiredExtensions"
    )]
    #[doc = "Getter for the `requiredExtensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    pub fn required_extensions(this: &SvgSymbolElement) -> SvgStringList;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGSymbolElement",
        js_name = "systemLanguage"
    )]
    #[doc = "Getter for the `systemLanguage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/systemLanguage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
    pub fn system_language(this: &SvgSymbolElement) -> SvgStringList;
    #[wasm_bindgen(method, js_class = "SVGSymbolElement", js_name = "hasExtension")]
    #[doc = "The `hasExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/hasExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
    pub fn has_extension(this: &SvgSymbolElement, extension: &str) -> bool;
}
