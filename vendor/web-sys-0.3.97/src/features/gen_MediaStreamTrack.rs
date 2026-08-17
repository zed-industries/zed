#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaStreamTrack",
        typescript_type = "MediaStreamTrack"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub type MediaStreamTrack;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn kind(this: &MediaStreamTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn id(this: &MediaStreamTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn label(this: &MediaStreamTrack) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "enabled")]
    #[doc = "Getter for the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn enabled(this: &MediaStreamTrack) -> bool;
    #[wasm_bindgen(method, setter, js_class = "MediaStreamTrack", js_name = "enabled")]
    #[doc = "Setter for the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn set_enabled(this: &MediaStreamTrack, value: bool);
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "muted")]
    #[doc = "Getter for the `muted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/muted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn muted(this: &MediaStreamTrack) -> bool;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "onmute")]
    #[doc = "Getter for the `onmute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn onmute(this: &MediaStreamTrack) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaStreamTrack", js_name = "onmute")]
    #[doc = "Setter for the `onmute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn set_onmute(this: &MediaStreamTrack, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "onunmute")]
    #[doc = "Getter for the `onunmute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn onunmute(this: &MediaStreamTrack) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaStreamTrack", js_name = "onunmute")]
    #[doc = "Setter for the `onunmute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn set_onunmute(this: &MediaStreamTrack, value: Option<&::js_sys::Function>);
    #[cfg(feature = "MediaStreamTrackState")]
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackState`*"]
    pub fn ready_state(this: &MediaStreamTrack) -> MediaStreamTrackState;
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrack", js_name = "onended")]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn onended(this: &MediaStreamTrack) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaStreamTrack", js_name = "onended")]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn set_onended(this: &MediaStreamTrack, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "MediaStreamTrack",
        js_name = "applyConstraints"
    )]
    #[doc = "The `applyConstraints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn apply_constraints(this: &MediaStreamTrack) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaTrackConstraints")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "MediaStreamTrack",
        js_name = "applyConstraints"
    )]
    #[doc = "The `applyConstraints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackConstraints`*"]
    pub fn apply_constraints_with_constraints(
        this: &MediaStreamTrack,
        constraints: &MediaTrackConstraints,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(method, js_class = "MediaStreamTrack")]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn clone(this: &MediaStreamTrack) -> MediaStreamTrack;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaTrackCapabilities")]
    #[wasm_bindgen(method, js_class = "MediaStreamTrack", js_name = "getCapabilities")]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_capabilities(this: &MediaStreamTrack) -> MediaTrackCapabilities;
    #[cfg(feature = "MediaTrackConstraints")]
    #[wasm_bindgen(method, js_class = "MediaStreamTrack", js_name = "getConstraints")]
    #[doc = "The `getConstraints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getConstraints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackConstraints`*"]
    pub fn get_constraints(this: &MediaStreamTrack) -> MediaTrackConstraints;
    #[cfg(feature = "MediaTrackSettings")]
    #[wasm_bindgen(method, js_class = "MediaStreamTrack", js_name = "getSettings")]
    #[doc = "The `getSettings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getSettings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackSettings`*"]
    pub fn get_settings(this: &MediaStreamTrack) -> MediaTrackSettings;
    #[wasm_bindgen(method, js_class = "MediaStreamTrack")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    pub fn stop(this: &MediaStreamTrack);
}
