#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MathMLElement",
        typescript_type = "MathMLElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MathMlElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub type MathMlElement;
    #[cfg(feature = "CssStyleDeclaration")]
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "style")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `MathMlElement`*"]
    pub fn style(this: &MathMlElement) -> CssStyleDeclaration;
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onabort(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onabort(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onblur")]
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onblur(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onblur")]
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onblur(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onfocus")]
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onfocus(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onfocus")]
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onfocus(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "oncancel")]
    #[doc = "Getter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn oncancel(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "oncancel")]
    #[doc = "Setter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_oncancel(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onauxclick")]
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onauxclick(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onauxclick")]
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onauxclick(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onbeforetoggle")]
    #[doc = "Getter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onbeforetoggle(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onbeforetoggle")]
    #[doc = "Setter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onbeforetoggle(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "oncanplay")]
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn oncanplay(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "oncanplay")]
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_oncanplay(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "oncanplaythrough"
    )]
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn oncanplaythrough(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "oncanplaythrough"
    )]
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_oncanplaythrough(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onchange(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onchange(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onclick")]
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onclick(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onclick")]
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onclick(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onclose(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onclose(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "oncontextmenu")]
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn oncontextmenu(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "oncontextmenu")]
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_oncontextmenu(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondblclick")]
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondblclick(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondblclick")]
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondblclick(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondrag")]
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondrag(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondrag")]
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondrag(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondragend")]
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondragend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondragend")]
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondragend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondragenter")]
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondragenter(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondragenter")]
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondragenter(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondragexit")]
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondragexit(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondragexit")]
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondragexit(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondragleave")]
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondragleave(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondragleave")]
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondragleave(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondragover")]
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondragover(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondragover")]
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondragover(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondragstart")]
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondragstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondragstart")]
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondragstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ondrop")]
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondrop(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ondrop")]
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondrop(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "ondurationchange"
    )]
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ondurationchange(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "ondurationchange"
    )]
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ondurationchange(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onemptied")]
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onemptied(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onemptied")]
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onemptied(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onended")]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onended(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onended")]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onended(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "oninput")]
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn oninput(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "oninput")]
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_oninput(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onbeforeinput")]
    #[doc = "Getter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onbeforeinput(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onbeforeinput")]
    #[doc = "Setter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onbeforeinput(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "oninvalid")]
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn oninvalid(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "oninvalid")]
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_oninvalid(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onkeydown")]
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onkeydown(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onkeydown")]
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onkeydown(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onkeypress")]
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onkeypress(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onkeypress")]
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onkeypress(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onkeyup")]
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onkeyup(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onkeyup")]
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onkeyup(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onload")]
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onload(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onload")]
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onload(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onloadeddata")]
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onloadeddata(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onloadeddata")]
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onloadeddata(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onloadedmetadata"
    )]
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onloadedmetadata(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onloadedmetadata"
    )]
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onloadedmetadata(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onloadend")]
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onloadend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onloadend")]
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onloadend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onloadstart")]
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onloadstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onloadstart")]
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onloadstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmousedown")]
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmousedown(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmousedown")]
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmousedown(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmouseenter")]
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmouseenter(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmouseenter")]
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmouseenter(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmouseleave")]
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmouseleave(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmouseleave")]
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmouseleave(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmousemove")]
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmousemove(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmousemove")]
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmousemove(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmouseout")]
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmouseout(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmouseout")]
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmouseout(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmouseover")]
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmouseover(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmouseover")]
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmouseover(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onmouseup")]
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onmouseup(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onmouseup")]
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onmouseup(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onwheel")]
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onwheel(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onwheel")]
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onwheel(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpause")]
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpause(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpause")]
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpause(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onplay")]
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onplay(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onplay")]
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onplay(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onplaying")]
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onplaying(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onplaying")]
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onplaying(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onprogress")]
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onprogress(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onprogress")]
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onprogress(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onratechange")]
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onratechange(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onratechange")]
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onratechange(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onreset")]
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onreset(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onreset")]
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onreset(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onresize")]
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onresize(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onresize")]
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onresize(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onscroll")]
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onscroll(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onscroll")]
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onscroll(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onseeked")]
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onseeked(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onseeked")]
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onseeked(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onseeking")]
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onseeking(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onseeking")]
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onseeking(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onselect")]
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onselect(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onselect")]
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onselect(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onshow")]
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onshow(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onshow")]
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onshow(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onstalled")]
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onstalled(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onstalled")]
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onstalled(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onsubmit")]
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onsubmit(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onsubmit")]
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onsubmit(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onsuspend")]
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onsuspend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onsuspend")]
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onsuspend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ontimeupdate")]
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontimeupdate(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ontimeupdate")]
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontimeupdate(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onvolumechange")]
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onvolumechange(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onvolumechange")]
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onvolumechange(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onwaiting")]
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onwaiting(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onwaiting")]
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onwaiting(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onselectstart")]
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onselectstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onselectstart")]
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onselectstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ontoggle")]
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontoggle(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ontoggle")]
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontoggle(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onpointercancel"
    )]
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointercancel(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onpointercancel"
    )]
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointercancel(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointerdown")]
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointerdown(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointerdown")]
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointerdown(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointerup")]
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointerup(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointerup")]
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointerup(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointermove")]
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointermove(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointermove")]
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointermove(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointerout")]
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointerout(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointerout")]
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointerout(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointerover")]
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointerover(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointerover")]
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointerover(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointerenter")]
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointerenter(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointerenter")]
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointerenter(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onpointerleave")]
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onpointerleave(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onpointerleave")]
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onpointerleave(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "ongotpointercapture"
    )]
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ongotpointercapture(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "ongotpointercapture"
    )]
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ongotpointercapture(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onlostpointercapture"
    )]
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onlostpointercapture(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onlostpointercapture"
    )]
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onlostpointercapture(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onanimationcancel"
    )]
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onanimationcancel(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onanimationcancel"
    )]
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onanimationcancel(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onanimationend")]
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onanimationend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onanimationend")]
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onanimationend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onanimationiteration"
    )]
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onanimationiteration(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onanimationiteration"
    )]
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onanimationiteration(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onanimationstart"
    )]
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onanimationstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onanimationstart"
    )]
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onanimationstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "ontransitioncancel"
    )]
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontransitioncancel(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "ontransitioncancel"
    )]
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontransitioncancel(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "ontransitionend"
    )]
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontransitionend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "ontransitionend"
    )]
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontransitionend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "ontransitionrun"
    )]
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontransitionrun(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "ontransitionrun"
    )]
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontransitionrun(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "ontransitionstart"
    )]
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontransitionstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "ontransitionstart"
    )]
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontransitionstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onwebkitanimationend"
    )]
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onwebkitanimationend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onwebkitanimationend"
    )]
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onwebkitanimationend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onwebkitanimationiteration(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onwebkitanimationiteration(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onwebkitanimationstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onwebkitanimationstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MathMLElement",
        js_name = "onwebkittransitionend"
    )]
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onwebkittransitionend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "onwebkittransitionend"
    )]
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onwebkittransitionend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[cfg(feature = "DomStringMap")]
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "dataset")]
    #[doc = "Getter for the `dataset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/dataset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`, `MathMlElement`*"]
    pub fn dataset(this: &MathMlElement) -> DomStringMap;
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "nonce")]
    #[doc = "Getter for the `nonce` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/nonce)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn nonce(this: &MathMlElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "nonce")]
    #[doc = "Setter for the `nonce` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/nonce)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_nonce(this: &MathMlElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "autofocus")]
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn autofocus(this: &MathMlElement) -> bool;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "autofocus"
    )]
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_autofocus(this: &MathMlElement, value: bool) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "tabIndex")]
    #[doc = "Getter for the `tabIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/tabIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn tab_index(this: &MathMlElement) -> i32;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "MathMLElement",
        js_name = "tabIndex"
    )]
    #[doc = "Setter for the `tabIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/tabIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_tab_index(this: &MathMlElement, value: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn onerror(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_onerror(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ontouchstart")]
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontouchstart(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ontouchstart")]
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontouchstart(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ontouchend")]
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontouchend(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ontouchend")]
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontouchend(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ontouchmove")]
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontouchmove(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ontouchmove")]
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontouchmove(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MathMLElement", js_name = "ontouchcancel")]
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn ontouchcancel(this: &MathMlElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MathMLElement", js_name = "ontouchcancel")]
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn set_ontouchcancel(this: &MathMlElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "MathMLElement")]
    #[doc = "The `blur()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/blur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn blur(this: &MathMlElement) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MathMLElement")]
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MathMlElement`*"]
    pub fn focus(this: &MathMlElement) -> Result<(), JsValue>;
    #[cfg(feature = "FocusOptions")]
    #[wasm_bindgen(catch, method, js_class = "MathMLElement", js_name = "focus")]
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MathMLElement/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`, `MathMlElement`*"]
    pub fn focus_with_options(this: &MathMlElement, options: &FocusOptions) -> Result<(), JsValue>;
}
