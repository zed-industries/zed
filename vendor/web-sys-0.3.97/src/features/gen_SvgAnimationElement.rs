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
        js_name = "SVGAnimationElement",
        typescript_type = "SVGAnimationElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimationElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub type SvgAnimationElement;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimationElement",
        js_name = "targetElement"
    )]
    #[doc = "Getter for the `targetElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/targetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn target_element(this: &SvgAnimationElement) -> Option<SvgElement>;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimationElement",
        js_name = "requiredFeatures"
    )]
    #[doc = "Getter for the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    pub fn required_features(this: &SvgAnimationElement) -> SvgStringList;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimationElement",
        js_name = "requiredExtensions"
    )]
    #[doc = "Getter for the `requiredExtensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    pub fn required_extensions(this: &SvgAnimationElement) -> SvgStringList;
    #[cfg(feature = "SvgStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAnimationElement",
        js_name = "systemLanguage"
    )]
    #[doc = "Getter for the `systemLanguage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/systemLanguage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    pub fn system_language(this: &SvgAnimationElement) -> SvgStringList;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAnimationElement",
        js_name = "beginElement"
    )]
    #[doc = "The `beginElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn begin_element(this: &SvgAnimationElement) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAnimationElement",
        js_name = "beginElementAt"
    )]
    #[doc = "The `beginElementAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElementAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn begin_element_at(this: &SvgAnimationElement, offset: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAnimationElement",
        js_name = "endElement"
    )]
    #[doc = "The `endElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn end_element(this: &SvgAnimationElement) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAnimationElement",
        js_name = "endElementAt"
    )]
    #[doc = "The `endElementAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElementAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn end_element_at(this: &SvgAnimationElement, offset: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "SVGAnimationElement", js_name = "getCurrentTime")]
    #[doc = "The `getCurrentTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getCurrentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn get_current_time(this: &SvgAnimationElement) -> f32;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAnimationElement",
        js_name = "getSimpleDuration"
    )]
    #[doc = "The `getSimpleDuration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getSimpleDuration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn get_simple_duration(this: &SvgAnimationElement) -> Result<f32, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAnimationElement",
        js_name = "getStartTime"
    )]
    #[doc = "The `getStartTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getStartTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn get_start_time(this: &SvgAnimationElement) -> Result<f32, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGAnimationElement", js_name = "hasExtension")]
    #[doc = "The `hasExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/hasExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    pub fn has_extension(this: &SvgAnimationElement, extension: &str) -> bool;
}
