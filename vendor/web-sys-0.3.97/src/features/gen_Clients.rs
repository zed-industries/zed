#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Clients",
        typescript_type = "Clients"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Clients` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    pub type Clients;
    #[wasm_bindgen(method, js_class = "Clients")]
    #[doc = "The `claim()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/claim)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    pub fn claim(this: &Clients) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Clients")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    pub fn get(this: &Clients, id: &str) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Clients", js_name = "matchAll")]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    pub fn match_all(this: &Clients) -> ::js_sys::Promise;
    #[cfg(feature = "ClientQueryOptions")]
    #[wasm_bindgen(method, js_class = "Clients", js_name = "matchAll")]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `Clients`*"]
    pub fn match_all_with_options(
        this: &Clients,
        options: &ClientQueryOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Clients", js_name = "openWindow")]
    #[doc = "The `openWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/openWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    pub fn open_window(this: &Clients, url: &str) -> ::js_sys::Promise;
}
