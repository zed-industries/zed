#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "OffscreenCanvasRenderingContext2D",
        typescript_type = "OffscreenCanvasRenderingContext2D"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OffscreenCanvasRenderingContext2d` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub type OffscreenCanvasRenderingContext2d;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "canvas"
    )]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn canvas(this: &OffscreenCanvasRenderingContext2d) -> OffscreenCanvas;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "globalAlpha"
    )]
    #[doc = "Getter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn global_alpha(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "globalAlpha"
    )]
    #[doc = "Setter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_global_alpha(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "globalCompositeOperation"
    )]
    #[doc = "Getter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn global_composite_operation(
        this: &OffscreenCanvasRenderingContext2d,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "globalCompositeOperation"
    )]
    #[doc = "Setter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_global_composite_operation(
        this: &OffscreenCanvasRenderingContext2d,
        value: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Getter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn stroke_style(this: &OffscreenCanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    #[deprecated]
    pub fn set_stroke_style(
        this: &OffscreenCanvasRenderingContext2d,
        value: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_stroke_style_str(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_stroke_style_canvas_gradient(
        this: &OffscreenCanvasRenderingContext2d,
        value: &CanvasGradient,
    );
    #[cfg(feature = "CanvasPattern")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_stroke_style_canvas_pattern(
        this: &OffscreenCanvasRenderingContext2d,
        value: &CanvasPattern,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Getter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn fill_style(this: &OffscreenCanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    #[deprecated]
    pub fn set_fill_style(
        this: &OffscreenCanvasRenderingContext2d,
        value: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_fill_style_str(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_fill_style_canvas_gradient(
        this: &OffscreenCanvasRenderingContext2d,
        value: &CanvasGradient,
    );
    #[cfg(feature = "CanvasPattern")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_fill_style_canvas_pattern(
        this: &OffscreenCanvasRenderingContext2d,
        value: &CanvasPattern,
    );
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "filter"
    )]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn filter(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "filter"
    )]
    #[doc = "Setter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_filter(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "imageSmoothingEnabled"
    )]
    #[doc = "Getter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn image_smoothing_enabled(this: &OffscreenCanvasRenderingContext2d) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "imageSmoothingEnabled"
    )]
    #[doc = "Setter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_image_smoothing_enabled(this: &OffscreenCanvasRenderingContext2d, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineWidth"
    )]
    #[doc = "Getter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn line_width(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineWidth"
    )]
    #[doc = "Setter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_line_width(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineCap"
    )]
    #[doc = "Getter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn line_cap(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineCap"
    )]
    #[doc = "Setter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_line_cap(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineJoin"
    )]
    #[doc = "Getter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn line_join(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineJoin"
    )]
    #[doc = "Setter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_line_join(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "miterLimit"
    )]
    #[doc = "Getter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn miter_limit(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "miterLimit"
    )]
    #[doc = "Setter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_miter_limit(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineDashOffset"
    )]
    #[doc = "Getter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn line_dash_offset(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineDashOffset"
    )]
    #[doc = "Setter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_line_dash_offset(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowOffsetX"
    )]
    #[doc = "Getter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn shadow_offset_x(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowOffsetX"
    )]
    #[doc = "Setter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_shadow_offset_x(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowOffsetY"
    )]
    #[doc = "Getter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn shadow_offset_y(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowOffsetY"
    )]
    #[doc = "Setter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_shadow_offset_y(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowBlur"
    )]
    #[doc = "Getter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn shadow_blur(this: &OffscreenCanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowBlur"
    )]
    #[doc = "Setter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_shadow_blur(this: &OffscreenCanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowColor"
    )]
    #[doc = "Getter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn shadow_color(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "shadowColor"
    )]
    #[doc = "Setter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_shadow_color(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "font"
    )]
    #[doc = "Getter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn font(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "font"
    )]
    #[doc = "Setter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_font(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "textAlign"
    )]
    #[doc = "Getter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn text_align(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "textAlign"
    )]
    #[doc = "Setter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_text_align(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "textBaseline"
    )]
    #[doc = "Getter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn text_baseline(this: &OffscreenCanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "textBaseline"
    )]
    #[doc = "Setter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_text_baseline(this: &OffscreenCanvasRenderingContext2d, value: &str);
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_image_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_canvas_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_video_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_image_bitmap(
        this: &OffscreenCanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_offscreen_canvas(
        this: &OffscreenCanvasRenderingContext2d,
        image: &OffscreenCanvas,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn draw_image_with_video_frame(
        this: &OffscreenCanvasRenderingContext2d,
        image: &VideoFrame,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_image_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_canvas_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_video_element_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_offscreen_canvas_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &OffscreenCanvas,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn draw_image_with_video_frame_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &VideoFrame,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &SvgImageElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlVideoElement,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &ImageBitmap,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn draw_image_with_offscreen_canvas_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &OffscreenCanvas,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn draw_image_with_video_frame_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &OffscreenCanvasRenderingContext2d,
        image: &VideoFrame,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "beginPath"
    )]
    #[doc = "The `beginPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/beginPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn begin_path(this: &OffscreenCanvasRenderingContext2d);
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn clip(this: &OffscreenCanvasRenderingContext2d);
    #[cfg(feature = "CanvasWindingRule")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "clip"
    )]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn clip_with_canvas_winding_rule(
        this: &OffscreenCanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "clip"
    )]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn clip_with_path_2d(this: &OffscreenCanvasRenderingContext2d, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "clip"
    )]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`, `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn clip_with_path_2d_and_winding(
        this: &OffscreenCanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn fill(this: &OffscreenCanvasRenderingContext2d);
    #[cfg(feature = "CanvasWindingRule")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fill"
    )]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn fill_with_canvas_winding_rule(
        this: &OffscreenCanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fill"
    )]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn fill_with_path_2d(this: &OffscreenCanvasRenderingContext2d, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fill"
    )]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`, `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn fill_with_path_2d_and_winding(
        this: &OffscreenCanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn is_point_in_path_with_f64(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[cfg(feature = "CanvasWindingRule")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn is_point_in_path_with_f64_and_canvas_winding_rule(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64(
        this: &OffscreenCanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`, `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64_and_winding(
        this: &OffscreenCanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "isPointInStroke"
    )]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn is_point_in_stroke_with_x_and_y(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "isPointInStroke"
    )]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_stroke_with_path_and_x_and_y(
        this: &OffscreenCanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn stroke(this: &OffscreenCanvasRenderingContext2d);
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "stroke"
    )]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `Path2d`*"]
    pub fn stroke_with_path(this: &OffscreenCanvasRenderingContext2d, path: &Path2d);
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createLinearGradient"
    )]
    #[doc = "The `createLinearGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createLinearGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_linear_gradient(
        this: &OffscreenCanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> CanvasGradient;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlImageElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `HtmlImageElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_pattern_with_html_image_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "SvgImageElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn create_pattern_with_svg_image_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &SvgImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlCanvasElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `HtmlCanvasElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_pattern_with_html_canvas_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlVideoElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `HtmlVideoElement`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_pattern_with_html_video_element(
        this: &OffscreenCanvasRenderingContext2d,
        image: &HtmlVideoElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "ImageBitmap",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `ImageBitmap`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_pattern_with_image_bitmap(
        this: &OffscreenCanvasRenderingContext2d,
        image: &ImageBitmap,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "OffscreenCanvas",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvas`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_pattern_with_offscreen_canvas(
        this: &OffscreenCanvasRenderingContext2d,
        image: &OffscreenCanvas,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "VideoFrame",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `OffscreenCanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn create_pattern_with_video_frame(
        this: &OffscreenCanvasRenderingContext2d,
        image: &VideoFrame,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createRadialGradient"
    )]
    #[doc = "The `createRadialGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createRadialGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_radial_gradient(
        this: &OffscreenCanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> Result<CanvasGradient, JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createImageData"
    )]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_image_data_with_sw_and_sh(
        this: &OffscreenCanvasRenderingContext2d,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "createImageData"
    )]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn create_image_data_with_imagedata(
        this: &OffscreenCanvasRenderingContext2d,
        imagedata: &ImageData,
    ) -> Result<ImageData, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "getImageData"
    )]
    #[doc = "The `getImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn get_image_data(
        this: &OffscreenCanvasRenderingContext2d,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "getImageData"
    )]
    #[doc = "The `getImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_image_data(
        this: &OffscreenCanvasRenderingContext2d,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
    ) -> Result<ImageData, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn put_image_data(
        this: &OffscreenCanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        this: &OffscreenCanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
        dirty_x: f64,
        dirty_y: f64,
        dirty_width: f64,
        dirty_height: f64,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn put_image_data(
        this: &OffscreenCanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: i32,
        dy: i32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `OffscreenCanvasRenderingContext2d`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        this: &OffscreenCanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: i32,
        dy: i32,
        dirty_x: i32,
        dirty_y: i32,
        dirty_width: i32,
        dirty_height: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn arc(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "arc"
    )]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn arc_with_anticlockwise(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "arcTo"
    )]
    #[doc = "The `arcTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/arcTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn arc_to(
        this: &OffscreenCanvasRenderingContext2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "bezierCurveTo"
    )]
    #[doc = "The `bezierCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/bezierCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn bezier_curve_to(
        this: &OffscreenCanvasRenderingContext2d,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "closePath"
    )]
    #[doc = "The `closePath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/closePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn close_path(this: &OffscreenCanvasRenderingContext2d);
    #[wasm_bindgen(catch, method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn ellipse(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "ellipse"
    )]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn ellipse_with_anticlockwise(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "lineTo"
    )]
    #[doc = "The `lineTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/lineTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn line_to(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64);
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "moveTo"
    )]
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn move_to(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64);
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "quadraticCurveTo"
    )]
    #[doc = "The `quadraticCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/quadraticCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn quadratic_curve_to(
        this: &OffscreenCanvasRenderingContext2d,
        cpx: f64,
        cpy: f64,
        x: f64,
        y: f64,
    );
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `rect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/rect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn rect(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn round_rect(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn round_rect_with_f64(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn round_rect_with_dom_point_init(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &DomPointInit,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn round_rect_with_f64_sequence(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn round_rect_with_dom_point_init_sequence(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "getLineDash"
    )]
    #[doc = "The `getLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn get_line_dash(this: &OffscreenCanvasRenderingContext2d) -> ::js_sys::Array;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "setLineDash"
    )]
    #[doc = "The `setLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_line_dash(
        this: &OffscreenCanvasRenderingContext2d,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "clearRect"
    )]
    #[doc = "The `clearRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/clearRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn clear_rect(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillRect"
    )]
    #[doc = "The `fillRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn fill_rect(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeRect"
    )]
    #[doc = "The `strokeRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn stroke_rect(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn reset(this: &OffscreenCanvasRenderingContext2d);
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `restore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/restore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn restore(this: &OffscreenCanvasRenderingContext2d);
    #[wasm_bindgen(method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `save()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/save)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn save(this: &OffscreenCanvasRenderingContext2d);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillText"
    )]
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn fill_text(
        this: &OffscreenCanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "fillText"
    )]
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn fill_text_with_max_width(
        this: &OffscreenCanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "TextMetrics")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "measureText"
    )]
    #[doc = "The `measureText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/measureText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`, `TextMetrics`*"]
    pub fn measure_text(
        this: &OffscreenCanvasRenderingContext2d,
        text: &str,
    ) -> Result<TextMetrics, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeText"
    )]
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn stroke_text(
        this: &OffscreenCanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "strokeText"
    )]
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn stroke_text_with_max_width(
        this: &OffscreenCanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "getTransform"
    )]
    #[doc = "The `getTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/getTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn get_transform(this: &OffscreenCanvasRenderingContext2d) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "resetTransform"
    )]
    #[doc = "The `resetTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/resetTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn reset_transform(this: &OffscreenCanvasRenderingContext2d) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn rotate(this: &OffscreenCanvasRenderingContext2d, angle: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn scale(this: &OffscreenCanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "setTransform"
    )]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_transform(
        this: &OffscreenCanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "setTransform"
    )]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_transform_with_default_dom_matrix_2d_init(
        this: &OffscreenCanvasRenderingContext2d,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomMatrix2dInit")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "OffscreenCanvasRenderingContext2D",
        js_name = "setTransform"
    )]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`, `OffscreenCanvasRenderingContext2d`*"]
    pub fn set_transform_with_dom_matrix_2d_init(
        this: &OffscreenCanvasRenderingContext2d,
        transform: &DomMatrix2dInit,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `transform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn transform(
        this: &OffscreenCanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OffscreenCanvasRenderingContext2D")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvasRenderingContext2D/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvasRenderingContext2d`*"]
    pub fn translate(
        this: &OffscreenCanvasRenderingContext2d,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
}
