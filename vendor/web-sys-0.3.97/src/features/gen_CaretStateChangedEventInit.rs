#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CaretStateChangedEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CaretStateChangedEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub type CaretStateChangedEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &CaretStateChangedEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &CaretStateChangedEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &CaretStateChangedEventInit, val: bool);
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Get the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`, `DomRectReadOnly`*"]
    #[wasm_bindgen(method, getter = "boundingClientRect")]
    pub fn get_bounding_client_rect(this: &CaretStateChangedEventInit) -> Option<DomRectReadOnly>;
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Change the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`, `DomRectReadOnly`*"]
    #[wasm_bindgen(method, setter = "boundingClientRect")]
    pub fn set_bounding_client_rect(
        this: &CaretStateChangedEventInit,
        val: Option<&DomRectReadOnly>,
    );
    #[doc = "Get the `caretVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "caretVisible")]
    pub fn get_caret_visible(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `caretVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "caretVisible")]
    pub fn set_caret_visible(this: &CaretStateChangedEventInit, val: bool);
    #[doc = "Get the `caretVisuallyVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "caretVisuallyVisible")]
    pub fn get_caret_visually_visible(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `caretVisuallyVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "caretVisuallyVisible")]
    pub fn set_caret_visually_visible(this: &CaretStateChangedEventInit, val: bool);
    #[doc = "Get the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "collapsed")]
    pub fn get_collapsed(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "collapsed")]
    pub fn set_collapsed(this: &CaretStateChangedEventInit, val: bool);
    #[cfg(feature = "CaretChangedReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretChangedReason`, `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &CaretStateChangedEventInit) -> Option<CaretChangedReason>;
    #[cfg(feature = "CaretChangedReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretChangedReason`, `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &CaretStateChangedEventInit, val: CaretChangedReason);
    #[doc = "Get the `selectedTextContent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "selectedTextContent")]
    pub fn get_selected_text_content(
        this: &CaretStateChangedEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `selectedTextContent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "selectedTextContent")]
    pub fn set_selected_text_content(this: &CaretStateChangedEventInit, val: &str);
    #[doc = "Get the `selectionEditable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "selectionEditable")]
    pub fn get_selection_editable(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `selectionEditable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "selectionEditable")]
    pub fn set_selection_editable(this: &CaretStateChangedEventInit, val: bool);
    #[doc = "Get the `selectionVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, getter = "selectionVisible")]
    pub fn get_selection_visible(this: &CaretStateChangedEventInit) -> Option<bool>;
    #[doc = "Change the `selectionVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    #[wasm_bindgen(method, setter = "selectionVisible")]
    pub fn set_selection_visible(this: &CaretStateChangedEventInit, val: bool);
}
impl CaretStateChangedEventInit {
    #[doc = "Construct a new `CaretStateChangedEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "DomRectReadOnly")]
    #[deprecated = "Use `set_bounding_client_rect()` instead."]
    pub fn bounding_client_rect(&mut self, val: Option<&DomRectReadOnly>) -> &mut Self {
        self.set_bounding_client_rect(val);
        self
    }
    #[deprecated = "Use `set_caret_visible()` instead."]
    pub fn caret_visible(&mut self, val: bool) -> &mut Self {
        self.set_caret_visible(val);
        self
    }
    #[deprecated = "Use `set_caret_visually_visible()` instead."]
    pub fn caret_visually_visible(&mut self, val: bool) -> &mut Self {
        self.set_caret_visually_visible(val);
        self
    }
    #[deprecated = "Use `set_collapsed()` instead."]
    pub fn collapsed(&mut self, val: bool) -> &mut Self {
        self.set_collapsed(val);
        self
    }
    #[cfg(feature = "CaretChangedReason")]
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: CaretChangedReason) -> &mut Self {
        self.set_reason(val);
        self
    }
    #[deprecated = "Use `set_selected_text_content()` instead."]
    pub fn selected_text_content(&mut self, val: &str) -> &mut Self {
        self.set_selected_text_content(val);
        self
    }
    #[deprecated = "Use `set_selection_editable()` instead."]
    pub fn selection_editable(&mut self, val: bool) -> &mut Self {
        self.set_selection_editable(val);
        self
    }
    #[deprecated = "Use `set_selection_visible()` instead."]
    pub fn selection_visible(&mut self, val: bool) -> &mut Self {
        self.set_selection_visible(val);
        self
    }
}
impl Default for CaretStateChangedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
