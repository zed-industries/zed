#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "TaskPriorityChangeEvent",
        typescript_type = "TaskPriorityChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskPriorityChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type TaskPriorityChangeEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TaskPriorityChangeEvent",
        js_name = "previousPriority"
    )]
    #[doc = "Getter for the `previousPriority` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent/previousPriority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskPriorityChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn previous_priority(this: &TaskPriorityChangeEvent) -> TaskPriority;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriorityChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TaskPriorityChangeEvent")]
    #[doc = "The `new TaskPriorityChangeEvent(..)` constructor, creating a new instance of `TaskPriorityChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent/TaskPriorityChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEvent`, `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        priority_change_event_init_dict: &TaskPriorityChangeEventInit,
    ) -> Result<TaskPriorityChangeEvent, JsValue>;
}
