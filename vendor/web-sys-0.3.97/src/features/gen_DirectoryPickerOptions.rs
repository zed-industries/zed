#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DirectoryPickerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DirectoryPickerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type DirectoryPickerOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &DirectoryPickerOptions) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &DirectoryPickerOptions, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemPermissionMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `FileSystemPermissionMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &DirectoryPickerOptions) -> Option<FileSystemPermissionMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemPermissionMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `FileSystemPermissionMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &DirectoryPickerOptions, val: FileSystemPermissionMode);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WellKnownDirectory")]
    #[doc = "Get the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `WellKnownDirectory`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "startIn")]
    pub fn get_start_in(this: &DirectoryPickerOptions) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WellKnownDirectory")]
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `WellKnownDirectory`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "startIn")]
    pub fn set_start_in(this: &DirectoryPickerOptions, val: WellKnownDirectory);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandle")]
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `FileSystemHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "startIn")]
    pub fn set_start_in_file_system_handle(this: &DirectoryPickerOptions, val: &FileSystemHandle);
}
#[cfg(web_sys_unstable_apis)]
impl DirectoryPickerOptions {
    #[doc = "Construct a new `DirectoryPickerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemPermissionMode")]
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: FileSystemPermissionMode) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WellKnownDirectory")]
    #[deprecated = "Use `set_start_in()` instead."]
    pub fn start_in(&mut self, val: WellKnownDirectory) -> &mut Self {
        self.set_start_in(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for DirectoryPickerOptions {
    fn default() -> Self {
        Self::new()
    }
}
