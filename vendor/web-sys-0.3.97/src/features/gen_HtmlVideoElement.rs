#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlMediaElement",
        extends = "HtmlElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLVideoElement",
        typescript_type = "HTMLVideoElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlVideoElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub type HtmlVideoElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLVideoElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn width(this: &HtmlVideoElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLVideoElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn set_width(this: &HtmlVideoElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLVideoElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn height(this: &HtmlVideoElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLVideoElement", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn set_height(this: &HtmlVideoElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLVideoElement", js_name = "videoWidth")]
    #[doc = "Getter for the `videoWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn video_width(this: &HtmlVideoElement) -> u32;
    #[wasm_bindgen(method, getter, js_class = "HTMLVideoElement", js_name = "videoHeight")]
    #[doc = "Getter for the `videoHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/videoHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn video_height(this: &HtmlVideoElement) -> u32;
    #[wasm_bindgen(method, getter, js_class = "HTMLVideoElement", js_name = "poster")]
    #[doc = "Getter for the `poster` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn poster(this: &HtmlVideoElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLVideoElement", js_name = "poster")]
    #[doc = "Setter for the `poster` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/poster)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    pub fn set_poster(this: &HtmlVideoElement, value: &str);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLVideoElement",
        js_name = "onenterpictureinpicture"
    )]
    #[doc = "Getter for the `onenterpictureinpicture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onenterpictureinpicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onenterpictureinpicture(this: &HtmlVideoElement) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLVideoElement",
        js_name = "onenterpictureinpicture"
    )]
    #[doc = "Setter for the `onenterpictureinpicture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onenterpictureinpicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onenterpictureinpicture(this: &HtmlVideoElement, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLVideoElement",
        js_name = "onleavepictureinpicture"
    )]
    #[doc = "Getter for the `onleavepictureinpicture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onleavepictureinpicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onleavepictureinpicture(this: &HtmlVideoElement) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLVideoElement",
        js_name = "onleavepictureinpicture"
    )]
    #[doc = "Setter for the `onleavepictureinpicture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/onleavepictureinpicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onleavepictureinpicture(this: &HtmlVideoElement, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLVideoElement",
        js_name = "disablePictureInPicture"
    )]
    #[doc = "Getter for the `disablePictureInPicture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/disablePictureInPicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn disable_picture_in_picture(this: &HtmlVideoElement) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLVideoElement",
        js_name = "disablePictureInPicture"
    )]
    #[doc = "Setter for the `disablePictureInPicture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/disablePictureInPicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_disable_picture_in_picture(this: &HtmlVideoElement, value: bool);
    #[cfg(feature = "VideoPlaybackQuality")]
    #[wasm_bindgen(
        method,
        js_class = "HTMLVideoElement",
        js_name = "getVideoPlaybackQuality"
    )]
    #[doc = "The `getVideoPlaybackQuality()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/getVideoPlaybackQuality)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoPlaybackQuality`*"]
    pub fn get_video_playback_quality(this: &HtmlVideoElement) -> VideoPlaybackQuality;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PictureInPictureWindow")]
    #[wasm_bindgen(
        method,
        js_class = "HTMLVideoElement",
        js_name = "requestPictureInPicture"
    )]
    #[doc = "The `requestPictureInPicture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement/requestPictureInPicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `PictureInPictureWindow`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_picture_in_picture(
        this: &HtmlVideoElement,
    ) -> ::js_sys::Promise<PictureInPictureWindow>;
}
