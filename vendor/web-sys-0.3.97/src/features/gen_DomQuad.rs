#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMQuad",
        typescript_type = "DOMQuad"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuad` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`*"]
    pub type DomQuad;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter, js_class = "DOMQuad", js_name = "p1")]
    #[doc = "Getter for the `p1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p1(this: &DomQuad) -> DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter, js_class = "DOMQuad", js_name = "p2")]
    #[doc = "Getter for the `p2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p2(this: &DomQuad) -> DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter, js_class = "DOMQuad", js_name = "p3")]
    #[doc = "Getter for the `p3` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p3)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p3(this: &DomQuad) -> DomPoint;
    #[cfg(feature = "DomPoint")]
    #[wasm_bindgen(method, getter, js_class = "DOMQuad", js_name = "p4")]
    #[doc = "Getter for the `p4` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p4)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    pub fn p4(this: &DomQuad) -> DomPoint;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, getter, js_class = "DOMQuad", js_name = "bounds")]
    #[doc = "Getter for the `bounds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/bounds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    pub fn bounds(this: &DomQuad) -> DomRectReadOnly;
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`*"]
    pub fn new() -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init(p1: &DomPointInit) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init_and_p2(
        p1: &DomPointInit,
        p2: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init_and_p2_and_p3(
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    pub fn new_with_dom_point_init_and_p2_and_p3_and_p4(
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
        p4: &DomPointInit,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(catch, constructor, js_class = "DOMQuad")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    pub fn new_with_rect(rect: &DomRectReadOnly) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, js_class = "DOMQuad", js_name = "getBounds")]
    #[doc = "The `getBounds()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/getBounds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    pub fn get_bounds(this: &DomQuad) -> DomRectReadOnly;
    #[cfg(feature = "DomQuadJson")]
    #[wasm_bindgen(method, js_class = "DOMQuad", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomQuadJson`*"]
    pub fn to_json(this: &DomQuad) -> DomQuadJson;
}
