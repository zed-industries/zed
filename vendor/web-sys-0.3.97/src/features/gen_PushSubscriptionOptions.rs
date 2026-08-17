#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PushSubscriptionOptions",
        typescript_type = "PushSubscriptionOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptions` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
    pub type PushSubscriptionOptions;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "PushSubscriptionOptions",
        js_name = "applicationServerKey"
    )]
    #[doc = "Getter for the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/applicationServerKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
    pub fn application_server_key(
        this: &PushSubscriptionOptions,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
}
