#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "WorkerGlobalScope",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "DedicatedWorkerGlobalScope",
        typescript_type = "DedicatedWorkerGlobalScope"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DedicatedWorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub type DedicatedWorkerGlobalScope;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "name"
    )]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn name(this: &DedicatedWorkerGlobalScope) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "onmessage"
    )]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn onmessage(this: &DedicatedWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "onmessage"
    )]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn set_onmessage(this: &DedicatedWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "onmessageerror"
    )]
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn onmessageerror(this: &DedicatedWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "onmessageerror"
    )]
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn set_onmessageerror(
        this: &DedicatedWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "onrtctransform"
    )]
    #[doc = "Getter for the `onrtctransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onrtctransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onrtctransform(this: &DedicatedWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "onrtctransform"
    )]
    #[doc = "Setter for the `onrtctransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onrtctransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onrtctransform(
        this: &DedicatedWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(method, js_class = "DedicatedWorkerGlobalScope")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn close(this: &DedicatedWorkerGlobalScope);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "postMessage"
    )]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn post_message(
        this: &DedicatedWorkerGlobalScope,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "postMessage"
    )]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn post_message_with_transfer(
        this: &DedicatedWorkerGlobalScope,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "cancelAnimationFrame"
    )]
    #[doc = "The `cancelAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/cancelAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn cancel_animation_frame(
        this: &DedicatedWorkerGlobalScope,
        handle: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DedicatedWorkerGlobalScope",
        js_name = "requestAnimationFrame"
    )]
    #[doc = "The `requestAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    pub fn request_animation_frame(
        this: &DedicatedWorkerGlobalScope,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
}
