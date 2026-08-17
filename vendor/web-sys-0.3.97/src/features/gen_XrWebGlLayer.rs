#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "XrLayer",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XRWebGLLayer",
        typescript_type = "XRWebGLLayer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrWebGlLayer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrWebGlLayer;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRWebGLLayer", js_name = "antialias")]
    #[doc = "Getter for the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/antialias)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn antialias(this: &XrWebGlLayer) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRWebGLLayer",
        js_name = "ignoreDepthValues"
    )]
    #[doc = "Getter for the `ignoreDepthValues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/ignoreDepthValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ignore_depth_values(this: &XrWebGlLayer) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRWebGLLayer", js_name = "fixedFoveation")]
    #[doc = "Getter for the `fixedFoveation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/fixedFoveation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fixed_foveation(this: &XrWebGlLayer) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "XRWebGLLayer", js_name = "fixedFoveation")]
    #[doc = "Setter for the `fixedFoveation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/fixedFoveation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_fixed_foveation(this: &XrWebGlLayer, value: Option<f32>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebGlFramebuffer")]
    #[wasm_bindgen(method, getter, js_class = "XRWebGLLayer", js_name = "framebuffer")]
    #[doc = "Getter for the `framebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn framebuffer(this: &XrWebGlLayer) -> Option<WebGlFramebuffer>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRWebGLLayer",
        js_name = "framebufferWidth"
    )]
    #[doc = "Getter for the `framebufferWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebufferWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn framebuffer_width(this: &XrWebGlLayer) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "XRWebGLLayer",
        js_name = "framebufferHeight"
    )]
    #[doc = "Getter for the `framebufferHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebufferHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn framebuffer_height(this: &XrWebGlLayer) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "WebGlRenderingContext", feature = "XrSession",))]
    #[wasm_bindgen(catch, constructor, js_class = "XRWebGLLayer")]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_web_gl_rendering_context(
        session: &XrSession,
        context: &WebGlRenderingContext,
    ) -> Result<XrWebGlLayer, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "WebGl2RenderingContext", feature = "XrSession",))]
    #[wasm_bindgen(catch, constructor, js_class = "XRWebGLLayer")]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_web_gl2_rendering_context(
        session: &XrSession,
        context: &WebGl2RenderingContext,
    ) -> Result<XrWebGlLayer, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "WebGlRenderingContext",
        feature = "XrSession",
        feature = "XrWebGlLayerInit",
    ))]
    #[wasm_bindgen(catch, constructor, js_class = "XRWebGLLayer")]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `XrSession`, `XrWebGlLayer`, `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_web_gl_rendering_context_and_layer_init(
        session: &XrSession,
        context: &WebGlRenderingContext,
        layer_init: &XrWebGlLayerInit,
    ) -> Result<XrWebGlLayer, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "WebGl2RenderingContext",
        feature = "XrSession",
        feature = "XrWebGlLayerInit",
    ))]
    #[wasm_bindgen(catch, constructor, js_class = "XRWebGLLayer")]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `XrSession`, `XrWebGlLayer`, `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_web_gl2_rendering_context_and_layer_init(
        session: &XrSession,
        context: &WebGl2RenderingContext,
        layer_init: &XrWebGlLayerInit,
    ) -> Result<XrWebGlLayer, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[wasm_bindgen(
        static_method_of = "XrWebGlLayer",
        js_class = "XRWebGLLayer",
        js_name = "getNativeFramebufferScaleFactor"
    )]
    #[doc = "The `getNativeFramebufferScaleFactor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/getNativeFramebufferScaleFactor_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_native_framebuffer_scale_factor(session: &XrSession) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrView", feature = "XrViewport",))]
    #[wasm_bindgen(method, js_class = "XRWebGLLayer", js_name = "getViewport")]
    #[doc = "The `getViewport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/getViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrView`, `XrViewport`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_viewport(this: &XrWebGlLayer, view: &XrView) -> Option<XrViewport>;
}
