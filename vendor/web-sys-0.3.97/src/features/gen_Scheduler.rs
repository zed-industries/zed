#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Scheduler",
        typescript_type = "Scheduler"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Scheduler` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Scheduler;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "Scheduler", js_name = "postTask")]
    #[doc = "The `postTask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn post_task(
        this: &Scheduler,
        callback: &::js_sys::Function<fn() -> ::wasm_bindgen::JsValue>,
    ) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SchedulerPostTaskOptions")]
    #[wasm_bindgen(method, js_class = "Scheduler", js_name = "postTask")]
    #[doc = "The `postTask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`, `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn post_task_with_options(
        this: &Scheduler,
        callback: &::js_sys::Function<fn() -> ::wasm_bindgen::JsValue>,
        options: &SchedulerPostTaskOptions,
    ) -> ::js_sys::Promise;
}
