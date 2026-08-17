#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "HighlightRegistry",
        typescript_type = "HighlightRegistry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HighlightRegistry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HighlightRegistry;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HighlightRegistry", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &HighlightRegistry) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn clear(this: &HighlightRegistry);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn delete(this: &HighlightRegistry, key: &str) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[wasm_bindgen(catch, method, js_class = "HighlightRegistry", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn for_each(
        this: &HighlightRegistry,
        callback: &::js_sys::Function<fn(Highlight, ::js_sys::JsString) -> ::js_sys::Undefined>,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get(this: &HighlightRegistry, key: &str) -> Option<Highlight>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has(this: &HighlightRegistry, key: &str) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HighlightHitResult")]
    #[wasm_bindgen(
        method,
        js_class = "HighlightRegistry",
        js_name = "highlightsFromPoint"
    )]
    #[doc = "The `highlightsFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/highlightsFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightHitResult`, `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn highlights_from_point(
        this: &HighlightRegistry,
        x: f32,
        y: f32,
    ) -> ::js_sys::Array<HighlightHitResult>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "HighlightHitResult", feature = "HighlightsFromPointOptions",))]
    #[wasm_bindgen(
        method,
        js_class = "HighlightRegistry",
        js_name = "highlightsFromPoint"
    )]
    #[doc = "The `highlightsFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/highlightsFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightHitResult`, `HighlightRegistry`, `HighlightsFromPointOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn highlights_from_point_with_options(
        this: &HighlightRegistry,
        x: f32,
        y: f32,
        options: &HighlightsFromPointOptions,
    ) -> ::js_sys::Array<HighlightHitResult>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set(this: &HighlightRegistry, key: &str, value: &Highlight) -> HighlightRegistry;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(
        this: &HighlightRegistry,
    ) -> ::js_sys::Iterator<::js_sys::ArrayTuple<(::js_sys::JsString, Highlight)>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &HighlightRegistry) -> ::js_sys::Iterator<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Highlight")]
    #[wasm_bindgen(method, js_class = "HighlightRegistry")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HighlightRegistry/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Highlight`, `HighlightRegistry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn values(this: &HighlightRegistry) -> ::js_sys::Iterator<Highlight>;
}
