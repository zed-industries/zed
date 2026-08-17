#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "UADataValues")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UaDataValues` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UaDataValues;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `architecture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "architecture")]
    pub fn get_architecture(this: &UaDataValues) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `architecture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "architecture")]
    pub fn set_architecture(this: &UaDataValues, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bitness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bitness")]
    pub fn get_bitness(this: &UaDataValues) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bitness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bitness")]
    pub fn set_bitness(this: &UaDataValues, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[doc = "Get the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`, `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "brands")]
    pub fn get_brands(this: &UaDataValues) -> Option<::js_sys::Array<NavigatorUaBrandVersion>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[doc = "Change the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`, `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "brands")]
    pub fn set_brands(this: &UaDataValues, val: &[NavigatorUaBrandVersion]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `formFactors` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "formFactors")]
    pub fn get_form_factors(this: &UaDataValues) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `formFactors` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "formFactors")]
    pub fn set_form_factors(this: &UaDataValues, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[doc = "Get the `fullVersionList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`, `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "fullVersionList")]
    pub fn get_full_version_list(
        this: &UaDataValues,
    ) -> Option<::js_sys::Array<NavigatorUaBrandVersion>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[doc = "Change the `fullVersionList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`, `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "fullVersionList")]
    pub fn set_full_version_list(this: &UaDataValues, val: &[NavigatorUaBrandVersion]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mobile")]
    pub fn get_mobile(this: &UaDataValues) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mobile")]
    pub fn set_mobile(this: &UaDataValues, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `model` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "model")]
    pub fn get_model(this: &UaDataValues) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `model` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "model")]
    pub fn set_model(this: &UaDataValues, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "platform")]
    pub fn get_platform(this: &UaDataValues) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "platform")]
    pub fn set_platform(this: &UaDataValues, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `platformVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "platformVersion")]
    pub fn get_platform_version(this: &UaDataValues) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platformVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "platformVersion")]
    pub fn set_platform_version(this: &UaDataValues, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `wow64` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "wow64")]
    pub fn get_wow64(this: &UaDataValues) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `wow64` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "wow64")]
    pub fn set_wow64(this: &UaDataValues, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl UaDataValues {
    #[doc = "Construct a new `UaDataValues`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaDataValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_architecture()` instead."]
    pub fn architecture(&mut self, val: &str) -> &mut Self {
        self.set_architecture(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_bitness()` instead."]
    pub fn bitness(&mut self, val: &str) -> &mut Self {
        self.set_bitness(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[deprecated = "Use `set_brands()` instead."]
    pub fn brands(&mut self, val: &[NavigatorUaBrandVersion]) -> &mut Self {
        self.set_brands(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_form_factors()` instead."]
    pub fn form_factors(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_form_factors(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaBrandVersion")]
    #[deprecated = "Use `set_full_version_list()` instead."]
    pub fn full_version_list(&mut self, val: &[NavigatorUaBrandVersion]) -> &mut Self {
        self.set_full_version_list(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mobile()` instead."]
    pub fn mobile(&mut self, val: bool) -> &mut Self {
        self.set_mobile(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_model()` instead."]
    pub fn model(&mut self, val: &str) -> &mut Self {
        self.set_model(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_platform()` instead."]
    pub fn platform(&mut self, val: &str) -> &mut Self {
        self.set_platform(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_platform_version()` instead."]
    pub fn platform_version(&mut self, val: &str) -> &mut Self {
        self.set_platform_version(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_wow64()` instead."]
    pub fn wow64(&mut self, val: bool) -> &mut Self {
        self.set_wow64(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UaDataValues {
    fn default() -> Self {
        Self::new()
    }
}
