#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "CommandEvent",
        typescript_type = "CommandEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CommandEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEvent`*"]
    pub type CommandEvent;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "CommandEvent", js_name = "source")]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEvent`, `Element`*"]
    pub fn source(this: &CommandEvent) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "CommandEvent", js_name = "command")]
    #[doc = "Getter for the `command` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent/command)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEvent`*"]
    pub fn command(this: &CommandEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "CommandEvent")]
    #[doc = "The `new CommandEvent(..)` constructor, creating a new instance of `CommandEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent/CommandEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEvent`*"]
    pub fn new(type_: &str) -> Result<CommandEvent, JsValue>;
    #[cfg(feature = "CommandEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CommandEvent")]
    #[doc = "The `new CommandEvent(..)` constructor, creating a new instance of `CommandEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent/CommandEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEvent`, `CommandEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CommandEventInit,
    ) -> Result<CommandEvent, JsValue>;
}
