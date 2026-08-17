#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ServiceWorkerContainer",
        typescript_type = "ServiceWorkerContainer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerContainer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub type ServiceWorkerContainer;
    #[cfg(feature = "ServiceWorker")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerContainer",
        js_name = "controller"
    )]
    #[doc = "Getter for the `controller` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/controller)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerContainer`*"]
    pub fn controller(this: &ServiceWorkerContainer) -> Option<ServiceWorker>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ServiceWorkerContainer",
        js_name = "ready"
    )]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn ready(this: &ServiceWorkerContainer) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerContainer",
        js_name = "oncontrollerchange"
    )]
    #[doc = "Getter for the `oncontrollerchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn oncontrollerchange(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerContainer",
        js_name = "oncontrollerchange"
    )]
    #[doc = "Setter for the `oncontrollerchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn set_oncontrollerchange(
        this: &ServiceWorkerContainer,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerContainer",
        js_name = "onerror"
    )]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn onerror(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerContainer",
        js_name = "onerror"
    )]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn set_onerror(this: &ServiceWorkerContainer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ServiceWorkerContainer",
        js_name = "onmessage"
    )]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn onmessage(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ServiceWorkerContainer",
        js_name = "onmessage"
    )]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn set_onmessage(this: &ServiceWorkerContainer, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        js_class = "ServiceWorkerContainer",
        js_name = "getRegistration"
    )]
    #[doc = "The `getRegistration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_registration(this: &ServiceWorkerContainer) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        js_class = "ServiceWorkerContainer",
        js_name = "getRegistration"
    )]
    #[doc = "The `getRegistration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_registration_with_document_url(
        this: &ServiceWorkerContainer,
        document_url: &str,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        js_class = "ServiceWorkerContainer",
        js_name = "getRegistrations"
    )]
    #[doc = "The `getRegistrations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistrations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_registrations(this: &ServiceWorkerContainer) -> ::js_sys::Promise;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ServiceWorkerContainer",
        js_name = "getScopeForUrl"
    )]
    #[doc = "The `getScopeForUrl()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getScopeForUrl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_scope_for_url(
        this: &ServiceWorkerContainer,
        url: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "ServiceWorkerContainer")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn register(this: &ServiceWorkerContainer, script_url: &str) -> ::js_sys::Promise;
    #[cfg(feature = "RegistrationOptions")]
    #[wasm_bindgen(method, js_class = "ServiceWorkerContainer", js_name = "register")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerContainer`*"]
    pub fn register_with_options(
        this: &ServiceWorkerContainer,
        script_url: &str,
        options: &RegistrationOptions,
    ) -> ::js_sys::Promise;
}
