#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SecurityPolicyViolationEventInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SecurityPolicyViolationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub type SecurityPolicyViolationEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SecurityPolicyViolationEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &SecurityPolicyViolationEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SecurityPolicyViolationEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &SecurityPolicyViolationEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SecurityPolicyViolationEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &SecurityPolicyViolationEventInit, val: bool);
    #[doc = "Get the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "blockedURI")]
    pub fn get_blocked_uri(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "blockedURI")]
    pub fn set_blocked_uri(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "columnNumber")]
    pub fn get_column_number(this: &SecurityPolicyViolationEventInit) -> Option<i32>;
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "columnNumber")]
    pub fn set_column_number(this: &SecurityPolicyViolationEventInit, val: i32);
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Get the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "disposition")]
    pub fn get_disposition(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<SecurityPolicyViolationEventDisposition>;
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Change the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "disposition")]
    pub fn set_disposition(
        this: &SecurityPolicyViolationEventInit,
        val: SecurityPolicyViolationEventDisposition,
    );
    #[doc = "Get the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "documentURI")]
    pub fn get_document_uri(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "documentURI")]
    pub fn set_document_uri(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "effectiveDirective")]
    pub fn get_effective_directive(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "effectiveDirective")]
    pub fn set_effective_directive(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "lineNumber")]
    pub fn get_line_number(this: &SecurityPolicyViolationEventInit) -> Option<i32>;
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "lineNumber")]
    pub fn set_line_number(this: &SecurityPolicyViolationEventInit, val: i32);
    #[doc = "Get the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "originalPolicy")]
    pub fn get_original_policy(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "originalPolicy")]
    pub fn set_original_policy(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "referrer")]
    pub fn get_referrer(this: &SecurityPolicyViolationEventInit)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "referrer")]
    pub fn set_referrer(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "sample")]
    pub fn get_sample(this: &SecurityPolicyViolationEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "sample")]
    pub fn set_sample(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "sourceFile")]
    pub fn get_source_file(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "sourceFile")]
    pub fn set_source_file(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &SecurityPolicyViolationEventInit) -> Option<u16>;
    #[doc = "Change the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "statusCode")]
    pub fn set_status_code(this: &SecurityPolicyViolationEventInit, val: u16);
    #[doc = "Get the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "violatedDirective")]
    pub fn get_violated_directive(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, setter = "violatedDirective")]
    pub fn set_violated_directive(this: &SecurityPolicyViolationEventInit, val: &str);
}
impl SecurityPolicyViolationEventInit {
    #[doc = "Construct a new `SecurityPolicyViolationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_blocked_uri()` instead."]
    pub fn blocked_uri(&mut self, val: &str) -> &mut Self {
        self.set_blocked_uri(val);
        self
    }
    #[deprecated = "Use `set_column_number()` instead."]
    pub fn column_number(&mut self, val: i32) -> &mut Self {
        self.set_column_number(val);
        self
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[deprecated = "Use `set_disposition()` instead."]
    pub fn disposition(&mut self, val: SecurityPolicyViolationEventDisposition) -> &mut Self {
        self.set_disposition(val);
        self
    }
    #[deprecated = "Use `set_document_uri()` instead."]
    pub fn document_uri(&mut self, val: &str) -> &mut Self {
        self.set_document_uri(val);
        self
    }
    #[deprecated = "Use `set_effective_directive()` instead."]
    pub fn effective_directive(&mut self, val: &str) -> &mut Self {
        self.set_effective_directive(val);
        self
    }
    #[deprecated = "Use `set_line_number()` instead."]
    pub fn line_number(&mut self, val: i32) -> &mut Self {
        self.set_line_number(val);
        self
    }
    #[deprecated = "Use `set_original_policy()` instead."]
    pub fn original_policy(&mut self, val: &str) -> &mut Self {
        self.set_original_policy(val);
        self
    }
    #[deprecated = "Use `set_referrer()` instead."]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        self.set_referrer(val);
        self
    }
    #[deprecated = "Use `set_sample()` instead."]
    pub fn sample(&mut self, val: &str) -> &mut Self {
        self.set_sample(val);
        self
    }
    #[deprecated = "Use `set_source_file()` instead."]
    pub fn source_file(&mut self, val: &str) -> &mut Self {
        self.set_source_file(val);
        self
    }
    #[deprecated = "Use `set_status_code()` instead."]
    pub fn status_code(&mut self, val: u16) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[deprecated = "Use `set_violated_directive()` instead."]
    pub fn violated_directive(&mut self, val: &str) -> &mut Self {
        self.set_violated_directive(val);
        self
    }
}
impl Default for SecurityPolicyViolationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
