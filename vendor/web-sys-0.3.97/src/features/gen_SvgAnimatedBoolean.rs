#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAnimatedBoolean",
        typescript_type = "SVGAnimatedBoolean"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedBoolean` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
    pub type SvgAnimatedBoolean;
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedBoolean", js_name = "baseVal")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
    pub fn base_val(this: &SvgAnimatedBoolean) -> bool;
    #[wasm_bindgen(method, setter, js_class = "SVGAnimatedBoolean", js_name = "baseVal")]
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
    pub fn set_base_val(this: &SvgAnimatedBoolean, value: bool);
    #[wasm_bindgen(method, getter, js_class = "SVGAnimatedBoolean", js_name = "animVal")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
    pub fn anim_val(this: &SvgAnimatedBoolean) -> bool;
}
