#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLMediaElement",
        typescript_type = "HTMLMediaElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMediaElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub type HtmlMediaElement;
    #[cfg(feature = "MediaError")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "error")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaError`*"]
    pub fn error(this: &HtmlMediaElement) -> Option<MediaError>;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn src(this: &HtmlMediaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_src(this: &HtmlMediaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "currentSrc")]
    #[doc = "Getter for the `currentSrc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentSrc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn current_src(this: &HtmlMediaElement) -> ::alloc::string::String;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "srcObject")]
    #[doc = "Getter for the `srcObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
    pub fn src_object(this: &HtmlMediaElement) -> Option<MediaStream>;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "srcObject")]
    #[doc = "Setter for the `srcObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
    pub fn set_src_object(this: &HtmlMediaElement, value: Option<&MediaStream>);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "crossOrigin")]
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn cross_origin(this: &HtmlMediaElement) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "crossOrigin")]
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_cross_origin(this: &HtmlMediaElement, value: Option<&str>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMediaElement",
        js_name = "networkState"
    )]
    #[doc = "Getter for the `networkState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/networkState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn network_state(this: &HtmlMediaElement) -> u16;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "preload")]
    #[doc = "Getter for the `preload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn preload(this: &HtmlMediaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "preload")]
    #[doc = "Setter for the `preload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_preload(this: &HtmlMediaElement, value: &str);
    #[cfg(feature = "TimeRanges")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "buffered")]
    #[doc = "Getter for the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    pub fn buffered(this: &HtmlMediaElement) -> TimeRanges;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn ready_state(this: &HtmlMediaElement) -> u16;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "seeking")]
    #[doc = "Getter for the `seeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn seeking(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "currentTime")]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn current_time(this: &HtmlMediaElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "currentTime")]
    #[doc = "Setter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_current_time(this: &HtmlMediaElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn duration(this: &HtmlMediaElement) -> f64;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "paused")]
    #[doc = "Getter for the `paused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/paused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn paused(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMediaElement",
        js_name = "defaultPlaybackRate"
    )]
    #[doc = "Getter for the `defaultPlaybackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn default_playback_rate(this: &HtmlMediaElement) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLMediaElement",
        js_name = "defaultPlaybackRate"
    )]
    #[doc = "Setter for the `defaultPlaybackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_default_playback_rate(this: &HtmlMediaElement, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMediaElement",
        js_name = "playbackRate"
    )]
    #[doc = "Getter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn playback_rate(this: &HtmlMediaElement) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLMediaElement",
        js_name = "playbackRate"
    )]
    #[doc = "Setter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_playback_rate(this: &HtmlMediaElement, value: f64);
    #[cfg(feature = "TimeRanges")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "played")]
    #[doc = "Getter for the `played` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/played)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    pub fn played(this: &HtmlMediaElement) -> TimeRanges;
    #[cfg(feature = "TimeRanges")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "seekable")]
    #[doc = "Getter for the `seekable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
    pub fn seekable(this: &HtmlMediaElement) -> TimeRanges;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "ended")]
    #[doc = "Getter for the `ended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn ended(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "autoplay")]
    #[doc = "Getter for the `autoplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn autoplay(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "autoplay")]
    #[doc = "Setter for the `autoplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_autoplay(this: &HtmlMediaElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "loop")]
    #[doc = "Getter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn loop_(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "loop")]
    #[doc = "Setter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_loop(this: &HtmlMediaElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "controls")]
    #[doc = "Getter for the `controls` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn controls(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "controls")]
    #[doc = "Setter for the `controls` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_controls(this: &HtmlMediaElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "volume")]
    #[doc = "Getter for the `volume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn volume(this: &HtmlMediaElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "volume")]
    #[doc = "Setter for the `volume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_volume(this: &HtmlMediaElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "muted")]
    #[doc = "Getter for the `muted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn muted(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "muted")]
    #[doc = "Setter for the `muted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_muted(this: &HtmlMediaElement, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMediaElement",
        js_name = "defaultMuted"
    )]
    #[doc = "Getter for the `defaultMuted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn default_muted(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLMediaElement",
        js_name = "defaultMuted"
    )]
    #[doc = "Setter for the `defaultMuted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_default_muted(this: &HtmlMediaElement, value: bool);
    #[cfg(feature = "AudioTrackList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "audioTracks")]
    #[doc = "Getter for the `audioTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`, `HtmlMediaElement`*"]
    pub fn audio_tracks(this: &HtmlMediaElement) -> AudioTrackList;
    #[cfg(feature = "VideoTrackList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "videoTracks")]
    #[doc = "Getter for the `videoTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/videoTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `VideoTrackList`*"]
    pub fn video_tracks(this: &HtmlMediaElement) -> VideoTrackList;
    #[cfg(feature = "TextTrackList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "textTracks")]
    #[doc = "Getter for the `textTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/textTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrackList`*"]
    pub fn text_tracks(this: &HtmlMediaElement) -> Option<TextTrackList>;
    #[cfg(feature = "MediaKeys")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "mediaKeys")]
    #[doc = "Getter for the `mediaKeys` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/mediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
    pub fn media_keys(this: &HtmlMediaElement) -> Option<MediaKeys>;
    #[wasm_bindgen(method, getter, js_class = "HTMLMediaElement", js_name = "onencrypted")]
    #[doc = "Getter for the `onencrypted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn onencrypted(this: &HtmlMediaElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLMediaElement", js_name = "onencrypted")]
    #[doc = "Setter for the `onencrypted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_onencrypted(this: &HtmlMediaElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLMediaElement",
        js_name = "onwaitingforkey"
    )]
    #[doc = "Getter for the `onwaitingforkey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn onwaitingforkey(this: &HtmlMediaElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLMediaElement",
        js_name = "onwaitingforkey"
    )]
    #[doc = "Setter for the `onwaitingforkey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_onwaitingforkey(this: &HtmlMediaElement, value: Option<&::js_sys::Function>);
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "addTextTrack")]
    #[doc = "The `addTextTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    pub fn add_text_track(this: &HtmlMediaElement, kind: TextTrackKind) -> TextTrack;
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "addTextTrack")]
    #[doc = "The `addTextTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    pub fn add_text_track_with_label(
        this: &HtmlMediaElement,
        kind: TextTrackKind,
        label: &str,
    ) -> TextTrack;
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "addTextTrack")]
    #[doc = "The `addTextTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
    pub fn add_text_track_with_label_and_language(
        this: &HtmlMediaElement,
        kind: TextTrackKind,
        label: &str,
        language: &str,
    ) -> TextTrack;
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "canPlayType")]
    #[doc = "The `canPlayType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canPlayType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn can_play_type(this: &HtmlMediaElement, type_: &str) -> ::alloc::string::String;
    #[wasm_bindgen(catch, method, js_class = "HTMLMediaElement", js_name = "fastSeek")]
    #[doc = "The `fastSeek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/fastSeek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn fast_seek(this: &HtmlMediaElement, time: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "hasSuspendTaint")]
    #[doc = "The `hasSuspendTaint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/hasSuspendTaint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn has_suspend_taint(this: &HtmlMediaElement) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLMediaElement")]
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn load(this: &HtmlMediaElement);
    #[wasm_bindgen(catch, method, js_class = "HTMLMediaElement")]
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn pause(this: &HtmlMediaElement) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLMediaElement")]
    #[doc = "The `play()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn play(this: &HtmlMediaElement) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLMediaElement",
        js_name = "seekToNextFrame"
    )]
    #[doc = "The `seekToNextFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekToNextFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn seek_to_next_frame(this: &HtmlMediaElement) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaKeys")]
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "setMediaKeys")]
    #[doc = "The `setMediaKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setMediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
    pub fn set_media_keys(
        this: &HtmlMediaElement,
        media_keys: Option<&MediaKeys>,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "HTMLMediaElement", js_name = "setVisible")]
    #[doc = "The `setVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub fn set_visible(this: &HtmlMediaElement, a_visible: bool);
}
impl HtmlMediaElement {
    #[doc = "The `HTMLMediaElement.NETWORK_EMPTY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_EMPTY: u16 = 0i64 as u16;
    #[doc = "The `HTMLMediaElement.NETWORK_IDLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_IDLE: u16 = 1u64 as u16;
    #[doc = "The `HTMLMediaElement.NETWORK_LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_LOADING: u16 = 2u64 as u16;
    #[doc = "The `HTMLMediaElement.NETWORK_NO_SOURCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_NO_SOURCE: u16 = 3u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_NOTHING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_NOTHING: u16 = 0i64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_METADATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_METADATA: u16 = 1u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_CURRENT_DATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_CURRENT_DATA: u16 = 2u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_FUTURE_DATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_FUTURE_DATA: u16 = 3u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_ENOUGH_DATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_ENOUGH_DATA: u16 = 4u64 as u16;
}
