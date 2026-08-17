#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FontFace",
        typescript_type = "FontFace"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFace` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub type FontFace;
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "family")]
    #[doc = "Getter for the `family` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn family(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "family")]
    #[doc = "Setter for the `family` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_family(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "style")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn style(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "style")]
    #[doc = "Setter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_style(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "weight")]
    #[doc = "Getter for the `weight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn weight(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "weight")]
    #[doc = "Setter for the `weight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_weight(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "stretch")]
    #[doc = "Getter for the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn stretch(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "stretch")]
    #[doc = "Setter for the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_stretch(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "unicodeRange")]
    #[doc = "Getter for the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn unicode_range(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "unicodeRange")]
    #[doc = "Setter for the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_unicode_range(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "variant")]
    #[doc = "Getter for the `variant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn variant(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "variant")]
    #[doc = "Setter for the `variant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_variant(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "featureSettings")]
    #[doc = "Getter for the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn feature_settings(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "featureSettings")]
    #[doc = "Setter for the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_feature_settings(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "variationSettings")]
    #[doc = "Getter for the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn variation_settings(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "variationSettings")]
    #[doc = "Setter for the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_variation_settings(this: &FontFace, value: &str);
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "display")]
    #[doc = "Getter for the `display` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn display(this: &FontFace) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "FontFace", js_name = "display")]
    #[doc = "Setter for the `display` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn set_display(this: &FontFace, value: &str);
    #[cfg(feature = "FontFaceLoadStatus")]
    #[wasm_bindgen(method, getter, js_class = "FontFace", js_name = "status")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceLoadStatus`*"]
    pub fn status(this: &FontFace) -> FontFaceLoadStatus;
    #[wasm_bindgen(catch, method, getter, js_class = "FontFace", js_name = "loaded")]
    #[doc = "Getter for the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/loaded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn loaded(this: &FontFace) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_str(family: &str, source: &str) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_array_buffer(
        family: &str,
        source: &::js_sys::ArrayBuffer,
    ) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_array_buffer_view(
        family: &str,
        source: &::js_sys::Object,
    ) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_u8_array(family: &str, source: &[u8]) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn new_with_js_u8_array(
        family: &str,
        source: &::js_sys::Uint8Array,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_str_and_descriptors(
        family: &str,
        source: &str,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_array_buffer_and_descriptors(
        family: &str,
        source: &::js_sys::ArrayBuffer,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_array_buffer_view_and_descriptors(
        family: &str,
        source: &::js_sys::Object,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_u8_array_and_descriptors(
        family: &str,
        source: &[u8],
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[cfg(feature = "FontFaceDescriptors")]
    #[wasm_bindgen(catch, constructor, js_class = "FontFace")]
    #[doc = "The `new FontFace(..)` constructor, creating a new instance of `FontFace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/FontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceDescriptors`*"]
    pub fn new_with_js_u8_array_and_descriptors(
        family: &str,
        source: &::js_sys::Uint8Array,
        descriptors: &FontFaceDescriptors,
    ) -> Result<FontFace, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "FontFace")]
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`*"]
    pub fn load(this: &FontFace) -> Result<::js_sys::Promise, JsValue>;
}
