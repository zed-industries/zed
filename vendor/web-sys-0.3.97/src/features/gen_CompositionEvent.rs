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
        js_name = "CompositionEvent",
        typescript_type = "CompositionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CompositionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub type CompositionEvent;
    #[wasm_bindgen(method, getter, js_class = "CompositionEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn data(this: &CompositionEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "CompositionEvent", js_name = "locale")]
    #[doc = "Getter for the `locale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/locale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn locale(this: &CompositionEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "CompositionEvent")]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn new(type_: &str) -> Result<CompositionEvent, JsValue>;
    #[cfg(feature = "CompositionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CompositionEvent")]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `CompositionEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CompositionEventInit,
    ) -> Result<CompositionEvent, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "CompositionEvent",
        js_name = "initCompositionEvent"
    )]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn init_composition_event(this: &CompositionEvent, type_arg: &str);
    #[wasm_bindgen(
        method,
        js_class = "CompositionEvent",
        js_name = "initCompositionEvent"
    )]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn init_composition_event_with_can_bubble_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
    );
    #[wasm_bindgen(
        method,
        js_class = "CompositionEvent",
        js_name = "initCompositionEvent"
    )]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "CompositionEvent",
        js_name = "initCompositionEvent"
    )]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "CompositionEvent",
        js_name = "initCompositionEvent"
    )]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        js_class = "CompositionEvent",
        js_name = "initCompositionEvent"
    )]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
    pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg(
        this: &CompositionEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        data_arg: Option<&str>,
        locale_arg: &str,
    );
}
