#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Response",
        typescript_type = "Response"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Response` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub type Response;
    #[cfg(feature = "ResponseType")]
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseType`*"]
    pub fn type_(this: &Response) -> ResponseType;
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn url(this: &Response) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "redirected")]
    #[doc = "Getter for the `redirected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirected(this: &Response) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "status")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn status(this: &Response) -> u16;
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "ok")]
    #[doc = "Getter for the `ok` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn ok(this: &Response) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "statusText")]
    #[doc = "Getter for the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn status_text(this: &Response) -> ::alloc::string::String;
    #[cfg(feature = "Headers")]
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "headers")]
    #[doc = "Getter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`, `Response`*"]
    pub fn headers(this: &Response) -> Headers;
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "bodyUsed")]
    #[doc = "Getter for the `bodyUsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn body_used(this: &Response) -> bool;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, getter, js_class = "Response", js_name = "body")]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    pub fn body(this: &Response) -> Option<ReadableStream>;
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new() -> Result<Response, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Response`*"]
    pub fn new_with_opt_blob(body: Option<&Blob>) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_buffer_source(body: Option<&::js_sys::Object>)
        -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_u8_array(body: Option<&mut [u8]>) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_js_u8_array(
        body: Option<&::js_sys::Uint8Array>,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "FormData")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Response`*"]
    pub fn new_with_opt_form_data(body: Option<&FormData>) -> Result<Response, JsValue>;
    #[cfg(feature = "UrlSearchParams")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `UrlSearchParams`*"]
    pub fn new_with_opt_url_search_params(
        body: Option<&UrlSearchParams>,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_str(body: Option<&str>) -> Result<Response, JsValue>;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    pub fn new_with_opt_readable_stream(body: Option<&ReadableStream>)
        -> Result<Response, JsValue>;
    #[cfg(all(feature = "Blob", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_blob_and_init(
        body: Option<&Blob>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_buffer_source_and_init(
        body: Option<&::js_sys::Object>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_u8_array_and_init(
        body: Option<&mut [u8]>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_js_u8_array_and_init(
        body: Option<&::js_sys::Uint8Array>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "FormData", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_form_data_and_init(
        body: Option<&FormData>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "ResponseInit", feature = "UrlSearchParams",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`, `UrlSearchParams`*"]
    pub fn new_with_opt_url_search_params_and_init(
        body: Option<&UrlSearchParams>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_str_and_init(
        body: Option<&str>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "ReadableStream", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_readable_stream_and_init(
        body: Option<&ReadableStream>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Response")]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn clone(this: &Response) -> Result<Response, JsValue>;
    #[wasm_bindgen(static_method_of = "Response", js_class = "Response")]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/error_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn error() -> Response;
    #[wasm_bindgen(catch, static_method_of = "Response", js_class = "Response")]
    #[doc = "The `redirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirect(url: &str) -> Result<Response, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "Response",
        js_class = "Response",
        js_name = "redirect"
    )]
    #[doc = "The `redirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirect_with_status(url: &str, status: u16) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Response", js_name = "arrayBuffer")]
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn array_buffer(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Response")]
    #[doc = "The `blob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn blob(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Response", js_name = "formData")]
    #[doc = "The `formData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn form_data(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Response")]
    #[doc = "The `json()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn json(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Response")]
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
    pub fn text(this: &Response) -> Result<::js_sys::Promise, JsValue>;
}
