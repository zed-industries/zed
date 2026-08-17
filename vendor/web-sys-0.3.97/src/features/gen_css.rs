pub mod css {
    #![allow(unused_imports)]
    #![allow(clippy::all)]
    use super::super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = "CSS")]
        pub type JsNamespaceCss;
    }
    #[wasm_bindgen]
    extern "C" {
        #[cfg(web_sys_unstable_apis)]
        #[cfg(feature = "HighlightRegistry")]
        #[wasm_bindgen(
            static_method_of = "JsNamespaceCss",
            js_class = "CSS",
            getter,
            js_name = "highlights"
        )]
        #[doc = "Getter for the `CSS.highlights` field."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/highlights)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `HighlightRegistry`, `css`*"]
        #[doc = ""]
        #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
        #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
        pub fn highlights() -> HighlightRegistry;
        #[wasm_bindgen(js_namespace = "CSS")]
        #[doc = "The `CSS.escape()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/escape)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `css`*"]
        pub fn escape(ident: &str) -> ::alloc::string::String;
        #[wasm_bindgen(catch, js_namespace = "CSS", js_name = "supports")]
        #[doc = "The `CSS.supports()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/supports)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `css`*"]
        pub fn supports_with_value(property: &str, value: &str) -> Result<bool, JsValue>;
        #[wasm_bindgen(catch, js_namespace = "CSS")]
        #[doc = "The `CSS.supports()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/supports)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `css`*"]
        pub fn supports(condition_text: &str) -> Result<bool, JsValue>;
    }
}
