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
        js_name = "DragEvent",
        typescript_type = "DragEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DragEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
    pub type DragEvent;
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, getter, js_class = "DragEvent", js_name = "dataTransfer")]
    #[doc = "Getter for the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/dataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DragEvent`*"]
    pub fn data_transfer(this: &DragEvent) -> Option<DataTransfer>;
    #[wasm_bindgen(catch, constructor, js_class = "DragEvent")]
    #[doc = "The `new DragEvent(..)` constructor, creating a new instance of `DragEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
    pub fn new(type_: &str) -> Result<DragEvent, JsValue>;
    #[cfg(feature = "DragEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DragEvent")]
    #[doc = "The `new DragEvent(..)` constructor, creating a new instance of `DragEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `DragEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DragEventInit,
    ) -> Result<DragEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
    pub fn init_drag_event(this: &DragEvent, type_: &str);
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
    pub fn init_drag_event_with_can_bubble(this: &DragEvent, type_: &str, can_bubble: bool);
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
        a_button: u16,
    );
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `EventTarget`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
        a_button: u16,
        a_related_target: Option<&EventTarget>,
    );
    #[cfg(all(feature = "DataTransfer", feature = "EventTarget", feature = "Window",))]
    #[wasm_bindgen(method, js_class = "DragEvent", js_name = "initDragEvent")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DragEvent`, `EventTarget`, `Window`*"]
    pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer(
        this: &DragEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
        a_screen_x: i32,
        a_screen_y: i32,
        a_client_x: i32,
        a_client_y: i32,
        a_ctrl_key: bool,
        a_alt_key: bool,
        a_shift_key: bool,
        a_meta_key: bool,
        a_button: u16,
        a_related_target: Option<&EventTarget>,
        a_data_transfer: Option<&DataTransfer>,
    );
}
