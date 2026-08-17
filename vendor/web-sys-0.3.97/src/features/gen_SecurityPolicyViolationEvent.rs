#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "SecurityPolicyViolationEvent",
        typescript_type = "SecurityPolicyViolationEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SecurityPolicyViolationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub type SecurityPolicyViolationEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "documentURI"
    )]
    #[doc = "Getter for the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/documentURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn document_uri(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "referrer"
    )]
    #[doc = "Getter for the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/referrer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn referrer(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "blockedURI"
    )]
    #[doc = "Getter for the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/blockedURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn blocked_uri(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "violatedDirective"
    )]
    #[doc = "Getter for the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/violatedDirective)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn violated_directive(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "effectiveDirective"
    )]
    #[doc = "Getter for the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/effectiveDirective)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn effective_directive(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "originalPolicy"
    )]
    #[doc = "Getter for the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/originalPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn original_policy(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "sourceFile"
    )]
    #[doc = "Getter for the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sourceFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn source_file(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "sample"
    )]
    #[doc = "Getter for the `sample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn sample(this: &SecurityPolicyViolationEvent) -> ::alloc::string::String;
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "disposition"
    )]
    #[doc = "Getter for the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/disposition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventDisposition`*"]
    pub fn disposition(
        this: &SecurityPolicyViolationEvent,
    ) -> SecurityPolicyViolationEventDisposition;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "statusCode"
    )]
    #[doc = "Getter for the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/statusCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn status_code(this: &SecurityPolicyViolationEvent) -> u16;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "lineNumber"
    )]
    #[doc = "Getter for the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/lineNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn line_number(this: &SecurityPolicyViolationEvent) -> i32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SecurityPolicyViolationEvent",
        js_name = "columnNumber"
    )]
    #[doc = "Getter for the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/columnNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn column_number(this: &SecurityPolicyViolationEvent) -> i32;
    #[wasm_bindgen(catch, constructor, js_class = "SecurityPolicyViolationEvent")]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn new(type_: &str) -> Result<SecurityPolicyViolationEvent, JsValue>;
    #[cfg(feature = "SecurityPolicyViolationEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SecurityPolicyViolationEvent")]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &SecurityPolicyViolationEventInit,
    ) -> Result<SecurityPolicyViolationEvent, JsValue>;
}
