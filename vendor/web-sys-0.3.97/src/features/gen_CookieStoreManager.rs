#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CookieStoreManager",
        typescript_type = "CookieStoreManager"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieStoreManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreManager`*"]
    pub type CookieStoreManager;
    #[wasm_bindgen(method, js_class = "CookieStoreManager", js_name = "getSubscriptions")]
    #[doc = "The `getSubscriptions()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager/getSubscriptions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreManager`*"]
    pub fn get_subscriptions(this: &CookieStoreManager) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStoreManager")]
    #[doc = "The `subscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager/subscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreManager`*"]
    pub fn subscribe(
        this: &CookieStoreManager,
        subscriptions: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStoreManager")]
    #[doc = "The `unsubscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager/unsubscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreManager`*"]
    pub fn unsubscribe(
        this: &CookieStoreManager,
        subscriptions: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
}
