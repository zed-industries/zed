#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SourceBuffer",
        typescript_type = "SourceBuffer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SourceBuffer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub type SourceBuffer;
    #[cfg(feature = "SourceBufferAppendMode")]
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "mode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
    pub fn mode(this: &SourceBuffer) -> SourceBufferAppendMode;
    #[cfg(feature = "SourceBufferAppendMode")]
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "mode")]
    #[doc = "Setter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
    pub fn set_mode(this: &SourceBuffer, value: SourceBufferAppendMode);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "updating")]
    #[doc = "Getter for the `updating` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/updating)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn updating(this: &SourceBuffer) -> bool;
    #[cfg(feature = "TimeRanges")]
    #[wasm_bindgen(catch, method, getter, js_class = "SourceBuffer", js_name = "buffered")]
    #[doc = "Getter for the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/buffered)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `TimeRanges`*"]
    pub fn buffered(this: &SourceBuffer) -> Result<TimeRanges, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "timestampOffset")]
    #[doc = "Getter for the `timestampOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn timestamp_offset(this: &SourceBuffer) -> f64;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "timestampOffset")]
    #[doc = "Setter for the `timestampOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_timestamp_offset(this: &SourceBuffer, value: f64);
    #[cfg(feature = "AudioTrackList")]
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "audioTracks")]
    #[doc = "Getter for the `audioTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/audioTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`, `SourceBuffer`*"]
    pub fn audio_tracks(this: &SourceBuffer) -> AudioTrackList;
    #[cfg(feature = "VideoTrackList")]
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "videoTracks")]
    #[doc = "Getter for the `videoTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/videoTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `VideoTrackList`*"]
    pub fn video_tracks(this: &SourceBuffer) -> VideoTrackList;
    #[cfg(feature = "TextTrackList")]
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "textTracks")]
    #[doc = "Getter for the `textTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/textTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `TextTrackList`*"]
    pub fn text_tracks(this: &SourceBuffer) -> TextTrackList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SourceBuffer",
        js_name = "appendWindowStart"
    )]
    #[doc = "Getter for the `appendWindowStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_window_start(this: &SourceBuffer) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SourceBuffer",
        js_name = "appendWindowStart"
    )]
    #[doc = "Setter for the `appendWindowStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_append_window_start(this: &SourceBuffer, value: f64);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "appendWindowEnd")]
    #[doc = "Getter for the `appendWindowEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_window_end(this: &SourceBuffer) -> f64;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "appendWindowEnd")]
    #[doc = "Setter for the `appendWindowEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_append_window_end(this: &SourceBuffer, value: f64);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "onupdatestart")]
    #[doc = "Getter for the `onupdatestart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onupdatestart(this: &SourceBuffer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "onupdatestart")]
    #[doc = "Setter for the `onupdatestart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onupdatestart(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "onupdate")]
    #[doc = "Getter for the `onupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onupdate(this: &SourceBuffer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "onupdate")]
    #[doc = "Setter for the `onupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onupdate(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "onupdateend")]
    #[doc = "Getter for the `onupdateend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onupdateend(this: &SourceBuffer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "onupdateend")]
    #[doc = "Setter for the `onupdateend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onupdateend(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onerror(this: &SourceBuffer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onerror(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SourceBuffer", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onabort(this: &SourceBuffer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SourceBuffer", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onabort(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn abort(this: &SourceBuffer) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer", js_name = "appendBuffer")]
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_array_buffer(
        this: &SourceBuffer,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer", js_name = "appendBuffer")]
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_array_buffer_view(
        this: &SourceBuffer,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer", js_name = "appendBuffer")]
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_u8_array(this: &SourceBuffer, data: &mut [u8])
        -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer", js_name = "appendBuffer")]
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_js_u8_array(
        this: &SourceBuffer,
        data: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SourceBuffer",
        js_name = "appendBufferAsync"
    )]
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_array_buffer(
        this: &SourceBuffer,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SourceBuffer",
        js_name = "appendBufferAsync"
    )]
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_array_buffer_view(
        this: &SourceBuffer,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SourceBuffer",
        js_name = "appendBufferAsync"
    )]
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_u8_array(
        this: &SourceBuffer,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SourceBuffer",
        js_name = "appendBufferAsync"
    )]
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_js_u8_array(
        this: &SourceBuffer,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer", js_name = "changeType")]
    #[doc = "The `changeType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/changeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn change_type(this: &SourceBuffer, type_: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn remove(this: &SourceBuffer, start: f64, end: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SourceBuffer", js_name = "removeAsync")]
    #[doc = "The `removeAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/removeAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn remove_async(
        this: &SourceBuffer,
        start: f64,
        end: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
}
