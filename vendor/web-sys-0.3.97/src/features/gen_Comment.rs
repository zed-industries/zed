#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CharacterData",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Comment",
        typescript_type = "Comment"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Comment` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`*"]
    pub type Comment;
    #[wasm_bindgen(catch, constructor, js_class = "Comment")]
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`*"]
    pub fn new() -> Result<Comment, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Comment")]
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`*"]
    pub fn new_with_data(data: &str) -> Result<Comment, JsValue>;
}
