#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "UiEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "KeyboardEvent",
        typescript_type = "KeyboardEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyboardEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub type KeyboardEvent;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "charCode")]
    #[doc = "Getter for the `charCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/charCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn char_code(this: &KeyboardEvent) -> u32;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "keyCode")]
    #[doc = "Getter for the `keyCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn key_code(this: &KeyboardEvent) -> u32;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "altKey")]
    #[doc = "Getter for the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn alt_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "ctrlKey")]
    #[doc = "Getter for the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn ctrl_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "shiftKey")]
    #[doc = "Getter for the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn shift_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "metaKey")]
    #[doc = "Getter for the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/metaKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn meta_key(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "location")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn location(this: &KeyboardEvent) -> u32;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "repeat")]
    #[doc = "Getter for the `repeat` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn repeat(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "isComposing")]
    #[doc = "Getter for the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn is_composing(this: &KeyboardEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "key")]
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn key(this: &KeyboardEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "KeyboardEvent", js_name = "code")]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn code(this: &KeyboardEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "KeyboardEvent")]
    #[doc = "The `new KeyboardEvent(..)` constructor, creating a new instance of `KeyboardEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn new(type_arg: &str) -> Result<KeyboardEvent, JsValue>;
    #[cfg(feature = "KeyboardEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyboardEvent")]
    #[doc = "The `new KeyboardEvent(..)` constructor, creating a new instance of `KeyboardEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `KeyboardEventInit`*"]
    pub fn new_with_keyboard_event_init_dict(
        type_arg: &str,
        keyboard_event_init_dict: &KeyboardEventInit,
    ) -> Result<KeyboardEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "KeyboardEvent", js_name = "getModifierState")]
    #[doc = "The `getModifierState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn get_modifier_state(this: &KeyboardEvent, key: &str) -> bool;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn init_keyboard_event(this: &KeyboardEvent, type_arg: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn init_keyboard_event_with_bubbles_arg(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "KeyboardEvent",
        js_name = "initKeyboardEvent"
    )]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        this: &KeyboardEvent,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) -> Result<(), JsValue>;
}
impl KeyboardEvent {
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_STANDARD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_STANDARD: u32 = 0u64 as u32;
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_LEFT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_LEFT: u32 = 1u64 as u32;
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_RIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_RIGHT: u32 = 2u64 as u32;
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_NUMPAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_NUMPAD: u32 = 3u64 as u32;
}
