#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "UIEvent",
        typescript_type = "UIEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UiEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub type UiEvent;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "view")]
    #[doc = "Getter for the `view` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/view)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    pub fn view(this: &UiEvent) -> Option<Window>;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "detail")]
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn detail(this: &UiEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "layerX")]
    #[doc = "Getter for the `layerX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn layer_x(this: &UiEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "layerY")]
    #[doc = "Getter for the `layerY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn layer_y(this: &UiEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "pageX")]
    #[doc = "Getter for the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn page_x(this: &UiEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "pageY")]
    #[doc = "Getter for the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn page_y(this: &UiEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "which")]
    #[doc = "Getter for the `which` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/which)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn which(this: &UiEvent) -> u32;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "rangeParent")]
    #[doc = "Getter for the `rangeParent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `UiEvent`*"]
    pub fn range_parent(this: &UiEvent) -> Option<Node>;
    #[wasm_bindgen(method, getter, js_class = "UIEvent", js_name = "rangeOffset")]
    #[doc = "Getter for the `rangeOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn range_offset(this: &UiEvent) -> i32;
    #[wasm_bindgen(catch, constructor, js_class = "UIEvent")]
    #[doc = "The `new UiEvent(..)` constructor, creating a new instance of `UiEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn new(type_: &str) -> Result<UiEvent, JsValue>;
    #[cfg(feature = "UiEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "UIEvent")]
    #[doc = "The `new UiEvent(..)` constructor, creating a new instance of `UiEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `UiEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &UiEventInit,
    ) -> Result<UiEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "UIEvent", js_name = "initUIEvent")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn init_ui_event(this: &UiEvent, a_type: &str);
    #[wasm_bindgen(method, js_class = "UIEvent", js_name = "initUIEvent")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn init_ui_event_with_a_can_bubble(this: &UiEvent, a_type: &str, a_can_bubble: bool);
    #[wasm_bindgen(method, js_class = "UIEvent", js_name = "initUIEvent")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable(
        this: &UiEvent,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "UIEvent", js_name = "initUIEvent")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view(
        this: &UiEvent,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
        a_view: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "UIEvent", js_name = "initUIEvent")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail(
        this: &UiEvent,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
    );
}
impl UiEvent {
    #[doc = "The `UIEvent.SCROLL_PAGE_UP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub const SCROLL_PAGE_UP: i32 = -32768i64 as i32;
    #[doc = "The `UIEvent.SCROLL_PAGE_DOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub const SCROLL_PAGE_DOWN: i32 = 32768u64 as i32;
}
