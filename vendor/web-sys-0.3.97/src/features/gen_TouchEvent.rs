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
        js_name = "TouchEvent",
        typescript_type = "TouchEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub type TouchEvent;
    #[cfg(feature = "TouchList")]
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "touches")]
    #[doc = "Getter for the `touches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    pub fn touches(this: &TouchEvent) -> TouchList;
    #[cfg(feature = "TouchList")]
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "targetTouches")]
    #[doc = "Getter for the `targetTouches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    pub fn target_touches(this: &TouchEvent) -> TouchList;
    #[cfg(feature = "TouchList")]
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "changedTouches")]
    #[doc = "Getter for the `changedTouches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    pub fn changed_touches(this: &TouchEvent) -> TouchList;
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "altKey")]
    #[doc = "Getter for the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn alt_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "metaKey")]
    #[doc = "Getter for the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn meta_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "ctrlKey")]
    #[doc = "Getter for the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn ctrl_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "TouchEvent", js_name = "shiftKey")]
    #[doc = "Getter for the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn shift_key(this: &TouchEvent) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "TouchEvent")]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn new(type_: &str) -> Result<TouchEvent, JsValue>;
    #[cfg(feature = "TouchEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TouchEvent")]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TouchEventInit,
    ) -> Result<TouchEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn init_touch_event(this: &TouchEvent, type_: &str);
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn init_touch_event_with_can_bubble(this: &TouchEvent, type_: &str, can_bubble: bool);
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    );
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
    );
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
        target_touches: Option<&TouchList>,
    );
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    #[wasm_bindgen(method, js_class = "TouchEvent", js_name = "initTouchEvent")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches(
        this: &TouchEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
        target_touches: Option<&TouchList>,
        changed_touches: Option<&TouchList>,
    );
}
