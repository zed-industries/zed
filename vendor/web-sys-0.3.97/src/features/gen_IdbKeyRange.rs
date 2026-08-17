#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IDBKeyRange",
        typescript_type = "IDBKeyRange"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbKeyRange` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub type IdbKeyRange;
    #[wasm_bindgen(catch, method, getter, js_class = "IDBKeyRange", js_name = "lower")]
    #[doc = "Getter for the `lower` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower(this: &IdbKeyRange) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "IDBKeyRange", js_name = "upper")]
    #[doc = "Getter for the `upper` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper(this: &IdbKeyRange) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "IDBKeyRange", js_name = "lowerOpen")]
    #[doc = "Getter for the `lowerOpen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower_open(this: &IdbKeyRange) -> bool;
    #[wasm_bindgen(method, getter, js_class = "IDBKeyRange", js_name = "upperOpen")]
    #[doc = "Getter for the `upperOpen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper_open(this: &IdbKeyRange) -> bool;
    #[wasm_bindgen(catch, static_method_of = "IdbKeyRange", js_class = "IDBKeyRange")]
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn bound(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "IdbKeyRange",
        js_class = "IDBKeyRange",
        js_name = "bound"
    )]
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn bound_with_lower_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "IdbKeyRange",
        js_class = "IDBKeyRange",
        js_name = "bound"
    )]
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn bound_with_lower_open_and_upper_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
        upper_open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBKeyRange")]
    #[doc = "The `includes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn includes(this: &IdbKeyRange, key: &::wasm_bindgen::JsValue) -> Result<bool, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "IdbKeyRange",
        js_class = "IDBKeyRange",
        js_name = "lowerBound"
    )]
    #[doc = "The `lowerBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower_bound(lower: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "IdbKeyRange",
        js_class = "IDBKeyRange",
        js_name = "lowerBound"
    )]
    #[doc = "The `lowerBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower_bound_with_open(
        lower: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(catch, static_method_of = "IdbKeyRange", js_class = "IDBKeyRange")]
    #[doc = "The `only()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn only(value: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "IdbKeyRange",
        js_class = "IDBKeyRange",
        js_name = "upperBound"
    )]
    #[doc = "The `upperBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper_bound(upper: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "IdbKeyRange",
        js_class = "IDBKeyRange",
        js_name = "upperBound"
    )]
    #[doc = "The `upperBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper_bound_with_open(
        upper: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
}
