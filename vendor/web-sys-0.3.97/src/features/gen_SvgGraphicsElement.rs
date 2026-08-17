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
        js_name = "SVGGraphicsElement",
        typescript_type = "SVGGraphicsElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGraphicsElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
    pub type SvgGraphicsElement;
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[wasm_bindgen(method, getter, js_class = "SVGGraphicsElement", js_name = "transform")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGraphicsElement`*"]
    pub fn transform(this: &SvgGraphicsElement) -> SvgAnimatedTransformList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGraphicsElement",
        js_name = "nearestViewportElement"
    )]
    #[doc = "Getter for the `nearestViewportElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/nearestViewportElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
    pub fn nearest_viewport_element(this: &SvgGraphicsElement) -> Option<SvgElement>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGraphicsElement",
        js_name = "farthestViewportElement"
    )]
    #[doc = "Getter for the `farthestViewportElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/farthestViewportElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
    pub fn farthest_viewport_element(this: &SvgGraphicsElement) -> Option<SvgElement>;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGraphicsElement",
        js_name = "requiredFeatures"
    )]
    #[doc = "Getter for the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
    pub fn required_features(this: &SvgGraphicsElement) -> SvgStringList;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGraphicsElement",
        js_name = "requiredExtensions"
    )]
    #[doc = "Getter for the `requiredExtensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
    pub fn required_extensions(this: &SvgGraphicsElement) -> SvgStringList;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGraphicsElement",
        js_name = "systemLanguage"
    )]
    #[doc = "Getter for the `systemLanguage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/systemLanguage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
    pub fn system_language(this: &SvgGraphicsElement) -> SvgStringList;
    #[cfg(feature = "SvgRect")]
    #[wasm_bindgen(catch, method, js_class = "SVGGraphicsElement", js_name = "getBBox")]
    #[doc = "The `getBBox()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgRect`*"]
    pub fn get_b_box(this: &SvgGraphicsElement) -> Result<SvgRect, JsValue>;
    #[cfg(all(feature = "SvgBoundingBoxOptions", feature = "SvgRect",))]
    #[wasm_bindgen(catch, method, js_class = "SVGGraphicsElement", js_name = "getBBox")]
    #[doc = "The `getBBox()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`, `SvgGraphicsElement`, `SvgRect`*"]
    pub fn get_b_box_with_a_options(
        this: &SvgGraphicsElement,
        a_options: &SvgBoundingBoxOptions,
    ) -> Result<SvgRect, JsValue>;
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(method, js_class = "SVGGraphicsElement", js_name = "getCTM")]
    #[doc = "The `getCTM()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getCTM)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
    pub fn get_ctm(this: &SvgGraphicsElement) -> Option<SvgMatrix>;
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(method, js_class = "SVGGraphicsElement", js_name = "getScreenCTM")]
    #[doc = "The `getScreenCTM()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getScreenCTM)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
    pub fn get_screen_ctm(this: &SvgGraphicsElement) -> Option<SvgMatrix>;
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGGraphicsElement",
        js_name = "getTransformToElement"
    )]
    #[doc = "The `getTransformToElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getTransformToElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
    pub fn get_transform_to_element(
        this: &SvgGraphicsElement,
        element: &SvgGraphicsElement,
    ) -> Result<SvgMatrix, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGGraphicsElement", js_name = "hasExtension")]
    #[doc = "The `hasExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/hasExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
    pub fn has_extension(this: &SvgGraphicsElement, extension: &str) -> bool;
}
