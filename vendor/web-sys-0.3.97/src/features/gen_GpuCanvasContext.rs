#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUCanvasContext",
        typescript_type = "GPUCanvasContext"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCanvasContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCanvasContext;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUCanvasContext", js_name = "canvas")]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn canvas(this: &GpuCanvasContext) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasConfiguration")]
    #[wasm_bindgen(catch, method, js_class = "GPUCanvasContext")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configure(
        this: &GpuCanvasContext,
        configuration: &GpuCanvasConfiguration,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasConfiguration")]
    #[wasm_bindgen(method, js_class = "GPUCanvasContext", js_name = "getConfiguration")]
    #[doc = "The `getConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_configuration(this: &GpuCanvasContext) -> Option<GpuCanvasConfiguration>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "GPUCanvasContext",
        js_name = "getCurrentTexture"
    )]
    #[doc = "The `getCurrentTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getCurrentTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_current_texture(this: &GpuCanvasContext) -> Result<GpuTexture, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUCanvasContext")]
    #[doc = "The `unconfigure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/unconfigure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unconfigure(this: &GpuCanvasContext);
}
