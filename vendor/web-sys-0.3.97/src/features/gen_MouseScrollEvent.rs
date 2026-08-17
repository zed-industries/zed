#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MouseEvent",
        extends = "UiEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MouseScrollEvent",
        typescript_type = "MouseScrollEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MouseScrollEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub type MouseScrollEvent;
    #[wasm_bindgen(method, getter, js_class = "MouseScrollEvent", js_name = "axis")]
    #[doc = "Getter for the `axis` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/axis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub fn axis(this: &MouseScrollEvent) -> i32;
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub fn init_mouse_scroll_event(this: &MouseScrollEvent, type_: &str);
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub fn init_mouse_scroll_event_with_can_bubble(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
    );
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        button: i16,
    );
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        button: i16,
        related_target: Option<&EventTarget>,
    );
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[wasm_bindgen(
        method,
        js_class = "MouseScrollEvent",
        js_name = "initMouseScrollEvent"
    )]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseScrollEvent`, `Window`*"]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis(
        this: &MouseScrollEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        button: i16,
        related_target: Option<&EventTarget>,
        axis: i32,
    );
}
impl MouseScrollEvent {
    #[doc = "The `MouseScrollEvent.HORIZONTAL_AXIS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub const HORIZONTAL_AXIS: i32 = 1u64 as i32;
    #[doc = "The `MouseScrollEvent.VERTICAL_AXIS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub const VERTICAL_AXIS: i32 = 2u64 as i32;
}
