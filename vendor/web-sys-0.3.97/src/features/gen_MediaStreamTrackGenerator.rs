#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MediaStreamTrack",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaStreamTrackGenerator",
        typescript_type = "MediaStreamTrackGenerator"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackGenerator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackGenerator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGenerator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaStreamTrackGenerator;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaStreamTrackGenerator",
        js_name = "writable"
    )]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackGenerator/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGenerator`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable(this: &MediaStreamTrackGenerator) -> WritableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MediaStreamTrackGenerator",
        js_name = "writable"
    )]
    #[doc = "Setter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackGenerator/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGenerator`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_writable(this: &MediaStreamTrackGenerator, value: &WritableStream);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaStreamTrackGeneratorInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamTrackGenerator")]
    #[doc = "The `new MediaStreamTrackGenerator(..)` constructor, creating a new instance of `MediaStreamTrackGenerator`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackGenerator/MediaStreamTrackGenerator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGenerator`, `MediaStreamTrackGeneratorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &MediaStreamTrackGeneratorInit) -> Result<MediaStreamTrackGenerator, JsValue>;
}
