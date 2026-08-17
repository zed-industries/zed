#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "URL", typescript_type = "URL")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Url` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub type Url;
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn href(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "href")]
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_href(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "origin")]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn origin(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "protocol")]
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn protocol(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "protocol")]
    #[doc = "Setter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_protocol(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "username")]
    #[doc = "Getter for the `username` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn username(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "username")]
    #[doc = "Setter for the `username` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_username(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "password")]
    #[doc = "Getter for the `password` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn password(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "password")]
    #[doc = "Setter for the `password` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_password(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "host")]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn host(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "host")]
    #[doc = "Setter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_host(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "hostname")]
    #[doc = "Getter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn hostname(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "hostname")]
    #[doc = "Setter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_hostname(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "port")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn port(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "port")]
    #[doc = "Setter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_port(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "pathname")]
    #[doc = "Getter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn pathname(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "pathname")]
    #[doc = "Setter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_pathname(this: &Url, value: &str);
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "search")]
    #[doc = "Getter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn search(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "search")]
    #[doc = "Setter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_search(this: &Url, value: &str);
    #[cfg(feature = "UrlSearchParams")]
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "searchParams")]
    #[doc = "Getter for the `searchParams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`, `UrlSearchParams`*"]
    pub fn search_params(this: &Url) -> UrlSearchParams;
    #[wasm_bindgen(method, getter, js_class = "URL", js_name = "hash")]
    #[doc = "Getter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn hash(this: &Url) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "URL", js_name = "hash")]
    #[doc = "Setter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_hash(this: &Url, value: &str);
    #[wasm_bindgen(catch, constructor, js_class = "URL")]
    #[doc = "The `new Url(..)` constructor, creating a new instance of `Url`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn new(url: &str) -> Result<Url, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "URL")]
    #[doc = "The `new Url(..)` constructor, creating a new instance of `Url`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn new_with_base(url: &str, base: &str) -> Result<Url, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(
        catch,
        static_method_of = "Url",
        js_class = "URL",
        js_name = "createObjectURL"
    )]
    #[doc = "The `createObjectURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Url`*"]
    pub fn create_object_url_with_blob(blob: &Blob) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "MediaSource")]
    #[wasm_bindgen(
        catch,
        static_method_of = "Url",
        js_class = "URL",
        js_name = "createObjectURL"
    )]
    #[doc = "The `createObjectURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `Url`*"]
    pub fn create_object_url_with_source(
        source: &MediaSource,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "Url",
        js_class = "URL",
        js_name = "revokeObjectURL"
    )]
    #[doc = "The `revokeObjectURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn revoke_object_url(url: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "URL", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Url`*"]
    pub fn to_json(this: &Url) -> ::alloc::string::String;
}
