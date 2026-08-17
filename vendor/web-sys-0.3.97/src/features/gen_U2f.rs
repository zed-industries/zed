#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "U2F", typescript_type = "U2F")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `U2f` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub type U2f;
    #[wasm_bindgen(catch, method, js_class = "U2F")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub fn register(
        this: &U2f,
        app_id: &str,
        register_requests: &::wasm_bindgen::JsValue,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "U2F", js_name = "register")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub fn register_with_opt_timeout_seconds(
        this: &U2f,
        app_id: &str,
        register_requests: &::wasm_bindgen::JsValue,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
        opt_timeout_seconds: Option<i32>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "U2F")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub fn sign(
        this: &U2f,
        app_id: &str,
        challenge: &str,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "U2F", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub fn sign_with_opt_timeout_seconds(
        this: &U2f,
        app_id: &str,
        challenge: &str,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
        opt_timeout_seconds: Option<i32>,
    ) -> Result<(), JsValue>;
}
impl U2f {
    #[doc = "The `U2F.OK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub const OK: u16 = 0i64 as u16;
    #[doc = "The `U2F.OTHER_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub const OTHER_ERROR: u16 = 1u64 as u16;
    #[doc = "The `U2F.BAD_REQUEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub const BAD_REQUEST: u16 = 2u64 as u16;
    #[doc = "The `U2F.CONFIGURATION_UNSUPPORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub const CONFIGURATION_UNSUPPORTED: u16 = 3u64 as u16;
    #[doc = "The `U2F.DEVICE_INELIGIBLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub const DEVICE_INELIGIBLE: u16 = 4u64 as u16;
    #[doc = "The `U2F.TIMEOUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`*"]
    pub const TIMEOUT: u16 = 5u64 as u16;
}
