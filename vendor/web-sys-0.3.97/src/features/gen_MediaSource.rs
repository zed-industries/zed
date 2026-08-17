#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaSource",
        typescript_type = "MediaSource"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSource` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub type MediaSource;
    #[cfg(feature = "SourceBufferList")]
    #[wasm_bindgen(method, getter, js_class = "MediaSource", js_name = "sourceBuffers")]
    #[doc = "Getter for the `sourceBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/sourceBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
    pub fn source_buffers(this: &MediaSource) -> SourceBufferList;
    #[cfg(feature = "SourceBufferList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaSource",
        js_name = "activeSourceBuffers"
    )]
    #[doc = "Getter for the `activeSourceBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/activeSourceBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
    pub fn active_source_buffers(this: &MediaSource) -> SourceBufferList;
    #[cfg(feature = "MediaSourceReadyState")]
    #[wasm_bindgen(method, getter, js_class = "MediaSource", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceReadyState`*"]
    pub fn ready_state(this: &MediaSource) -> MediaSourceReadyState;
    #[wasm_bindgen(method, getter, js_class = "MediaSource", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn duration(this: &MediaSource) -> f64;
    #[wasm_bindgen(method, setter, js_class = "MediaSource", js_name = "duration")]
    #[doc = "Setter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_duration(this: &MediaSource, value: f64);
    #[wasm_bindgen(method, getter, js_class = "MediaSource", js_name = "onsourceopen")]
    #[doc = "Getter for the `onsourceopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn onsourceopen(this: &MediaSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaSource", js_name = "onsourceopen")]
    #[doc = "Setter for the `onsourceopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_onsourceopen(this: &MediaSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaSource", js_name = "onsourceended")]
    #[doc = "Getter for the `onsourceended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn onsourceended(this: &MediaSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaSource", js_name = "onsourceended")]
    #[doc = "Setter for the `onsourceended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_onsourceended(this: &MediaSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaSource", js_name = "onsourceclose")]
    #[doc = "Getter for the `onsourceclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn onsourceclose(this: &MediaSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaSource", js_name = "onsourceclose")]
    #[doc = "Setter for the `onsourceclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_onsourceclose(this: &MediaSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "MediaSource")]
    #[doc = "The `new MediaSource(..)` constructor, creating a new instance of `MediaSource`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/MediaSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn new() -> Result<MediaSource, JsValue>;
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(catch, method, js_class = "MediaSource", js_name = "addSourceBuffer")]
    #[doc = "The `addSourceBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/addSourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
    pub fn add_source_buffer(this: &MediaSource, type_: &str) -> Result<SourceBuffer, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "MediaSource",
        js_name = "clearLiveSeekableRange"
    )]
    #[doc = "The `clearLiveSeekableRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/clearLiveSeekableRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn clear_live_seekable_range(this: &MediaSource) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaSource", js_name = "endOfStream")]
    #[doc = "The `endOfStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn end_of_stream(this: &MediaSource) -> Result<(), JsValue>;
    #[cfg(feature = "MediaSourceEndOfStreamError")]
    #[wasm_bindgen(catch, method, js_class = "MediaSource", js_name = "endOfStream")]
    #[doc = "The `endOfStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceEndOfStreamError`*"]
    pub fn end_of_stream_with_error(
        this: &MediaSource,
        error: MediaSourceEndOfStreamError,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        static_method_of = "MediaSource",
        js_class = "MediaSource",
        js_name = "isTypeSupported"
    )]
    #[doc = "The `isTypeSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/isTypeSupported_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn is_type_supported(type_: &str) -> bool;
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "MediaSource",
        js_name = "removeSourceBuffer"
    )]
    #[doc = "The `removeSourceBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/removeSourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
    pub fn remove_source_buffer(
        this: &MediaSource,
        source_buffer: &SourceBuffer,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "MediaSource",
        js_name = "setLiveSeekableRange"
    )]
    #[doc = "The `setLiveSeekableRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/setLiveSeekableRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_live_seekable_range(this: &MediaSource, start: f64, end: f64)
        -> Result<(), JsValue>;
}
