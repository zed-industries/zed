#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "RTCDataChannelEvent",
        typescript_type = "RTCDataChannelEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEvent`*"]
    pub type RtcDataChannelEvent;
    #[cfg(feature = "RtcDataChannel")]
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannelEvent", js_name = "channel")]
    #[doc = "Getter for the `channel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/channel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEvent`*"]
    pub fn channel(this: &RtcDataChannelEvent) -> RtcDataChannel;
    #[cfg(feature = "RtcDataChannelEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCDataChannelEvent")]
    #[doc = "The `new RtcDataChannelEvent(..)` constructor, creating a new instance of `RtcDataChannelEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/RTCDataChannelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEvent`, `RtcDataChannelEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &RtcDataChannelEventInit,
    ) -> Result<RtcDataChannelEvent, JsValue>;
}
