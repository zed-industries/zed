#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "TimeEvent",
        typescript_type = "TimeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TimeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`*"]
    pub type TimeEvent;
    #[wasm_bindgen(method, getter, js_class = "TimeEvent", js_name = "detail")]
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`*"]
    pub fn detail(this: &TimeEvent) -> i32;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter, js_class = "TimeEvent", js_name = "view")]
    #[doc = "Getter for the `view` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/view)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    pub fn view(this: &TimeEvent) -> Option<Window>;
    #[wasm_bindgen(method, js_class = "TimeEvent", js_name = "initTimeEvent")]
    #[doc = "The `initTimeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`*"]
    pub fn init_time_event(this: &TimeEvent, a_type: &str);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TimeEvent", js_name = "initTimeEvent")]
    #[doc = "The `initTimeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    pub fn init_time_event_with_a_view(this: &TimeEvent, a_type: &str, a_view: Option<&Window>);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "TimeEvent", js_name = "initTimeEvent")]
    #[doc = "The `initTimeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    pub fn init_time_event_with_a_view_and_a_detail(
        this: &TimeEvent,
        a_type: &str,
        a_view: Option<&Window>,
        a_detail: i32,
    );
}
