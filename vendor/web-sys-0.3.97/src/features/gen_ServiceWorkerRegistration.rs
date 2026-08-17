#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ServiceWorkerRegistration",
        typescript_type = "ServiceWorkerRegistration"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerRegistration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub type ServiceWorkerRegistration;
    #[cfg(feature = "CookieStoreManager")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "cookies"
    )]
    #[doc = "Getter for the `cookies` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/cookies)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreManager`, `ServiceWorkerRegistration`*"]
    pub fn cookies(this: &ServiceWorkerRegistration) -> CookieStoreManager;
    #[cfg(feature = "ServiceWorker")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "installing"
    )]
    #[doc = "Getter for the `installing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/installing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    pub fn installing(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;
    #[cfg(feature = "ServiceWorker")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "waiting"
    )]
    #[doc = "Getter for the `waiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/waiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    pub fn waiting(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;
    #[cfg(feature = "ServiceWorker")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "active"
    )]
    #[doc = "Getter for the `active` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/active)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    pub fn active(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "scope"
    )]
    #[doc = "Getter for the `scope` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn scope(this: &ServiceWorkerRegistration) -> ::alloc::string::String;
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "updateViaCache"
    )]
    #[doc = "Getter for the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`, `ServiceWorkerUpdateViaCache`*"]
    pub fn update_via_cache(
        this: &ServiceWorkerRegistration,
    ) -> Result<ServiceWorkerUpdateViaCache, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "onupdatefound"
    )]
    #[doc = "Getter for the `onupdatefound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn onupdatefound(this: &ServiceWorkerRegistration) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerRegistration",
        js_name = "onupdatefound"
    )]
    #[doc = "Setter for the `onupdatefound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn set_onupdatefound(this: &ServiceWorkerRegistration, value: Option<&::js_sys::Function>);
    #[cfg(feature = "PushManager")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ServiceWorkerRegistration",
        js_name = "pushManager"
    )]
    #[doc = "Getter for the `pushManager` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/pushManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`, `ServiceWorkerRegistration`*"]
    pub fn push_manager(this: &ServiceWorkerRegistration) -> Result<PushManager, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ServiceWorkerRegistration",
        js_name = "getNotifications"
    )]
    #[doc = "The `getNotifications()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn get_notifications(
        this: &ServiceWorkerRegistration,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ServiceWorkerRegistration",
        js_name = "showNotification"
    )]
    #[doc = "The `showNotification()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn show_notification(
        this: &ServiceWorkerRegistration,
        title: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "NotificationOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ServiceWorkerRegistration",
        js_name = "showNotification"
    )]
    #[doc = "The `showNotification()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`, `ServiceWorkerRegistration`*"]
    pub fn show_notification_with_options(
        this: &ServiceWorkerRegistration,
        title: &str,
        options: &NotificationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ServiceWorkerRegistration")]
    #[doc = "The `unregister()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn unregister(this: &ServiceWorkerRegistration) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ServiceWorkerRegistration")]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn update(this: &ServiceWorkerRegistration) -> Result<::js_sys::Promise, JsValue>;
}
