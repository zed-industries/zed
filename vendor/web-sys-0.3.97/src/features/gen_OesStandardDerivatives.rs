#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "OES_standard_derivatives" , typescript_type = "OES_standard_derivatives")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesStandardDerivatives` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_standard_derivatives)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesStandardDerivatives`*"]
    pub type OesStandardDerivatives;
}
impl OesStandardDerivatives {
    #[doc = "The `OES_standard_derivatives.FRAGMENT_SHADER_DERIVATIVE_HINT_OES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesStandardDerivatives`*"]
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT_OES: u32 = 35723u64 as u32;
}
