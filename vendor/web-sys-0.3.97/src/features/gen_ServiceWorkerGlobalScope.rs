#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "WorkerGlobalScope",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ServiceWorkerGlobalScope",
        typescript_type = "ServiceWorkerGlobalScope"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub type ServiceWorkerGlobalScope;
    #[cfg(feature = "CookieStore")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "cookieStore"
    )]
    #[doc = "Getter for the `cookieStore` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/cookieStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`, `ServiceWorkerGlobalScope`*"]
    pub fn cookie_store(this: &ServiceWorkerGlobalScope) -> CookieStore;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "oncookiechange"
    )]
    #[doc = "Getter for the `oncookiechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncookiechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn oncookiechange(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "oncookiechange"
    )]
    #[doc = "Setter for the `oncookiechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncookiechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_oncookiechange(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[cfg(feature = "Clients")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "clients"
    )]
    #[doc = "Getter for the `clients` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/clients)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`, `ServiceWorkerGlobalScope`*"]
    pub fn clients(this: &ServiceWorkerGlobalScope) -> Clients;
    #[cfg(feature = "ServiceWorkerRegistration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "registration"
    )]
    #[doc = "Getter for the `registration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/registration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`, `ServiceWorkerRegistration`*"]
    pub fn registration(this: &ServiceWorkerGlobalScope) -> ServiceWorkerRegistration;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "oninstall"
    )]
    #[doc = "Getter for the `oninstall` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn oninstall(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "oninstall"
    )]
    #[doc = "Setter for the `oninstall` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_oninstall(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onactivate"
    )]
    #[doc = "Getter for the `onactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onactivate(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onactivate"
    )]
    #[doc = "Setter for the `onactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onactivate(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onfetch"
    )]
    #[doc = "Getter for the `onfetch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onfetch(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onfetch"
    )]
    #[doc = "Setter for the `onfetch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onfetch(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onmessage"
    )]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onmessage(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onmessage"
    )]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onmessage(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onpush"
    )]
    #[doc = "Getter for the `onpush` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onpush(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onpush"
    )]
    #[doc = "Setter for the `onpush` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onpush(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onpushsubscriptionchange"
    )]
    #[doc = "Getter for the `onpushsubscriptionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onpushsubscriptionchange(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onpushsubscriptionchange"
    )]
    #[doc = "Setter for the `onpushsubscriptionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onpushsubscriptionchange(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onnotificationclick"
    )]
    #[doc = "Getter for the `onnotificationclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onnotificationclick(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onnotificationclick"
    )]
    #[doc = "Setter for the `onnotificationclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onnotificationclick(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onnotificationclose"
    )]
    #[doc = "Getter for the `onnotificationclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onnotificationclose(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "onnotificationclose"
    )]
    #[doc = "Setter for the `onnotificationclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onnotificationclose(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ServiceWorkerGlobalScope",
        js_name = "skipWaiting"
    )]
    #[doc = "The `skipWaiting()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/skipWaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn skip_waiting(this: &ServiceWorkerGlobalScope) -> Result<::js_sys::Promise, JsValue>;
}
