#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "L10nElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub type L10nElement;
    #[doc = "Get the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "l10nArgs")]
    pub fn get_l10n_args(this: &L10nElement) -> Option<::js_sys::Object>;
    #[doc = "Change the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, setter = "l10nArgs")]
    pub fn set_l10n_args(this: &L10nElement, val: Option<&::js_sys::Object>);
    #[doc = "Get the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "l10nAttrs")]
    pub fn get_l10n_attrs(this: &L10nElement) -> Option<::alloc::string::String>;
    #[doc = "Change the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, setter = "l10nAttrs")]
    pub fn set_l10n_attrs(this: &L10nElement, val: Option<&str>);
    #[doc = "Get the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "l10nId")]
    pub fn get_l10n_id(this: &L10nElement) -> ::alloc::string::String;
    #[doc = "Change the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, setter = "l10nId")]
    pub fn set_l10n_id(this: &L10nElement, val: &str);
    #[doc = "Get the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "localName")]
    pub fn get_local_name(this: &L10nElement) -> ::alloc::string::String;
    #[doc = "Change the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, setter = "localName")]
    pub fn set_local_name(this: &L10nElement, val: &str);
    #[doc = "Get the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "namespaceURI")]
    pub fn get_namespace_uri(this: &L10nElement) -> ::alloc::string::String;
    #[doc = "Change the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, setter = "namespaceURI")]
    pub fn set_namespace_uri(this: &L10nElement, val: &str);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &L10nElement) -> Option<::alloc::string::String>;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &L10nElement, val: Option<&str>);
}
impl L10nElement {
    #[doc = "Construct a new `L10nElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn new(l10n_id: &str, local_name: &str, namespace_uri: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_l10n_id(l10n_id);
        ret.set_local_name(local_name);
        ret.set_namespace_uri(namespace_uri);
        ret
    }
    #[deprecated = "Use `set_l10n_args()` instead."]
    pub fn l10n_args(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_l10n_args(val);
        self
    }
    #[deprecated = "Use `set_l10n_attrs()` instead."]
    pub fn l10n_attrs(&mut self, val: Option<&str>) -> &mut Self {
        self.set_l10n_attrs(val);
        self
    }
    #[deprecated = "Use `set_l10n_id()` instead."]
    pub fn l10n_id(&mut self, val: &str) -> &mut Self {
        self.set_l10n_id(val);
        self
    }
    #[deprecated = "Use `set_local_name()` instead."]
    pub fn local_name(&mut self, val: &str) -> &mut Self {
        self.set_local_name(val);
        self
    }
    #[deprecated = "Use `set_namespace_uri()` instead."]
    pub fn namespace_uri(&mut self, val: &str) -> &mut Self {
        self.set_namespace_uri(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: Option<&str>) -> &mut Self {
        self.set_type(val);
        self
    }
}
