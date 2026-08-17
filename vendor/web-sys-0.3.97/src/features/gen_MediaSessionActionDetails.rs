#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaSessionActionDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSessionActionDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaSessionActionDetails;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &MediaSessionActionDetails) -> MediaSessionAction;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "action")]
    pub fn set_action(this: &MediaSessionActionDetails, val: MediaSessionAction);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `fastSeek` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "fastSeek")]
    pub fn get_fast_seek(this: &MediaSessionActionDetails) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fastSeek` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "fastSeek")]
    pub fn set_fast_seek(this: &MediaSessionActionDetails, val: Option<bool>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `seekOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "seekOffset")]
    pub fn get_seek_offset(this: &MediaSessionActionDetails) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `seekOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "seekOffset")]
    pub fn set_seek_offset(this: &MediaSessionActionDetails, val: Option<f64>);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `seekTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "seekTime")]
    pub fn get_seek_time(this: &MediaSessionActionDetails) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `seekTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "seekTime")]
    pub fn set_seek_time(this: &MediaSessionActionDetails, val: Option<f64>);
}
#[cfg(web_sys_unstable_apis)]
impl MediaSessionActionDetails {
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Construct a new `MediaSessionActionDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(action: MediaSessionAction) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_action(action);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[deprecated = "Use `set_action()` instead."]
    pub fn action(&mut self, val: MediaSessionAction) -> &mut Self {
        self.set_action(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_fast_seek()` instead."]
    pub fn fast_seek(&mut self, val: Option<bool>) -> &mut Self {
        self.set_fast_seek(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_seek_offset()` instead."]
    pub fn seek_offset(&mut self, val: Option<f64>) -> &mut Self {
        self.set_seek_offset(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_seek_time()` instead."]
    pub fn seek_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_seek_time(val);
        self
    }
}
