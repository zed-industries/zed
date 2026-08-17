#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CssRule",
        extends = "::js_sys::Object",
        js_name = "CSSCounterStyleRule",
        typescript_type = "CSSCounterStyleRule"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssCounterStyleRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub type CssCounterStyleRule;
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn name(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_name(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "system")]
    #[doc = "Getter for the `system` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn system(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "system")]
    #[doc = "Setter for the `system` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_system(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "symbols")]
    #[doc = "Getter for the `symbols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn symbols(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "symbols")]
    #[doc = "Setter for the `symbols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_symbols(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSCounterStyleRule",
        js_name = "additiveSymbols"
    )]
    #[doc = "Getter for the `additiveSymbols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn additive_symbols(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CSSCounterStyleRule",
        js_name = "additiveSymbols"
    )]
    #[doc = "Setter for the `additiveSymbols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_additive_symbols(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "negative")]
    #[doc = "Getter for the `negative` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn negative(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "negative")]
    #[doc = "Setter for the `negative` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_negative(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "prefix")]
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn prefix(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "prefix")]
    #[doc = "Setter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_prefix(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "suffix")]
    #[doc = "Getter for the `suffix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn suffix(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "suffix")]
    #[doc = "Setter for the `suffix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_suffix(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "range")]
    #[doc = "Getter for the `range` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn range(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "range")]
    #[doc = "Setter for the `range` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_range(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "pad")]
    #[doc = "Getter for the `pad` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn pad(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "pad")]
    #[doc = "Setter for the `pad` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_pad(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "speakAs")]
    #[doc = "Getter for the `speakAs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn speak_as(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "speakAs")]
    #[doc = "Setter for the `speakAs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_speak_as(this: &CssCounterStyleRule, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSCounterStyleRule", js_name = "fallback")]
    #[doc = "Getter for the `fallback` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn fallback(this: &CssCounterStyleRule) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSCounterStyleRule", js_name = "fallback")]
    #[doc = "Setter for the `fallback` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_fallback(this: &CssCounterStyleRule, value: &str);
}
