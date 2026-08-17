#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "InputEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub type InputEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &InputEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &InputEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &InputEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &InputEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &InputEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &InputEventInit, val: bool);
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "detail")]
    pub fn get_detail(this: &InputEventInit) -> Option<i32>;
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "detail")]
    pub fn set_detail(this: &InputEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
    #[wasm_bindgen(method, getter = "view")]
    pub fn get_view(this: &InputEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
    #[wasm_bindgen(method, setter = "view")]
    pub fn set_view(this: &InputEventInit, val: Option<&Window>);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &InputEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &InputEventInit, val: Option<&str>);
    #[cfg(feature = "DataTransfer")]
    #[doc = "Get the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "dataTransfer")]
    pub fn get_data_transfer(this: &InputEventInit) -> Option<DataTransfer>;
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "dataTransfer")]
    pub fn set_data_transfer(this: &InputEventInit, val: Option<&DataTransfer>);
    #[doc = "Get the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "inputType")]
    pub fn get_input_type(this: &InputEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "inputType")]
    pub fn set_input_type(this: &InputEventInit, val: &str);
    #[doc = "Get the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "isComposing")]
    pub fn get_is_composing(this: &InputEventInit) -> Option<bool>;
    #[doc = "Change the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "isComposing")]
    pub fn set_is_composing(this: &InputEventInit, val: bool);
    #[doc = "Get the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, getter = "targetRanges")]
    pub fn get_target_ranges(this: &InputEventInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    #[wasm_bindgen(method, setter = "targetRanges")]
    pub fn set_target_ranges(this: &InputEventInit, val: &::wasm_bindgen::JsValue);
}
impl InputEventInit {
    #[doc = "Construct a new `InputEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
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
    #[deprecated = "Use `set_detail()` instead."]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail(val);
        self
    }
    #[cfg(feature = "Window")]
    #[deprecated = "Use `set_view()` instead."]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: Option<&str>) -> &mut Self {
        self.set_data(val);
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[deprecated = "Use `set_data_transfer()` instead."]
    pub fn data_transfer(&mut self, val: Option<&DataTransfer>) -> &mut Self {
        self.set_data_transfer(val);
        self
    }
    #[deprecated = "Use `set_input_type()` instead."]
    pub fn input_type(&mut self, val: &str) -> &mut Self {
        self.set_input_type(val);
        self
    }
    #[deprecated = "Use `set_is_composing()` instead."]
    pub fn is_composing(&mut self, val: bool) -> &mut Self {
        self.set_is_composing(val);
        self
    }
    #[deprecated = "Use `set_target_ranges()` instead."]
    pub fn target_ranges(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_target_ranges(val);
        self
    }
}
impl Default for InputEventInit {
    fn default() -> Self {
        Self::new()
    }
}
