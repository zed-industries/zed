#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Window",
        typescript_type = "Window"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Window` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub type Window;
    #[cfg(feature = "CookieStore")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "cookieStore")]
    #[doc = "Getter for the `cookieStore` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cookieStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`, `Window`*"]
    pub fn cookie_store(this: &Window) -> CookieStore;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "window")]
    #[doc = "Getter for the `window` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/window)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn window(this: &Window) -> Window;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "self")]
    #[doc = "Getter for the `self` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/self)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn self_(this: &Window) -> Window;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "document")]
    #[doc = "Getter for the `document` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Window`*"]
    pub fn document(this: &Window) -> Option<Document>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn name(this: &Window) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_name(this: &Window, value: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Location")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "location")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`, `Window`*"]
    pub fn location(this: &Window) -> Location;
    #[cfg(feature = "History")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "history")]
    #[doc = "Getter for the `history` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/history)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`, `Window`*"]
    pub fn history(this: &Window) -> Result<History, JsValue>;
    #[cfg(feature = "CustomElementRegistry")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "customElements")]
    #[doc = "Getter for the `customElements` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `Window`*"]
    pub fn custom_elements(this: &Window) -> CustomElementRegistry;
    #[cfg(feature = "BarProp")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "locationbar")]
    #[doc = "Getter for the `locationbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/locationbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn locationbar(this: &Window) -> Result<BarProp, JsValue>;
    #[cfg(feature = "BarProp")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "menubar")]
    #[doc = "Getter for the `menubar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/menubar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn menubar(this: &Window) -> Result<BarProp, JsValue>;
    #[cfg(feature = "BarProp")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "personalbar")]
    #[doc = "Getter for the `personalbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/personalbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn personalbar(this: &Window) -> Result<BarProp, JsValue>;
    #[cfg(feature = "BarProp")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "scrollbars")]
    #[doc = "Getter for the `scrollbars` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollbars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn scrollbars(this: &Window) -> Result<BarProp, JsValue>;
    #[cfg(feature = "BarProp")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "statusbar")]
    #[doc = "Getter for the `statusbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/statusbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn statusbar(this: &Window) -> Result<BarProp, JsValue>;
    #[cfg(feature = "BarProp")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "toolbar")]
    #[doc = "Getter for the `toolbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/toolbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
    pub fn toolbar(this: &Window) -> Result<BarProp, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "status")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn status(this: &Window) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "status")]
    #[doc = "Setter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_status(this: &Window, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "closed")]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn closed(this: &Window) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "event")]
    #[doc = "Getter for the `event` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn event(this: &Window) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "frames")]
    #[doc = "Getter for the `frames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn frames(this: &Window) -> Result<Window, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn length(this: &Window) -> u32;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "top")]
    #[doc = "Getter for the `top` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/top)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn top(this: &Window) -> Result<Option<Window>, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "opener")]
    #[doc = "Getter for the `opener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn opener(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "opener")]
    #[doc = "Setter for the `opener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_opener(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "parent")]
    #[doc = "Getter for the `parent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/parent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn parent(this: &Window) -> Result<Option<Window>, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "frameElement")]
    #[doc = "Getter for the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frameElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `Window`*"]
    pub fn frame_element(this: &Window) -> Result<Option<Element>, JsValue>;
    #[cfg(feature = "Navigator")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "navigator")]
    #[doc = "Getter for the `navigator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Window`*"]
    pub fn navigator(this: &Window) -> Navigator;
    #[cfg(feature = "External")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "external")]
    #[doc = "Getter for the `external` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/external)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`, `Window`*"]
    pub fn external(this: &Window) -> Result<External, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onappinstalled")]
    #[doc = "Getter for the `onappinstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onappinstalled(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onappinstalled")]
    #[doc = "Setter for the `onappinstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onappinstalled(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(feature = "Screen")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "screen")]
    #[doc = "Getter for the `screen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `Window`*"]
    pub fn screen(this: &Window) -> Result<Screen, JsValue>;
    #[cfg(feature = "VisualViewport")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "visualViewport")]
    #[doc = "Getter for the `visualViewport` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/visualViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`, `Window`*"]
    pub fn visual_viewport(this: &Window) -> Option<VisualViewport>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "innerWidth")]
    #[doc = "Getter for the `innerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn inner_width(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "innerWidth")]
    #[doc = "Setter for the `innerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_inner_width(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "innerHeight")]
    #[doc = "Getter for the `innerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn inner_height(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "innerHeight")]
    #[doc = "Setter for the `innerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_inner_height(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "scrollX")]
    #[doc = "Getter for the `scrollX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_x(this: &Window) -> Result<f64, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "pageXOffset")]
    #[doc = "Getter for the `pageXOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageXOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn page_x_offset(this: &Window) -> Result<f64, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "scrollY")]
    #[doc = "Getter for the `scrollY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_y(this: &Window) -> Result<f64, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "pageYOffset")]
    #[doc = "Getter for the `pageYOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageYOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn page_y_offset(this: &Window) -> Result<f64, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "screenX")]
    #[doc = "Getter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn screen_x(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "screenX")]
    #[doc = "Setter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_screen_x(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "screenY")]
    #[doc = "Getter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn screen_y(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "screenY")]
    #[doc = "Setter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_screen_y(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "outerWidth")]
    #[doc = "Getter for the `outerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn outer_width(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "outerWidth")]
    #[doc = "Setter for the `outerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_outer_width(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "outerHeight")]
    #[doc = "Getter for the `outerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn outer_height(this: &Window) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "Window", js_name = "outerHeight")]
    #[doc = "Setter for the `outerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_outer_height(this: &Window, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "devicePixelRatio")]
    #[doc = "Getter for the `devicePixelRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn device_pixel_ratio(this: &Window) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "orientation")]
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn orientation(this: &Window) -> i16;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onorientationchange")]
    #[doc = "Getter for the `onorientationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onorientationchange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onorientationchange")]
    #[doc = "Setter for the `onorientationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onorientationchange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onvrdisplayconnect")]
    #[doc = "Getter for the `onvrdisplayconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplayconnect(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onvrdisplayconnect")]
    #[doc = "Setter for the `onvrdisplayconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplayconnect(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onvrdisplaydisconnect")]
    #[doc = "Getter for the `onvrdisplaydisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplaydisconnect(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onvrdisplaydisconnect")]
    #[doc = "Setter for the `onvrdisplaydisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplaydisconnect(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onvrdisplayactivate")]
    #[doc = "Getter for the `onvrdisplayactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplayactivate(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onvrdisplayactivate")]
    #[doc = "Setter for the `onvrdisplayactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplayactivate(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onvrdisplaydeactivate")]
    #[doc = "Getter for the `onvrdisplaydeactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplaydeactivate(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onvrdisplaydeactivate")]
    #[doc = "Setter for the `onvrdisplaydeactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplaydeactivate(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Window",
        js_name = "onvrdisplaypresentchange"
    )]
    #[doc = "Getter for the `onvrdisplaypresentchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvrdisplaypresentchange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Window",
        js_name = "onvrdisplaypresentchange"
    )]
    #[doc = "Setter for the `onvrdisplaypresentchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvrdisplaypresentchange(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(feature = "Worklet")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "paintWorklet")]
    #[doc = "Getter for the `paintWorklet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/paintWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`, `Worklet`*"]
    pub fn paint_worklet(this: &Window) -> Result<Worklet, JsValue>;
    #[cfg(feature = "Crypto")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "crypto")]
    #[doc = "Getter for the `crypto` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/crypto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`, `Window`*"]
    pub fn crypto(this: &Window) -> Result<Crypto, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onabort(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onabort(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onblur")]
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onblur(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onblur")]
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onblur(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onfocus")]
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onfocus(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onfocus")]
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onfocus(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "oncancel")]
    #[doc = "Getter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncancel(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "oncancel")]
    #[doc = "Setter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncancel(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onauxclick")]
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onauxclick(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onauxclick")]
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onauxclick(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onbeforetoggle")]
    #[doc = "Getter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onbeforetoggle(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onbeforetoggle")]
    #[doc = "Setter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onbeforetoggle(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "oncanplay")]
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncanplay(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "oncanplay")]
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncanplay(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "oncanplaythrough")]
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncanplaythrough(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "oncanplaythrough")]
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncanplaythrough(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onchange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onchange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onclick")]
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onclick(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onclick")]
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onclick(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onclose(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onclose(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "oncontextmenu")]
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn oncontextmenu(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "oncontextmenu")]
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oncontextmenu(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondblclick")]
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondblclick(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondblclick")]
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondblclick(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondrag")]
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondrag(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondrag")]
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondrag(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondragend")]
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondragend")]
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondragenter")]
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragenter(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondragenter")]
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragenter(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondragexit")]
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragexit(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondragexit")]
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragexit(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondragleave")]
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragleave(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondragleave")]
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragleave(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondragover")]
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragover(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondragover")]
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragover(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondragstart")]
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondragstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondragstart")]
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondragstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondrop")]
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondrop(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondrop")]
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondrop(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ondurationchange")]
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ondurationchange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ondurationchange")]
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ondurationchange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onemptied")]
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onemptied(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onemptied")]
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onemptied(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onended")]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onended(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onended")]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onended(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "oninput")]
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn oninput(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "oninput")]
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oninput(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onbeforeinput")]
    #[doc = "Getter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onbeforeinput(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onbeforeinput")]
    #[doc = "Setter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onbeforeinput(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "oninvalid")]
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn oninvalid(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "oninvalid")]
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_oninvalid(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onkeydown")]
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onkeydown(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onkeydown")]
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onkeydown(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onkeypress")]
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onkeypress(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onkeypress")]
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onkeypress(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onkeyup")]
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onkeyup(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onkeyup")]
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onkeyup(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onload")]
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onload(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onload")]
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onload(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onloadeddata")]
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadeddata(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onloadeddata")]
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadeddata(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onloadedmetadata")]
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadedmetadata(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onloadedmetadata")]
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadedmetadata(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onloadend")]
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onloadend")]
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onloadstart")]
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onloadstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onloadstart")]
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onloadstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmousedown")]
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmousedown(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmousedown")]
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmousedown(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmouseenter")]
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseenter(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmouseenter")]
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseenter(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmouseleave")]
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseleave(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmouseleave")]
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseleave(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmousemove")]
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmousemove(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmousemove")]
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmousemove(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmouseout")]
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseout(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmouseout")]
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseout(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmouseover")]
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseover(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmouseover")]
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseover(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmouseup")]
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmouseup(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmouseup")]
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmouseup(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onwheel")]
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwheel(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onwheel")]
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwheel(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpause")]
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpause(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpause")]
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpause(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onplay")]
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onplay(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onplay")]
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onplay(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onplaying")]
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onplaying(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onplaying")]
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onplaying(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onprogress")]
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onprogress(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onprogress")]
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onprogress(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onratechange")]
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onratechange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onratechange")]
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onratechange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onreset")]
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onreset(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onreset")]
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onreset(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onresize")]
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onresize(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onresize")]
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onresize(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onscroll")]
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onscroll(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onscroll")]
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onscroll(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onseeked")]
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onseeked(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onseeked")]
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onseeked(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onseeking")]
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onseeking(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onseeking")]
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onseeking(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onselect")]
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onselect(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onselect")]
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onselect(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onshow")]
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onshow(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onshow")]
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onshow(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onstalled")]
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onstalled(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onstalled")]
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onstalled(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onsubmit")]
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onsubmit(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onsubmit")]
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onsubmit(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onsuspend")]
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onsuspend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onsuspend")]
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onsuspend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontimeupdate")]
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontimeupdate(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontimeupdate")]
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontimeupdate(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onvolumechange")]
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onvolumechange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onvolumechange")]
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onvolumechange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onwaiting")]
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwaiting(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onwaiting")]
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwaiting(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onselectstart")]
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onselectstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onselectstart")]
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onselectstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontoggle")]
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontoggle(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontoggle")]
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontoggle(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointercancel")]
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointercancel(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointercancel")]
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointercancel(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointerdown")]
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerdown(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointerdown")]
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerdown(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointerup")]
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerup(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointerup")]
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerup(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointermove")]
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointermove(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointermove")]
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointermove(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointerout")]
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerout(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointerout")]
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerout(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointerover")]
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerover(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointerover")]
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerover(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointerenter")]
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerenter(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointerenter")]
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerenter(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpointerleave")]
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpointerleave(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpointerleave")]
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpointerleave(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ongotpointercapture")]
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ongotpointercapture(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ongotpointercapture")]
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ongotpointercapture(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onlostpointercapture")]
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onlostpointercapture(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onlostpointercapture")]
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onlostpointercapture(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onanimationcancel")]
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationcancel(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onanimationcancel")]
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationcancel(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onanimationend")]
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onanimationend")]
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onanimationiteration")]
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationiteration(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onanimationiteration")]
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationiteration(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onanimationstart")]
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onanimationstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onanimationstart")]
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onanimationstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontransitioncancel")]
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitioncancel(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontransitioncancel")]
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitioncancel(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontransitionend")]
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitionend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontransitionend")]
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitionend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontransitionrun")]
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitionrun(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontransitionrun")]
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitionrun(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontransitionstart")]
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontransitionstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontransitionstart")]
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontransitionstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onwebkitanimationend")]
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkitanimationend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onwebkitanimationend")]
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkitanimationend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Window",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkitanimationiteration(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Window",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkitanimationiteration(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Window",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkitanimationstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Window",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkitanimationstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onwebkittransitionend")]
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onwebkittransitionend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onwebkittransitionend")]
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onwebkittransitionend(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(feature = "U2f")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "u2f")]
    #[doc = "Getter for the `u2f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/u2f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`, `Window`*"]
    pub fn u2f(this: &Window) -> Result<U2f, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onerror(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onerror(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(feature = "SpeechSynthesis")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "Window",
        js_name = "speechSynthesis"
    )]
    #[doc = "Getter for the `speechSynthesis` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/speechSynthesis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`, `Window`*"]
    pub fn speech_synthesis(this: &Window) -> Result<SpeechSynthesis, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontouchstart")]
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchstart(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontouchstart")]
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchstart(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontouchend")]
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchend(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontouchend")]
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchend(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontouchmove")]
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchmove(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontouchmove")]
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchmove(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ontouchcancel")]
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ontouchcancel(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ontouchcancel")]
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ontouchcancel(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onafterprint")]
    #[doc = "Getter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onafterprint(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onafterprint")]
    #[doc = "Setter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onafterprint(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onbeforeprint")]
    #[doc = "Getter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onbeforeprint(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onbeforeprint")]
    #[doc = "Setter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onbeforeprint(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onbeforeunload")]
    #[doc = "Getter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onbeforeunload(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onbeforeunload")]
    #[doc = "Setter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onbeforeunload(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onhashchange")]
    #[doc = "Getter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onhashchange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onhashchange")]
    #[doc = "Setter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onhashchange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onlanguagechange")]
    #[doc = "Getter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onlanguagechange(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onlanguagechange")]
    #[doc = "Setter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onlanguagechange(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmessage")]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmessage(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmessage")]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmessage(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onmessageerror")]
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onmessageerror(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onmessageerror")]
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onmessageerror(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onoffline")]
    #[doc = "Getter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onoffline(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onoffline")]
    #[doc = "Setter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onoffline(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ononline")]
    #[doc = "Getter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn ononline(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ononline")]
    #[doc = "Setter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_ononline(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpagehide")]
    #[doc = "Getter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpagehide(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpagehide")]
    #[doc = "Setter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpagehide(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpageshow")]
    #[doc = "Getter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpageshow(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpageshow")]
    #[doc = "Setter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpageshow(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onpopstate")]
    #[doc = "Getter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onpopstate(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onpopstate")]
    #[doc = "Setter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onpopstate(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onstorage")]
    #[doc = "Getter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onstorage(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onstorage")]
    #[doc = "Setter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onstorage(this: &Window, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "onunload")]
    #[doc = "Getter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn onunload(this: &Window) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "onunload")]
    #[doc = "Setter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_onunload(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ongamepadconnected")]
    #[doc = "Getter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongamepadconnected(this: &Window) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ongamepadconnected")]
    #[doc = "Setter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongamepadconnected(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "ongamepaddisconnected")]
    #[doc = "Getter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongamepaddisconnected(this: &Window) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "Window", js_name = "ongamepaddisconnected")]
    #[doc = "Setter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongamepaddisconnected(this: &Window, value: Option<&::js_sys::Function>);
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "localStorage")]
    #[doc = "Getter for the `localStorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `Window`*"]
    pub fn local_storage(this: &Window) -> Result<Option<Storage>, JsValue>;
    #[cfg(feature = "IdbFactory")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "indexedDB")]
    #[doc = "Getter for the `indexedDB` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/indexedDB)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `Window`*"]
    pub fn indexed_db(this: &Window) -> Result<Option<IdbFactory>, JsValue>;
    #[cfg(feature = "Performance")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "performance")]
    #[doc = "Getter for the `performance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/performance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`, `Window`*"]
    pub fn performance(this: &Window) -> Option<Performance>;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "origin")]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn origin(this: &Window) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "isSecureContext")]
    #[doc = "Getter for the `isSecureContext` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/isSecureContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn is_secure_context(this: &Window) -> bool;
    #[cfg(feature = "CacheStorage")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "caches")]
    #[doc = "Getter for the `caches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/caches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`, `Window`*"]
    pub fn caches(this: &Window) -> Result<CacheStorage, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Scheduler")]
    #[wasm_bindgen(method, getter, js_class = "Window", js_name = "scheduler")]
    #[doc = "Getter for the `scheduler` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scheduler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scheduler(this: &Window) -> Scheduler;
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(catch, method, getter, js_class = "Window", js_name = "sessionStorage")]
    #[doc = "Getter for the `sessionStorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `Window`*"]
    pub fn session_storage(this: &Window) -> Result<Option<Storage>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `alert()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn alert(this: &Window) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "alert")]
    #[doc = "The `alert()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn alert_with_message(this: &Window, message: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `blur()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/blur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn blur(this: &Window) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Window", js_name = "cancelIdleCallback")]
    #[doc = "The `cancelIdleCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelIdleCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn cancel_idle_callback(this: &Window, handle: u32);
    #[wasm_bindgen(method, js_class = "Window", js_name = "captureEvents")]
    #[doc = "The `captureEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/captureEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn capture_events(this: &Window);
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn close(this: &Window) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `confirm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn confirm(this: &Window) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "confirm")]
    #[doc = "The `confirm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn confirm_with_message(this: &Window, message: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn focus(this: &Window) -> Result<(), JsValue>;
    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "getComputedStyle")]
    #[doc = "The `getComputedStyle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*"]
    pub fn get_computed_style(
        this: &Window,
        elt: &Element,
    ) -> Result<Option<CssStyleDeclaration>, JsValue>;
    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "getComputedStyle")]
    #[doc = "The `getComputedStyle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*"]
    pub fn get_computed_style_with_pseudo_elt(
        this: &Window,
        elt: &Element,
        pseudo_elt: &str,
    ) -> Result<Option<CssStyleDeclaration>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ScreenDetails")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "getScreenDetails")]
    #[doc = "The `getScreenDetails()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getScreenDetails)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetails`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_screen_details(this: &Window) -> Result<::js_sys::Promise<ScreenDetails>, JsValue>;
    #[cfg(feature = "Selection")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "getSelection")]
    #[doc = "The `getSelection()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`, `Window`*"]
    pub fn get_selection(this: &Window) -> Result<Option<Selection>, JsValue>;
    #[cfg(feature = "MediaQueryList")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "matchMedia")]
    #[doc = "The `matchMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`, `Window`*"]
    pub fn match_media(this: &Window, query: &str) -> Result<Option<MediaQueryList>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "moveBy")]
    #[doc = "The `moveBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn move_by(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "moveTo")]
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn move_to(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn open(this: &Window) -> Result<Option<Window>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn open_with_url(this: &Window, url: &str) -> Result<Option<Window>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn open_with_url_and_target(
        this: &Window,
        url: &str,
        target: &str,
    ) -> Result<Option<Window>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn open_with_url_and_target_and_features(
        this: &Window,
        url: &str,
        target: &str,
        features: &str,
    ) -> Result<Option<Window>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn post_message(
        this: &Window,
        message: &::wasm_bindgen::JsValue,
        target_origin: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn post_message_with_transfer(
        this: &Window,
        message: &::wasm_bindgen::JsValue,
        target_origin: &str,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `print()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/print)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn print(this: &Window) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `prompt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn prompt(this: &Window) -> Result<Option<::alloc::string::String>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "prompt")]
    #[doc = "The `prompt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn prompt_with_message(
        this: &Window,
        message: &str,
    ) -> Result<Option<::alloc::string::String>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "prompt")]
    #[doc = "The `prompt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn prompt_with_message_and_default(
        this: &Window,
        message: &str,
        default: &str,
    ) -> Result<Option<::alloc::string::String>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FontData")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "queryLocalFonts")]
    #[doc = "The `queryLocalFonts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontData`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn query_local_fonts(
        this: &Window,
    ) -> Result<::js_sys::Promise<::js_sys::Array<FontData>>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "FontData", feature = "QueryOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "queryLocalFonts")]
    #[doc = "The `queryLocalFonts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontData`, `QueryOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn query_local_fonts_with_options(
        this: &Window,
        options: &QueryOptions,
    ) -> Result<::js_sys::Promise<::js_sys::Array<FontData>>, JsValue>;
    #[wasm_bindgen(method, js_class = "Window", js_name = "releaseEvents")]
    #[doc = "The `releaseEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/releaseEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn release_events(this: &Window);
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "requestIdleCallback")]
    #[doc = "The `requestIdleCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn request_idle_callback(
        this: &Window,
        callback: &::js_sys::Function,
    ) -> Result<u32, JsValue>;
    #[cfg(feature = "IdleRequestOptions")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "requestIdleCallback")]
    #[doc = "The `requestIdleCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`, `Window`*"]
    pub fn request_idle_callback_with_options(
        this: &Window,
        callback: &::js_sys::Function,
        options: &IdleRequestOptions,
    ) -> Result<u32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "resizeBy")]
    #[doc = "The `resizeBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn resize_by(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "resizeTo")]
    #[doc = "The `resizeTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn resize_to(this: &Window, x: i32, y: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Window", js_name = "scroll")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_with_x_and_y(this: &Window, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Window")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll(this: &Window);
    #[cfg(feature = "ScrollToOptions")]
    #[wasm_bindgen(method, js_class = "Window", js_name = "scroll")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
    pub fn scroll_with_scroll_to_options(this: &Window, options: &ScrollToOptions);
    #[wasm_bindgen(method, js_class = "Window", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_by_with_x_and_y(this: &Window, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Window", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_by(this: &Window);
    #[cfg(feature = "ScrollToOptions")]
    #[wasm_bindgen(method, js_class = "Window", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
    pub fn scroll_by_with_scroll_to_options(this: &Window, options: &ScrollToOptions);
    #[wasm_bindgen(method, js_class = "Window", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_to_with_x_and_y(this: &Window, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Window", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn scroll_to(this: &Window);
    #[cfg(feature = "ScrollToOptions")]
    #[wasm_bindgen(method, js_class = "Window", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
    pub fn scroll_to_with_scroll_to_options(this: &Window, options: &ScrollToOptions);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemDirectoryHandle")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "showDirectoryPicker")]
    #[doc = "The `showDirectoryPicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn show_directory_picker(
        this: &Window,
    ) -> Result<::js_sys::Promise<FileSystemDirectoryHandle>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "DirectoryPickerOptions",
        feature = "FileSystemDirectoryHandle",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "showDirectoryPicker")]
    #[doc = "The `showDirectoryPicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `FileSystemDirectoryHandle`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn show_directory_picker_with_options(
        this: &Window,
        options: &DirectoryPickerOptions,
    ) -> Result<::js_sys::Promise<FileSystemDirectoryHandle>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemFileHandle")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "showOpenFilePicker")]
    #[doc = "The `showOpenFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn show_open_file_picker(
        this: &Window,
    ) -> Result<::js_sys::Promise<::js_sys::Array<FileSystemFileHandle>>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "FileSystemFileHandle", feature = "OpenFilePickerOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "showOpenFilePicker")]
    #[doc = "The `showOpenFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`, `OpenFilePickerOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn show_open_file_picker_with_options(
        this: &Window,
        options: &OpenFilePickerOptions,
    ) -> Result<::js_sys::Promise<::js_sys::Array<FileSystemFileHandle>>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemFileHandle")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "showSaveFilePicker")]
    #[doc = "The `showSaveFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn show_save_file_picker(
        this: &Window,
    ) -> Result<::js_sys::Promise<FileSystemFileHandle>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "FileSystemFileHandle", feature = "SaveFilePickerOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "showSaveFilePicker")]
    #[doc = "The `showSaveFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`, `SaveFilePickerOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn show_save_file_picker_with_options(
        this: &Window,
        options: &SaveFilePickerOptions,
    ) -> Result<::js_sys::Promise<FileSystemFileHandle>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn stop(this: &Window) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Window", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn get(this: &Window, name: &str) -> Option<::js_sys::Object>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "cancelAnimationFrame")]
    #[doc = "The `cancelAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn cancel_animation_frame(this: &Window, handle: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "requestAnimationFrame")]
    #[doc = "The `requestAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn request_animation_frame(
        this: &Window,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `atob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/atob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn atob(this: &Window, atob: &str) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window")]
    #[doc = "The `btoa()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/btoa)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn btoa(this: &Window, btoa: &str) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "Window", js_name = "clearInterval")]
    #[doc = "The `clearInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_interval(this: &Window);
    #[wasm_bindgen(method, js_class = "Window", js_name = "clearInterval")]
    #[doc = "The `clearInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_interval_with_handle(this: &Window, handle: i32);
    #[wasm_bindgen(method, js_class = "Window", js_name = "clearTimeout")]
    #[doc = "The `clearTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_timeout(this: &Window);
    #[wasm_bindgen(method, js_class = "Window", js_name = "clearTimeout")]
    #[doc = "The `clearTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn clear_timeout_with_handle(this: &Window, handle: i32);
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_image_element(
        this: &Window,
        a_image: &HtmlImageElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_svg_image_element(
        this: &Window,
        a_image: &SvgImageElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_canvas_element(
        this: &Window,
        a_image: &HtmlCanvasElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_video_element(
        this: &Window,
        a_image: &HtmlVideoElement,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*"]
    pub fn create_image_bitmap_with_image_bitmap(
        this: &Window,
        a_image: &ImageBitmap,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `Window`*"]
    pub fn create_image_bitmap_with_offscreen_canvas(
        this: &Window,
        a_image: &OffscreenCanvas,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmap", feature = "VideoFrame",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `VideoFrame`, `Window`*"]
    pub fn create_image_bitmap_with_video_frame(
        this: &Window,
        a_image: &VideoFrame,
    ) -> Result<::js_sys::Promise<ImageBitmap>, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Window`*"]
    pub fn create_image_bitmap_with_blob(
        this: &Window,
        a_image: &Blob,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `Window`*"]
    pub fn create_image_bitmap_with_image_data(
        this: &Window,
        a_image: &ImageData,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlImageElement", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_html_image_element_and_image_bitmap_options(
        this: &Window,
        a_image: &HtmlImageElement,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmapOptions", feature = "SvgImageElement",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `SvgImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_svg_image_element_and_image_bitmap_options(
        this: &Window,
        a_image: &SvgImageElement,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlCanvasElement", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_html_canvas_element_and_image_bitmap_options(
        this: &Window,
        a_image: &HtmlCanvasElement,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlVideoElement", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_html_video_element_and_image_bitmap_options(
        this: &Window,
        a_image: &HtmlVideoElement,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_image_bitmap_and_image_bitmap_options(
        this: &Window,
        a_image: &ImageBitmap,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmapOptions", feature = "OffscreenCanvas",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `OffscreenCanvas`, `Window`*"]
    pub fn create_image_bitmap_with_offscreen_canvas_and_image_bitmap_options(
        this: &Window,
        a_image: &OffscreenCanvas,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(
        feature = "ImageBitmap",
        feature = "ImageBitmapOptions",
        feature = "VideoFrame",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `VideoFrame`, `Window`*"]
    pub fn create_image_bitmap_with_video_frame_and_image_bitmap_options(
        this: &Window,
        a_image: &VideoFrame,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise<ImageBitmap>, JsValue>;
    #[cfg(all(feature = "Blob", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_blob_and_image_bitmap_options(
        this: &Window,
        a_image: &Blob,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmapOptions", feature = "ImageData",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageData`, `Window`*"]
    pub fn create_image_bitmap_with_image_data_and_image_bitmap_options(
        this: &Window,
        a_image: &ImageData,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_svg_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &SvgImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlCanvasElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*"]
    pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &HtmlVideoElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*"]
    pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &ImageBitmap,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `Window`*"]
    pub fn create_image_bitmap_with_offscreen_canvas_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &OffscreenCanvas,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmap", feature = "VideoFrame",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `VideoFrame`, `Window`*"]
    pub fn create_image_bitmap_with_video_frame_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &VideoFrame,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise<ImageBitmap>, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Window`*"]
    pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &Blob,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `Window`*"]
    pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh(
        this: &Window,
        a_image: &ImageData,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlImageElement", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &HtmlImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmapOptions", feature = "SvgImageElement",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `SvgImageElement`, `Window`*"]
    pub fn create_image_bitmap_with_svg_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &SvgImageElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlCanvasElement", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &HtmlCanvasElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlVideoElement", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &HtmlVideoElement,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &ImageBitmap,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmapOptions", feature = "OffscreenCanvas",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `OffscreenCanvas`, `Window`*"]
    pub fn create_image_bitmap_with_offscreen_canvas_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &OffscreenCanvas,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(
        feature = "ImageBitmap",
        feature = "ImageBitmapOptions",
        feature = "VideoFrame",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `VideoFrame`, `Window`*"]
    pub fn create_image_bitmap_with_video_frame_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &VideoFrame,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise<ImageBitmap>, JsValue>;
    #[cfg(all(feature = "Blob", feature = "ImageBitmapOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ImageBitmapOptions`, `Window`*"]
    pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &Blob,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "ImageBitmapOptions", feature = "ImageData",))]
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "createImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageData`, `Window`*"]
    pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(
        this: &Window,
        a_image: &ImageData,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
        a_options: &ImageBitmapOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, js_class = "Window", js_name = "fetch")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `Window`*"]
    pub fn fetch_with_request(this: &Window, input: &Request) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Window", js_name = "fetch")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn fetch_with_str(this: &Window, input: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    #[wasm_bindgen(method, js_class = "Window", js_name = "fetch")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`, `Window`*"]
    pub fn fetch_with_request_and_init(
        this: &Window,
        input: &Request,
        init: &RequestInit,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(method, js_class = "Window", js_name = "fetch")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `Window`*"]
    pub fn fetch_with_str_and_init(
        this: &Window,
        input: &str,
        init: &RequestInit,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Window", js_name = "queueMicrotask")]
    #[doc = "The `queueMicrotask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/queueMicrotask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn queue_microtask(this: &Window, callback: &::js_sys::Function);
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback(
        this: &Window,
        handler: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::js_sys::Array,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_0(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_1(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_2(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_3(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_4(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_5(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_6(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_callback_and_timeout_and_arguments_7(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
        arguments_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str(this: &Window, handler: &str) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused: &::js_sys::Array,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_0(
        this: &Window,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_1(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_2(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_3(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_4(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_5(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_6(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setInterval")]
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_interval_with_str_and_timeout_and_unused_7(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
        unused_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback(
        this: &Window,
        handler: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments: &::js_sys::Array,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_0(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_1(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_2(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_3(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_4(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_5(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_6(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_7(
        this: &Window,
        handler: &::js_sys::Function,
        timeout: i32,
        arguments_1: &::wasm_bindgen::JsValue,
        arguments_2: &::wasm_bindgen::JsValue,
        arguments_3: &::wasm_bindgen::JsValue,
        arguments_4: &::wasm_bindgen::JsValue,
        arguments_5: &::wasm_bindgen::JsValue,
        arguments_6: &::wasm_bindgen::JsValue,
        arguments_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str(this: &Window, handler: &str) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused: &::js_sys::Array,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_0(
        this: &Window,
        handler: &str,
        timeout: i32,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_1(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_2(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_3(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_4(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_5(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_6(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Window", js_name = "setTimeout")]
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn set_timeout_with_str_and_timeout_and_unused_7(
        this: &Window,
        handler: &str,
        timeout: i32,
        unused_1: &::wasm_bindgen::JsValue,
        unused_2: &::wasm_bindgen::JsValue,
        unused_3: &::wasm_bindgen::JsValue,
        unused_4: &::wasm_bindgen::JsValue,
        unused_5: &::wasm_bindgen::JsValue,
        unused_6: &::wasm_bindgen::JsValue,
        unused_7: &::wasm_bindgen::JsValue,
    ) -> Result<i32, JsValue>;
}
