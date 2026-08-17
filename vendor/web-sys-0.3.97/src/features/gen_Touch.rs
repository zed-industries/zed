#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Touch",
        typescript_type = "Touch"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Touch` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub type Touch;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "identifier")]
    #[doc = "Getter for the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/identifier)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn identifier(this: &Touch) -> i32;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `Touch`*"]
    pub fn target(this: &Touch) -> Option<EventTarget>;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "screenX")]
    #[doc = "Getter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn screen_x(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "screenY")]
    #[doc = "Getter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn screen_y(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "clientX")]
    #[doc = "Getter for the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn client_x(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "clientY")]
    #[doc = "Getter for the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn client_y(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "pageX")]
    #[doc = "Getter for the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn page_x(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "pageY")]
    #[doc = "Getter for the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn page_y(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "radiusX")]
    #[doc = "Getter for the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn radius_x(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "radiusY")]
    #[doc = "Getter for the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn radius_y(this: &Touch) -> i32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "rotationAngle")]
    #[doc = "Getter for the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/rotationAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn rotation_angle(this: &Touch) -> f32;
    #[wasm_bindgen(method, getter, js_class = "Touch", js_name = "force")]
    #[doc = "Getter for the `force` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/force)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
    pub fn force(this: &Touch) -> f32;
    #[cfg(feature = "TouchInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Touch")]
    #[doc = "The `new Touch(..)` constructor, creating a new instance of `Touch`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/Touch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`, `TouchInit`*"]
    pub fn new(touch_init_dict: &TouchInit) -> Result<Touch, JsValue>;
}
