#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaMetadataInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaMetadataInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaMetadataInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `album` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "album")]
    pub fn get_album(this: &MediaMetadataInit) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `album` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "album")]
    pub fn set_album(this: &MediaMetadataInit, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `artist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "artist")]
    pub fn get_artist(this: &MediaMetadataInit) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `artist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "artist")]
    pub fn set_artist(this: &MediaMetadataInit, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaImage")]
    #[doc = "Get the `artwork` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`, `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "artwork")]
    pub fn get_artwork(this: &MediaMetadataInit) -> Option<::js_sys::Array<MediaImage>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaImage")]
    #[doc = "Change the `artwork` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`, `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "artwork")]
    pub fn set_artwork(this: &MediaMetadataInit, val: &[MediaImage]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &MediaMetadataInit) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &MediaMetadataInit, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl MediaMetadataInit {
    #[doc = "Construct a new `MediaMetadataInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_album()` instead."]
    pub fn album(&mut self, val: &str) -> &mut Self {
        self.set_album(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_artist()` instead."]
    pub fn artist(&mut self, val: &str) -> &mut Self {
        self.set_artist(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaImage")]
    #[deprecated = "Use `set_artwork()` instead."]
    pub fn artwork(&mut self, val: &[MediaImage]) -> &mut Self {
        self.set_artwork(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaMetadataInit {
    fn default() -> Self {
        Self::new()
    }
}
