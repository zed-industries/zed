#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoFrameInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrameInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    pub type VideoFrameInit;
    #[cfg(feature = "AlphaOption")]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoFrameInit`*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &VideoFrameInit) -> Option<AlphaOption>;
    #[cfg(feature = "AlphaOption")]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "alpha")]
    pub fn set_alpha(this: &VideoFrameInit, val: AlphaOption);
    #[doc = "Get the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, getter = "displayHeight")]
    pub fn get_display_height(this: &VideoFrameInit) -> Option<u32>;
    #[doc = "Change the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "displayHeight")]
    pub fn set_display_height(this: &VideoFrameInit, val: u32);
    #[doc = "Get the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, getter = "displayWidth")]
    pub fn get_display_width(this: &VideoFrameInit) -> Option<u32>;
    #[doc = "Change the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "displayWidth")]
    pub fn set_display_width(this: &VideoFrameInit, val: u32);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &VideoFrameInit) -> Option<f64>;
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &VideoFrameInit, val: u32);
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration_f64(this: &VideoFrameInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `flip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "flip")]
    pub fn get_flip(this: &VideoFrameInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `flip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "flip")]
    pub fn set_flip(this: &VideoFrameInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[doc = "Get the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`, `VideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "metadata")]
    pub fn get_metadata(this: &VideoFrameInit) -> Option<VideoFrameMetadata>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[doc = "Change the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`, `VideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "metadata")]
    pub fn set_metadata(this: &VideoFrameInit, val: &VideoFrameMetadata);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `rotation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "rotation")]
    pub fn get_rotation(this: &VideoFrameInit) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `rotation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "rotation")]
    pub fn set_rotation(this: &VideoFrameInit, val: f64);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &VideoFrameInit) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &VideoFrameInit, val: i32);
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_f64(this: &VideoFrameInit, val: f64);
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameInit`*"]
    #[wasm_bindgen(method, getter = "visibleRect")]
    pub fn get_visible_rect(this: &VideoFrameInit) -> Option<DomRectInit>;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameInit`*"]
    #[wasm_bindgen(method, setter = "visibleRect")]
    pub fn set_visible_rect(this: &VideoFrameInit, val: &DomRectInit);
}
impl VideoFrameInit {
    #[doc = "Construct a new `VideoFrameInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AlphaOption")]
    #[deprecated = "Use `set_alpha()` instead."]
    pub fn alpha(&mut self, val: AlphaOption) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[deprecated = "Use `set_display_height()` instead."]
    pub fn display_height(&mut self, val: u32) -> &mut Self {
        self.set_display_height(val);
        self
    }
    #[deprecated = "Use `set_display_width()` instead."]
    pub fn display_width(&mut self, val: u32) -> &mut Self {
        self.set_display_width(val);
        self
    }
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: u32) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_flip()` instead."]
    pub fn flip(&mut self, val: bool) -> &mut Self {
        self.set_flip(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameMetadata")]
    #[deprecated = "Use `set_metadata()` instead."]
    pub fn metadata(&mut self, val: &VideoFrameMetadata) -> &mut Self {
        self.set_metadata(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_rotation()` instead."]
    pub fn rotation(&mut self, val: f64) -> &mut Self {
        self.set_rotation(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: i32) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[deprecated = "Use `set_visible_rect()` instead."]
    pub fn visible_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_visible_rect(val);
        self
    }
}
impl Default for VideoFrameInit {
    fn default() -> Self {
        Self::new()
    }
}
