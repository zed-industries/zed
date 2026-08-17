#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "GamepadEvent",
        typescript_type = "GamepadEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEvent`*"]
    pub type GamepadEvent;
    #[cfg(feature = "Gamepad")]
    #[wasm_bindgen(method, getter, js_class = "GamepadEvent", js_name = "gamepad")]
    #[doc = "Getter for the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/gamepad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadEvent`*"]
    pub fn gamepad(this: &GamepadEvent) -> Option<Gamepad>;
    #[wasm_bindgen(catch, constructor, js_class = "GamepadEvent")]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEvent`*"]
    pub fn new(type_: &str) -> Result<GamepadEvent, JsValue>;
    #[cfg(feature = "GamepadEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "GamepadEvent")]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEvent`, `GamepadEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadEventInit,
    ) -> Result<GamepadEvent, JsValue>;
}
