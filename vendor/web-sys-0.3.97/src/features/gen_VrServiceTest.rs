#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRServiceTest",
        typescript_type = "VRServiceTest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrServiceTest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrServiceTest`*"]
    pub type VrServiceTest;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "VRServiceTest",
        js_name = "attachVRController"
    )]
    #[doc = "The `attachVRController()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrServiceTest`*"]
    pub fn attach_vr_controller(
        this: &VrServiceTest,
        id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "VRServiceTest", js_name = "attachVRDisplay")]
    #[doc = "The `attachVRDisplay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrServiceTest`*"]
    pub fn attach_vr_display(this: &VrServiceTest, id: &str) -> Result<::js_sys::Promise, JsValue>;
}
