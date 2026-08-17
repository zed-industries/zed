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
        js_name = "Text",
        typescript_type = "Text"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Text` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
    pub type Text;
    #[wasm_bindgen(catch, method, getter, js_class = "Text", js_name = "wholeText")]
    #[doc = "Getter for the `wholeText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/wholeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
    pub fn whole_text(this: &Text) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "HtmlSlotElement")]
    #[wasm_bindgen(method, getter, js_class = "Text", js_name = "assignedSlot")]
    #[doc = "Getter for the `assignedSlot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/assignedSlot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`, `Text`*"]
    pub fn assigned_slot(this: &Text) -> Option<HtmlSlotElement>;
    #[wasm_bindgen(catch, constructor, js_class = "Text")]
    #[doc = "The `new Text(..)` constructor, creating a new instance of `Text`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/Text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
    pub fn new() -> Result<Text, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Text")]
    #[doc = "The `new Text(..)` constructor, creating a new instance of `Text`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/Text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
    pub fn new_with_data(data: &str) -> Result<Text, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "splitText")]
    #[doc = "The `splitText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/splitText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
    pub fn split_text(this: &Text, offset: u32) -> Result<Text, JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Text`*"]
    pub fn convert_point_from_node_with_text(
        this: &Text,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    pub fn convert_point_from_node_with_element(
        this: &Text,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    pub fn convert_point_from_node_with_document(
        this: &Text,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Text`*"]
    pub fn convert_point_from_node_with_text_and_options(
        this: &Text,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
    pub fn convert_point_from_node_with_element_and_options(
        this: &Text,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    pub fn convert_point_from_node_with_document_and_options(
        this: &Text,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(feature = "DomQuad")]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Text`*"]
    pub fn convert_quad_from_node_with_text(
        this: &Text,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
    pub fn convert_quad_from_node_with_element(
        this: &Text,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*"]
    pub fn convert_quad_from_node_with_document(
        this: &Text,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Text`*"]
    pub fn convert_quad_from_node_with_text_and_options(
        this: &Text,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
    pub fn convert_quad_from_node_with_element_and_options(
        this: &Text,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*"]
    pub fn convert_quad_from_node_with_document_and_options(
        this: &Text,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Text`*"]
    pub fn convert_rect_from_node_with_text(
        this: &Text,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    pub fn convert_rect_from_node_with_element(
        this: &Text,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    pub fn convert_rect_from_node_with_document(
        this: &Text,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    pub fn convert_rect_from_node_with_text_and_options(
        this: &Text,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
    pub fn convert_rect_from_node_with_element_and_options(
        this: &Text,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    pub fn convert_rect_from_node_with_document_and_options(
        this: &Text,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "getBoxQuads")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
    pub fn get_box_quads(this: &Text) -> Result<::js_sys::Array, JsValue>;
    #[cfg(feature = "BoxQuadOptions")]
    #[wasm_bindgen(catch, method, js_class = "Text", js_name = "getBoxQuads")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Text`*"]
    pub fn get_box_quads_with_options(
        this: &Text,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, JsValue>;
}
