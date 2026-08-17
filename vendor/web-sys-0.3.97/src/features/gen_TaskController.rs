#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AbortController",
        extends = "::js_sys::Object",
        js_name = "TaskController",
        typescript_type = "TaskController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type TaskController;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "TaskController")]
    #[doc = "The `new TaskController(..)` constructor, creating a new instance of `TaskController`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/TaskController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Result<TaskController, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskControllerInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TaskController")]
    #[doc = "The `new TaskController(..)` constructor, creating a new instance of `TaskController`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/TaskController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`, `TaskControllerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_init(init: &TaskControllerInit) -> Result<TaskController, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(catch, method, js_class = "TaskController", js_name = "setPriority")]
    #[doc = "The `setPriority()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/setPriority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`, `TaskPriority`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_priority(this: &TaskController, priority: TaskPriority) -> Result<(), JsValue>;
}
