#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PhotoSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PhotoSettings` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type PhotoSettings;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FillLightMode")]
    #[doc = "Get the `fillLightMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillLightMode`, `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "fillLightMode")]
    pub fn get_fill_light_mode(this: &PhotoSettings) -> Option<FillLightMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FillLightMode")]
    #[doc = "Change the `fillLightMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillLightMode`, `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "fillLightMode")]
    pub fn set_fill_light_mode(this: &PhotoSettings, val: FillLightMode);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `imageHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "imageHeight")]
    pub fn get_image_height(this: &PhotoSettings) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `imageHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "imageHeight")]
    pub fn set_image_height(this: &PhotoSettings, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `imageWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "imageWidth")]
    pub fn get_image_width(this: &PhotoSettings) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `imageWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "imageWidth")]
    pub fn set_image_width(this: &PhotoSettings, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `redEyeReduction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "redEyeReduction")]
    pub fn get_red_eye_reduction(this: &PhotoSettings) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `redEyeReduction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "redEyeReduction")]
    pub fn set_red_eye_reduction(this: &PhotoSettings, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl PhotoSettings {
    #[doc = "Construct a new `PhotoSettings`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PhotoSettings`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FillLightMode")]
    #[deprecated = "Use `set_fill_light_mode()` instead."]
    pub fn fill_light_mode(&mut self, val: FillLightMode) -> &mut Self {
        self.set_fill_light_mode(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_image_height()` instead."]
    pub fn image_height(&mut self, val: f64) -> &mut Self {
        self.set_image_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_image_width()` instead."]
    pub fn image_width(&mut self, val: f64) -> &mut Self {
        self.set_image_width(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_red_eye_reduction()` instead."]
    pub fn red_eye_reduction(&mut self, val: bool) -> &mut Self {
        self.set_red_eye_reduction(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for PhotoSettings {
    fn default() -> Self {
        Self::new()
    }
}
