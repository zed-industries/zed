#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PushSubscriptionOptionsInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptionsInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub type PushSubscriptionOptionsInit;
    #[doc = "Get the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, getter = "applicationServerKey")]
    pub fn get_application_server_key(
        this: &PushSubscriptionOptionsInit,
    ) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, setter = "applicationServerKey")]
    pub fn set_application_server_key(
        this: &PushSubscriptionOptionsInit,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, setter = "applicationServerKey")]
    pub fn set_application_server_key_opt_buffer_source(
        this: &PushSubscriptionOptionsInit,
        val: Option<&::js_sys::Object>,
    );
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, setter = "applicationServerKey")]
    pub fn set_application_server_key_opt_u8_slice(
        this: &PushSubscriptionOptionsInit,
        val: Option<&mut [u8]>,
    );
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, setter = "applicationServerKey")]
    pub fn set_application_server_key_opt_u8_array(
        this: &PushSubscriptionOptionsInit,
        val: Option<&::js_sys::Uint8Array>,
    );
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, setter = "applicationServerKey")]
    pub fn set_application_server_key_opt_str(
        this: &PushSubscriptionOptionsInit,
        val: Option<&str>,
    );
    #[doc = "Get the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, getter = "userVisibleOnly")]
    pub fn get_user_visible_only(this: &PushSubscriptionOptionsInit) -> Option<bool>;
    #[doc = "Change the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    #[wasm_bindgen(method, setter = "userVisibleOnly")]
    pub fn set_user_visible_only(this: &PushSubscriptionOptionsInit, val: bool);
}
impl PushSubscriptionOptionsInit {
    #[doc = "Construct a new `PushSubscriptionOptionsInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_application_server_key()` instead."]
    pub fn application_server_key(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_application_server_key(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[deprecated = "Use `set_user_visible_only()` instead."]
    pub fn user_visible_only(&mut self, val: bool) -> &mut Self {
        self.set_user_visible_only(val);
        self
    }
}
impl Default for PushSubscriptionOptionsInit {
    fn default() -> Self {
        Self::new()
    }
}
