#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VREyeParameters",
        typescript_type = "VREyeParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrEyeParameters` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    pub type VrEyeParameters;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VREyeParameters",
        js_name = "offset"
    )]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    pub fn offset(this: &VrEyeParameters) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[cfg(feature = "VrFieldOfView")]
    #[wasm_bindgen(method, getter, js_class = "VREyeParameters", js_name = "fieldOfView")]
    #[doc = "Getter for the `fieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/fieldOfView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`, `VrFieldOfView`*"]
    pub fn field_of_view(this: &VrEyeParameters) -> VrFieldOfView;
    #[wasm_bindgen(method, getter, js_class = "VREyeParameters", js_name = "renderWidth")]
    #[doc = "Getter for the `renderWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    pub fn render_width(this: &VrEyeParameters) -> u32;
    #[wasm_bindgen(method, getter, js_class = "VREyeParameters", js_name = "renderHeight")]
    #[doc = "Getter for the `renderHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    pub fn render_height(this: &VrEyeParameters) -> u32;
}
