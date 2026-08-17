#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AutocompleteInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AutocompleteInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub type AutocompleteInfo;
    #[doc = "Get the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, getter = "addressType")]
    pub fn get_address_type(this: &AutocompleteInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, setter = "addressType")]
    pub fn set_address_type(this: &AutocompleteInfo, val: &str);
    #[doc = "Get the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, getter = "contactType")]
    pub fn get_contact_type(this: &AutocompleteInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, setter = "contactType")]
    pub fn set_contact_type(this: &AutocompleteInfo, val: &str);
    #[doc = "Get the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, getter = "fieldName")]
    pub fn get_field_name(this: &AutocompleteInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, setter = "fieldName")]
    pub fn set_field_name(this: &AutocompleteInfo, val: &str);
    #[doc = "Get the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, getter = "section")]
    pub fn get_section(this: &AutocompleteInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    #[wasm_bindgen(method, setter = "section")]
    pub fn set_section(this: &AutocompleteInfo, val: &str);
}
impl AutocompleteInfo {
    #[doc = "Construct a new `AutocompleteInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_address_type()` instead."]
    pub fn address_type(&mut self, val: &str) -> &mut Self {
        self.set_address_type(val);
        self
    }
    #[deprecated = "Use `set_contact_type()` instead."]
    pub fn contact_type(&mut self, val: &str) -> &mut Self {
        self.set_contact_type(val);
        self
    }
    #[deprecated = "Use `set_field_name()` instead."]
    pub fn field_name(&mut self, val: &str) -> &mut Self {
        self.set_field_name(val);
        self
    }
    #[deprecated = "Use `set_section()` instead."]
    pub fn section(&mut self, val: &str) -> &mut Self {
        self.set_section(val);
        self
    }
}
impl Default for AutocompleteInfo {
    fn default() -> Self {
        Self::new()
    }
}
