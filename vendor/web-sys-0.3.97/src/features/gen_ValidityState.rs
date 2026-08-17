#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ValidityState",
        typescript_type = "ValidityState"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ValidityState` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub type ValidityState;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "valueMissing")]
    #[doc = "Getter for the `valueMissing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valueMissing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn value_missing(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "typeMismatch")]
    #[doc = "Getter for the `typeMismatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/typeMismatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn type_mismatch(this: &ValidityState) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ValidityState",
        js_name = "patternMismatch"
    )]
    #[doc = "Getter for the `patternMismatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/patternMismatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn pattern_mismatch(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "tooLong")]
    #[doc = "Getter for the `tooLong` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooLong)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn too_long(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "tooShort")]
    #[doc = "Getter for the `tooShort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooShort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn too_short(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "rangeUnderflow")]
    #[doc = "Getter for the `rangeUnderflow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeUnderflow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn range_underflow(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "rangeOverflow")]
    #[doc = "Getter for the `rangeOverflow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeOverflow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn range_overflow(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "stepMismatch")]
    #[doc = "Getter for the `stepMismatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/stepMismatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn step_mismatch(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "badInput")]
    #[doc = "Getter for the `badInput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/badInput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn bad_input(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "customError")]
    #[doc = "Getter for the `customError` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/customError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn custom_error(this: &ValidityState) -> bool;
    #[wasm_bindgen(method, getter, js_class = "ValidityState", js_name = "valid")]
    #[doc = "Getter for the `valid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn valid(this: &ValidityState) -> bool;
}
