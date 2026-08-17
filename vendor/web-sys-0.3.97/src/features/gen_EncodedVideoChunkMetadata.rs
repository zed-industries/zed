#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "EncodedVideoChunkMetadata")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedVideoChunkMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedVideoChunkMetadata;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "alphaSideData")]
    pub fn get_alpha_side_data(this: &EncodedVideoChunkMetadata) -> Option<::js_sys::Object>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "alphaSideData")]
    pub fn set_alpha_side_data(this: &EncodedVideoChunkMetadata, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "The `val` slice must outlive any use of the dictionary on the JavaScript side. wasm-bindgen passes a raw view into wasm linear memory — if the underlying data is freed or moved before JS is done reading the field, the behaviour is undefined. Prefer the `_array` variant (takes `&Uint8Array`) for a safe alternative that copies the data."]
    #[wasm_bindgen(method, setter = "alphaSideData")]
    pub unsafe fn set_alpha_side_data_u8_slice(this: &EncodedVideoChunkMetadata, val: &mut [u8]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "alphaSideData")]
    pub fn set_alpha_side_data_u8_array(
        this: &EncodedVideoChunkMetadata,
        val: &::js_sys::Uint8Array,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Get the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "decoderConfig")]
    pub fn get_decoder_config(this: &EncodedVideoChunkMetadata) -> Option<VideoDecoderConfig>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Change the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "decoderConfig")]
    pub fn set_decoder_config(this: &EncodedVideoChunkMetadata, val: &VideoDecoderConfig);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[doc = "Get the `svc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "svc")]
    pub fn get_svc(this: &EncodedVideoChunkMetadata) -> Option<SvcOutputMetadata>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[doc = "Change the `svc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "svc")]
    pub fn set_svc(this: &EncodedVideoChunkMetadata, val: &SvcOutputMetadata);
}
#[cfg(web_sys_unstable_apis)]
impl EncodedVideoChunkMetadata {
    #[doc = "Construct a new `EncodedVideoChunkMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_alpha_side_data()` instead."]
    pub fn alpha_side_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_alpha_side_data(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[deprecated = "Use `set_decoder_config()` instead."]
    pub fn decoder_config(&mut self, val: &VideoDecoderConfig) -> &mut Self {
        self.set_decoder_config(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[deprecated = "Use `set_svc()` instead."]
    pub fn svc(&mut self, val: &SvcOutputMetadata) -> &mut Self {
        self.set_svc(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for EncodedVideoChunkMetadata {
    fn default() -> Self {
        Self::new()
    }
}
