#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AbstractRange",
        extends = "::js_sys::Object",
        js_name = "Range",
        typescript_type = "Range"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Range` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub type Range;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, getter, js_class = "Range", js_name = "startContainer")]
    #[doc = "Getter for the `startContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn start_container(this: &Range) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Range", js_name = "startOffset")]
    #[doc = "Getter for the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn start_offset(this: &Range) -> Result<u32, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, getter, js_class = "Range", js_name = "endContainer")]
    #[doc = "Getter for the `endContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn end_container(this: &Range) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Range", js_name = "endOffset")]
    #[doc = "Getter for the `endOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn end_offset(this: &Range) -> Result<u32, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Range", js_name = "collapsed")]
    #[doc = "Getter for the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn collapsed(this: &Range) -> bool;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "Range",
        js_name = "commonAncestorContainer"
    )]
    #[doc = "Getter for the `commonAncestorContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/commonAncestorContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn common_ancestor_container(this: &Range) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Range")]
    #[doc = "The `new Range(..)` constructor, creating a new instance of `Range`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/Range)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn new() -> Result<Range, JsValue>;
    #[cfg(feature = "DocumentFragment")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "cloneContents")]
    #[doc = "The `cloneContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
    pub fn clone_contents(this: &Range) -> Result<DocumentFragment, JsValue>;
    #[wasm_bindgen(method, js_class = "Range", js_name = "cloneRange")]
    #[doc = "The `cloneRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn clone_range(this: &Range) -> Range;
    #[wasm_bindgen(method, js_class = "Range")]
    #[doc = "The `collapse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn collapse(this: &Range);
    #[wasm_bindgen(method, js_class = "Range", js_name = "collapse")]
    #[doc = "The `collapse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn collapse_with_to_start(this: &Range, to_start: bool);
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "compareBoundaryPoints")]
    #[doc = "The `compareBoundaryPoints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/compareBoundaryPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn compare_boundary_points(
        this: &Range,
        how: u16,
        source_range: &Range,
    ) -> Result<i16, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "comparePoint")]
    #[doc = "The `comparePoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/comparePoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn compare_point(this: &Range, node: &Node, offset: u32) -> Result<i16, JsValue>;
    #[cfg(feature = "DocumentFragment")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Range",
        js_name = "createContextualFragment"
    )]
    #[doc = "The `createContextualFragment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/createContextualFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
    pub fn create_contextual_fragment(
        this: &Range,
        fragment: &str,
    ) -> Result<DocumentFragment, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "deleteContents")]
    #[doc = "The `deleteContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/deleteContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn delete_contents(this: &Range) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Range")]
    #[doc = "The `detach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/detach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub fn detach(this: &Range);
    #[cfg(feature = "DocumentFragment")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "extractContents")]
    #[doc = "The `extractContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/extractContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
    pub fn extract_contents(this: &Range) -> Result<DocumentFragment, JsValue>;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, js_class = "Range", js_name = "getBoundingClientRect")]
    #[doc = "The `getBoundingClientRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getBoundingClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `Range`*"]
    pub fn get_bounding_client_rect(this: &Range) -> DomRect;
    #[cfg(feature = "DomRectList")]
    #[wasm_bindgen(method, js_class = "Range", js_name = "getClientRects")]
    #[doc = "The `getClientRects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getClientRects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectList`, `Range`*"]
    pub fn get_client_rects(this: &Range) -> Option<DomRectList>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "insertNode")]
    #[doc = "The `insertNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/insertNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn insert_node(this: &Range, node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "intersectsNode")]
    #[doc = "The `intersectsNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/intersectsNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn intersects_node(this: &Range, node: &Node) -> Result<bool, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "isPointInRange")]
    #[doc = "The `isPointInRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/isPointInRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn is_point_in_range(this: &Range, node: &Node, offset: u32) -> Result<bool, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "selectNode")]
    #[doc = "The `selectNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn select_node(this: &Range, ref_node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "selectNodeContents")]
    #[doc = "The `selectNodeContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNodeContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn select_node_contents(this: &Range, ref_node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "setEnd")]
    #[doc = "The `setEnd()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn set_end(this: &Range, ref_node: &Node, offset: u32) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "setEndAfter")]
    #[doc = "The `setEndAfter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndAfter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn set_end_after(this: &Range, ref_node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "setEndBefore")]
    #[doc = "The `setEndBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn set_end_before(this: &Range, ref_node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "setStart")]
    #[doc = "The `setStart()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn set_start(this: &Range, ref_node: &Node, offset: u32) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "setStartAfter")]
    #[doc = "The `setStartAfter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartAfter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn set_start_after(this: &Range, ref_node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "setStartBefore")]
    #[doc = "The `setStartBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn set_start_before(this: &Range, ref_node: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "Range", js_name = "surroundContents")]
    #[doc = "The `surroundContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/surroundContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
    pub fn surround_contents(this: &Range, new_parent: &Node) -> Result<(), JsValue>;
}
impl Range {
    #[doc = "The `Range.START_TO_START` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const START_TO_START: u16 = 0i64 as u16;
    #[doc = "The `Range.START_TO_END` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const START_TO_END: u16 = 1u64 as u16;
    #[doc = "The `Range.END_TO_END` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const END_TO_END: u16 = 2u64 as u16;
    #[doc = "The `Range.END_TO_START` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const END_TO_START: u16 = 3u64 as u16;
}
