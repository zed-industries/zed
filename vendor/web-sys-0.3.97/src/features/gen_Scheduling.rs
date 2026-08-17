#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Scheduling",
        typescript_type = "Scheduling"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Scheduling` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Scheduling;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "Scheduling", js_name = "isInputPending")]
    #[doc = "The `isInputPending()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling/isInputPending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_input_pending(this: &Scheduling) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IsInputPendingOptions")]
    #[wasm_bindgen(method, js_class = "Scheduling", js_name = "isInputPending")]
    #[doc = "The `isInputPending()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling/isInputPending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`, `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_input_pending_with_is_input_pending_options(
        this: &Scheduling,
        is_input_pending_options: &IsInputPendingOptions,
    ) -> bool;
}
