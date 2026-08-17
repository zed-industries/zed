#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SaveFilePickerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SaveFilePickerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SaveFilePickerOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `excludeAcceptAllOption` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "excludeAcceptAllOption")]
    pub fn get_exclude_accept_all_option(this: &SaveFilePickerOptions) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `excludeAcceptAllOption` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "excludeAcceptAllOption")]
    pub fn set_exclude_accept_all_option(this: &SaveFilePickerOptions, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &SaveFilePickerOptions) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &SaveFilePickerOptions, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WellKnownDirectory")]
    #[doc = "Get the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`, `WellKnownDirectory`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "startIn")]
    pub fn get_start_in(this: &SaveFilePickerOptions) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WellKnownDirectory")]
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`, `WellKnownDirectory`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "startIn")]
    pub fn set_start_in(this: &SaveFilePickerOptions, val: WellKnownDirectory);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandle")]
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "startIn")]
    pub fn set_start_in_file_system_handle(this: &SaveFilePickerOptions, val: &FileSystemHandle);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FilePickerAcceptType")]
    #[doc = "Get the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`, `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "types")]
    pub fn get_types(this: &SaveFilePickerOptions)
        -> Option<::js_sys::Array<FilePickerAcceptType>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FilePickerAcceptType")]
    #[doc = "Change the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`, `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "types")]
    pub fn set_types(this: &SaveFilePickerOptions, val: &[FilePickerAcceptType]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `suggestedName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "suggestedName")]
    pub fn get_suggested_name(this: &SaveFilePickerOptions) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `suggestedName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "suggestedName")]
    pub fn set_suggested_name(this: &SaveFilePickerOptions, val: Option<&str>);
}
#[cfg(web_sys_unstable_apis)]
impl SaveFilePickerOptions {
    #[doc = "Construct a new `SaveFilePickerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_exclude_accept_all_option()` instead."]
    pub fn exclude_accept_all_option(&mut self, val: bool) -> &mut Self {
        self.set_exclude_accept_all_option(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WellKnownDirectory")]
    #[deprecated = "Use `set_start_in()` instead."]
    pub fn start_in(&mut self, val: WellKnownDirectory) -> &mut Self {
        self.set_start_in(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FilePickerAcceptType")]
    #[deprecated = "Use `set_types()` instead."]
    pub fn types(&mut self, val: &[FilePickerAcceptType]) -> &mut Self {
        self.set_types(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_suggested_name()` instead."]
    pub fn suggested_name(&mut self, val: Option<&str>) -> &mut Self {
        self.set_suggested_name(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SaveFilePickerOptions {
    fn default() -> Self {
        Self::new()
    }
}
