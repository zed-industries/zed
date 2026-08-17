#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "ScrollBoxObject" , typescript_type = "ScrollBoxObject")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollBoxObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub type ScrollBoxObject;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ScrollBoxObject",
        js_name = "positionX"
    )]
    #[doc = "Getter for the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn position_x(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ScrollBoxObject",
        js_name = "positionY"
    )]
    #[doc = "Getter for the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn position_y(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ScrollBoxObject",
        js_name = "scrolledWidth"
    )]
    #[doc = "Getter for the `scrolledWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scrolled_width(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ScrollBoxObject",
        js_name = "scrolledHeight"
    )]
    #[doc = "Getter for the `scrolledHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scrolled_height(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ScrollBoxObject",
        js_name = "ensureElementIsVisible"
    )]
    #[doc = "The `ensureElementIsVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/ensureElementIsVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
    pub fn ensure_element_is_visible(
        this: &ScrollBoxObject,
        child: &Element,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ScrollBoxObject", js_name = "scrollBy")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scroll_by(this: &ScrollBoxObject, dx: i32, dy: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ScrollBoxObject", js_name = "scrollByIndex")]
    #[doc = "The `scrollByIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollByIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scroll_by_index(this: &ScrollBoxObject, dindexes: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ScrollBoxObject", js_name = "scrollTo")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scroll_to(this: &ScrollBoxObject, x: i32, y: i32) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ScrollBoxObject",
        js_name = "scrollToElement"
    )]
    #[doc = "The `scrollToElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollToElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
    pub fn scroll_to_element(this: &ScrollBoxObject, child: &Element) -> Result<(), JsValue>;
}
