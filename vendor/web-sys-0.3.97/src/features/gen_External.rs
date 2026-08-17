#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "External" , typescript_type = "External")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `External` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`*"]
    pub type External;
    #[wasm_bindgen(method, js_class = "External", js_name = "AddSearchProvider")]
    #[doc = "The `AddSearchProvider()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External/AddSearchProvider)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`*"]
    pub fn add_search_provider(this: &External, a_description_url: &str);
    #[wasm_bindgen(method, js_class = "External", js_name = "IsSearchProviderInstalled")]
    #[doc = "The `IsSearchProviderInstalled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External/IsSearchProviderInstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`*"]
    pub fn is_search_provider_installed(this: &External, a_search_url: &str) -> u32;
}
