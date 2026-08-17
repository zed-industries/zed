#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PictureInPictureEvent",
        typescript_type = "PictureInPictureEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PictureInPictureEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PictureInPictureEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type PictureInPictureEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PictureInPictureWindow")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PictureInPictureEvent",
        js_name = "pictureInPictureWindow"
    )]
    #[doc = "Getter for the `pictureInPictureWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent/pictureInPictureWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PictureInPictureEvent`, `PictureInPictureWindow`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn picture_in_picture_window(this: &PictureInPictureEvent) -> PictureInPictureWindow;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PictureInPictureEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PictureInPictureEvent")]
    #[doc = "The `new PictureInPictureEvent(..)` constructor, creating a new instance of `PictureInPictureEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent/PictureInPictureEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PictureInPictureEvent`, `PictureInPictureEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &PictureInPictureEventInit,
    ) -> Result<PictureInPictureEvent, JsValue>;
}
