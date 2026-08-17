#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FilePickerAcceptType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePickerAcceptType` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type FilePickerAcceptType;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `accept` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "accept")]
    pub fn get_accept(this: &FilePickerAcceptType) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `accept` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "accept")]
    pub fn set_accept(this: &FilePickerAcceptType, val: &::js_sys::Object<::js_sys::JsString>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `accept` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "accept")]
    pub fn set_accept_record_from_str_to_str_sequence(
        this: &FilePickerAcceptType,
        val: &::js_sys::Object<::js_sys::Array<::js_sys::JsString>>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &FilePickerAcceptType) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &FilePickerAcceptType, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl FilePickerAcceptType {
    #[doc = "Construct a new `FilePickerAcceptType`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_accept()` instead."]
    pub fn accept(&mut self, val: &::js_sys::Object<::js_sys::JsString>) -> &mut Self {
        self.set_accept(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.set_description(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for FilePickerAcceptType {
    fn default() -> Self {
        Self::new()
    }
}
