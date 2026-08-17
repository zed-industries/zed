#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WorkerLocation",
        typescript_type = "WorkerLocation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerLocation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub type WorkerLocation;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn href(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "origin")]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn origin(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "protocol")]
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn protocol(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "host")]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn host(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "hostname")]
    #[doc = "Getter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn hostname(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "port")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn port(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "pathname")]
    #[doc = "Getter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn pathname(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "search")]
    #[doc = "Getter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn search(this: &WorkerLocation) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WorkerLocation", js_name = "hash")]
    #[doc = "Getter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn hash(this: &WorkerLocation) -> ::alloc::string::String;
}
