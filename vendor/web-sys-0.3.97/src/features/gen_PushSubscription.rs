#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PushSubscription",
        typescript_type = "PushSubscription"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscription` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`*"]
    pub type PushSubscription;
    #[wasm_bindgen(method, getter, js_class = "PushSubscription", js_name = "endpoint")]
    #[doc = "Getter for the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/endpoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`*"]
    pub fn endpoint(this: &PushSubscription) -> ::alloc::string::String;
    #[cfg(feature = "PushSubscriptionOptions")]
    #[wasm_bindgen(method, getter, js_class = "PushSubscription", js_name = "options")]
    #[doc = "Getter for the `options` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/options)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionOptions`*"]
    pub fn options(this: &PushSubscription) -> PushSubscriptionOptions;
    #[cfg(feature = "PushEncryptionKeyName")]
    #[wasm_bindgen(catch, method, js_class = "PushSubscription", js_name = "getKey")]
    #[doc = "The `getKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/getKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEncryptionKeyName`, `PushSubscription`*"]
    pub fn get_key(
        this: &PushSubscription,
        name: PushEncryptionKeyName,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
    #[cfg(feature = "PushSubscriptionJson")]
    #[wasm_bindgen(catch, method, js_class = "PushSubscription", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionJson`*"]
    pub fn to_json(this: &PushSubscription) -> Result<PushSubscriptionJson, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PushSubscription")]
    #[doc = "The `unsubscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/unsubscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`*"]
    pub fn unsubscribe(this: &PushSubscription) -> Result<::js_sys::Promise, JsValue>;
}
