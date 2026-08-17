#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaRecorder",
        typescript_type = "MediaRecorder"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub type MediaRecorder;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "stream")]
    #[doc = "Getter for the `stream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
    pub fn stream(this: &MediaRecorder) -> MediaStream;
    #[cfg(feature = "RecordingState")]
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `RecordingState`*"]
    pub fn state(this: &MediaRecorder) -> RecordingState;
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "mimeType")]
    #[doc = "Getter for the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/mimeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn mime_type(this: &MediaRecorder) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaRecorder",
        js_name = "ondataavailable"
    )]
    #[doc = "Getter for the `ondataavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn ondataavailable(this: &MediaRecorder) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "MediaRecorder",
        js_name = "ondataavailable"
    )]
    #[doc = "Setter for the `ondataavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_ondataavailable(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onerror(this: &MediaRecorder) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaRecorder", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onerror(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "onpause")]
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onpause(this: &MediaRecorder) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaRecorder", js_name = "onpause")]
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onpause(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "onresume")]
    #[doc = "Getter for the `onresume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onresume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onresume(this: &MediaRecorder) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaRecorder", js_name = "onresume")]
    #[doc = "Setter for the `onresume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onresume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onresume(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "onstart")]
    #[doc = "Getter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onstart(this: &MediaRecorder) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaRecorder", js_name = "onstart")]
    #[doc = "Setter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onstart(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaRecorder", js_name = "onstop")]
    #[doc = "Getter for the `onstop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onstop(this: &MediaRecorder) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaRecorder", js_name = "onstop")]
    #[doc = "Setter for the `onstop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onstop(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaRecorder",
        js_name = "videoBitsPerSecond"
    )]
    #[doc = "Getter for the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/videoBitsPerSecond)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn video_bits_per_second(this: &MediaRecorder) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaRecorder",
        js_name = "audioBitsPerSecond"
    )]
    #[doc = "Getter for the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/audioBitsPerSecond)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn audio_bits_per_second(this: &MediaRecorder) -> u32;
    #[cfg(feature = "BitrateMode")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaRecorder",
        js_name = "audioBitrateMode"
    )]
    #[doc = "Getter for the `audioBitrateMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/audioBitrateMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BitrateMode`, `MediaRecorder`*"]
    pub fn audio_bitrate_mode(this: &MediaRecorder) -> BitrateMode;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
    pub fn new_with_media_stream(stream: &MediaStream) -> Result<MediaRecorder, JsValue>;
    #[cfg(all(feature = "MediaRecorderOptions", feature = "MediaStream",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `MediaRecorderOptions`, `MediaStream`*"]
    pub fn new_with_media_stream_and_media_recorder_options(
        stream: &MediaStream,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, JsValue>;
    #[cfg(feature = "AudioNode")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
    pub fn new_with_audio_node(node: &AudioNode) -> Result<MediaRecorder, JsValue>;
    #[cfg(feature = "AudioNode")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
    pub fn new_with_audio_node_and_u32(
        node: &AudioNode,
        output: u32,
    ) -> Result<MediaRecorder, JsValue>;
    #[cfg(all(feature = "AudioNode", feature = "MediaRecorderOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`, `MediaRecorderOptions`*"]
    pub fn new_with_audio_node_and_u32_and_options(
        node: &AudioNode,
        output: u32,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, JsValue>;
    #[wasm_bindgen(
        static_method_of = "MediaRecorder",
        js_class = "MediaRecorder",
        js_name = "isTypeSupported"
    )]
    #[doc = "The `isTypeSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/isTypeSupported_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn is_type_supported(type_: &str) -> bool;
    #[wasm_bindgen(catch, method, js_class = "MediaRecorder")]
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn pause(this: &MediaRecorder) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaRecorder", js_name = "requestData")]
    #[doc = "The `requestData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/requestData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn request_data(this: &MediaRecorder) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaRecorder")]
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn resume(this: &MediaRecorder) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaRecorder")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn start(this: &MediaRecorder) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaRecorder", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn start_with_time_slice(this: &MediaRecorder, time_slice: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaRecorder")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn stop(this: &MediaRecorder) -> Result<(), JsValue>;
}
