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
        js_name = "HTMLCanvasElement",
        typescript_type = "HTMLCanvasElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlCanvasElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub type HtmlCanvasElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLCanvasElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn width(this: &HtmlCanvasElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLCanvasElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn set_width(this: &HtmlCanvasElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLCanvasElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn height(this: &HtmlCanvasElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLCanvasElement", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn set_height(this: &HtmlCanvasElement, value: u32);
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLCanvasElement",
        js_name = "captureStream"
    )]
    #[doc = "The `captureStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/captureStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `MediaStream`*"]
    pub fn capture_stream(this: &HtmlCanvasElement) -> Result<MediaStream, JsValue>;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLCanvasElement",
        js_name = "captureStream"
    )]
    #[doc = "The `captureStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/captureStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `MediaStream`*"]
    pub fn capture_stream_with_frame_request_rate(
        this: &HtmlCanvasElement,
        frame_request_rate: f64,
    ) -> Result<MediaStream, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "getContext")]
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn get_context(
        this: &HtmlCanvasElement,
        context_id: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "getContext")]
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn get_context_with_context_options(
        this: &HtmlCanvasElement,
        context_id: &str,
        context_options: &::wasm_bindgen::JsValue,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "toBlob")]
    #[doc = "The `toBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn to_blob(this: &HtmlCanvasElement, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "toBlob")]
    #[doc = "The `toBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn to_blob_with_type(
        this: &HtmlCanvasElement,
        callback: &::js_sys::Function,
        type_: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "toBlob")]
    #[doc = "The `toBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn to_blob_with_type_and_encoder_options(
        this: &HtmlCanvasElement,
        callback: &::js_sys::Function,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "toDataURL")]
    #[doc = "The `toDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn to_data_url(this: &HtmlCanvasElement) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "toDataURL")]
    #[doc = "The `toDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn to_data_url_with_type(
        this: &HtmlCanvasElement,
        type_: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLCanvasElement", js_name = "toDataURL")]
    #[doc = "The `toDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn to_data_url_with_type_and_encoder_options(
        this: &HtmlCanvasElement,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLCanvasElement",
        js_name = "transferControlToOffscreen"
    )]
    #[doc = "The `transferControlToOffscreen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/transferControlToOffscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvas`*"]
    pub fn transfer_control_to_offscreen(
        this: &HtmlCanvasElement,
    ) -> Result<OffscreenCanvas, JsValue>;
}
