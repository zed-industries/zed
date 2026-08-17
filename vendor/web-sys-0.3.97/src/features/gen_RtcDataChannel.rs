#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "RTCDataChannel",
        typescript_type = "RTCDataChannel"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub type RtcDataChannel;
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn label(this: &RtcDataChannel) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn id(this: &RtcDataChannel) -> Option<u16>;
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "reliable")]
    #[doc = "Getter for the `reliable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/reliable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn reliable(this: &RtcDataChannel) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCDataChannel",
        js_name = "maxPacketLifeTime"
    )]
    #[doc = "Getter for the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxPacketLifeTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn max_packet_life_time(this: &RtcDataChannel) -> Option<u16>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCDataChannel",
        js_name = "maxRetransmits"
    )]
    #[doc = "Getter for the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxRetransmits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn max_retransmits(this: &RtcDataChannel) -> Option<u16>;
    #[cfg(feature = "RtcDataChannelState")]
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelState`*"]
    pub fn ready_state(this: &RtcDataChannel) -> RtcDataChannelState;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCDataChannel",
        js_name = "bufferedAmount"
    )]
    #[doc = "Getter for the `bufferedAmount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn buffered_amount(this: &RtcDataChannel) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCDataChannel",
        js_name = "bufferedAmountLowThreshold"
    )]
    #[doc = "Getter for the `bufferedAmountLowThreshold` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn buffered_amount_low_threshold(this: &RtcDataChannel) -> u32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCDataChannel",
        js_name = "bufferedAmountLowThreshold"
    )]
    #[doc = "Setter for the `bufferedAmountLowThreshold` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_buffered_amount_low_threshold(this: &RtcDataChannel, value: u32);
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "onopen")]
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onopen(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCDataChannel", js_name = "onopen")]
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onopen(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onerror(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCDataChannel", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onerror(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onclose(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCDataChannel", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onclose(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "onmessage")]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onmessage(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCDataChannel", js_name = "onmessage")]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onmessage(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCDataChannel",
        js_name = "onbufferedamountlow"
    )]
    #[doc = "Getter for the `onbufferedamountlow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onbufferedamountlow(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "RTCDataChannel",
        js_name = "onbufferedamountlow"
    )]
    #[doc = "Setter for the `onbufferedamountlow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onbufferedamountlow(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    #[cfg(feature = "RtcDataChannelType")]
    #[wasm_bindgen(method, getter, js_class = "RTCDataChannel", js_name = "binaryType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
    pub fn binary_type(this: &RtcDataChannel) -> RtcDataChannelType;
    #[cfg(feature = "RtcDataChannelType")]
    #[wasm_bindgen(method, setter, js_class = "RTCDataChannel", js_name = "binaryType")]
    #[doc = "Setter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
    pub fn set_binary_type(this: &RtcDataChannel, value: RtcDataChannelType);
    #[wasm_bindgen(method, js_class = "RTCDataChannel")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn close(this: &RtcDataChannel);
    #[wasm_bindgen(catch, method, js_class = "RTCDataChannel", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_str(this: &RtcDataChannel, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "RTCDataChannel", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `RtcDataChannel`*"]
    pub fn send_with_blob(this: &RtcDataChannel, data: &Blob) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "RTCDataChannel", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_array_buffer(
        this: &RtcDataChannel,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "RTCDataChannel", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_array_buffer_view(
        this: &RtcDataChannel,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "RTCDataChannel", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_u8_array(this: &RtcDataChannel, data: &[u8]) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "RTCDataChannel", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_js_u8_array(
        this: &RtcDataChannel,
        data: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
}
