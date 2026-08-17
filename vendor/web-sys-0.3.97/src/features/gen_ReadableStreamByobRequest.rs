#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableStreamBYOBRequest",
        typescript_type = "ReadableStreamBYOBRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamByobRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub type ReadableStreamByobRequest;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ReadableStreamBYOBRequest",
        js_name = "view"
    )]
    #[doc = "Getter for the `view` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/view)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub fn view(this: &ReadableStreamByobRequest) -> Option<::js_sys::Object>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableStreamBYOBRequest",
        js_name = "respond"
    )]
    #[doc = "The `respond()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respond)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub fn respond_with_u32(
        this: &ReadableStreamByobRequest,
        bytes_written: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableStreamBYOBRequest",
        js_name = "respond"
    )]
    #[doc = "The `respond()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respond)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub fn respond_with_f64(
        this: &ReadableStreamByobRequest,
        bytes_written: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableStreamBYOBRequest",
        js_name = "respondWithNewView"
    )]
    #[doc = "The `respondWithNewView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub fn respond_with_new_view_with_array_buffer_view(
        this: &ReadableStreamByobRequest,
        view: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableStreamBYOBRequest",
        js_name = "respondWithNewView"
    )]
    #[doc = "The `respondWithNewView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub fn respond_with_new_view_with_u8_array(
        this: &ReadableStreamByobRequest,
        view: &mut [u8],
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableStreamBYOBRequest",
        js_name = "respondWithNewView"
    )]
    #[doc = "The `respondWithNewView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
    pub fn respond_with_new_view_with_js_u8_array(
        this: &ReadableStreamByobRequest,
        view: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
}
