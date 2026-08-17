#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "MOZ_debug" , typescript_type = "MOZ_debug")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MozDebug` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MOZ_debug)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MozDebug`*"]
    pub type MozDebug;
    #[wasm_bindgen(catch, method, js_class = "MOZ_debug", js_name = "getParameter")]
    #[doc = "The `getParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MOZ_debug/getParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MozDebug`*"]
    pub fn get_parameter(this: &MozDebug, pname: u32) -> Result<::wasm_bindgen::JsValue, JsValue>;
}
impl MozDebug {
    #[doc = "The `MOZ_debug.EXTENSIONS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MozDebug`*"]
    pub const EXTENSIONS: u32 = 7939u64 as u32;
    #[doc = "The `MOZ_debug.WSI_INFO` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MozDebug`*"]
    pub const WSI_INFO: u32 = 65536u64 as u32;
    #[doc = "The `MOZ_debug.UNPACK_REQUIRE_FASTPATH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MozDebug`*"]
    pub const UNPACK_REQUIRE_FASTPATH: u32 = 65537u64 as u32;
}
