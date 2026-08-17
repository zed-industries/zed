#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AbortSignal",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "TaskSignal",
        typescript_type = "TaskSignal"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskSignal` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type TaskSignal;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(method, getter, js_class = "TaskSignal", js_name = "priority")]
    #[doc = "Getter for the `priority` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/priority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn priority(this: &TaskSignal) -> TaskPriority;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "TaskSignal", js_name = "onprioritychange")]
    #[doc = "Getter for the `onprioritychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/onprioritychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onprioritychange(this: &TaskSignal) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "TaskSignal", js_name = "onprioritychange")]
    #[doc = "Setter for the `onprioritychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/onprioritychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onprioritychange(this: &TaskSignal, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(static_method_of = "TaskSignal", js_class = "TaskSignal")]
    #[doc = "The `any()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/any_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn any(signals: &[AbortSignal]) -> TaskSignal;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskSignalAnyInit")]
    #[wasm_bindgen(
        static_method_of = "TaskSignal",
        js_class = "TaskSignal",
        js_name = "any"
    )]
    #[doc = "The `any()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/any_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`, `TaskSignalAnyInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn any_with_init(signals: &[AbortSignal], init: &TaskSignalAnyInit) -> TaskSignal;
}
