#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CanvasRenderingContext2D",
        typescript_type = "CanvasRenderingContext2D"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasRenderingContext2d` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub type CanvasRenderingContext2d;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "canvas"
    )]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn canvas(this: &CanvasRenderingContext2d) -> Option<HtmlCanvasElement>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "globalAlpha"
    )]
    #[doc = "Getter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn global_alpha(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "globalAlpha"
    )]
    #[doc = "Setter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_global_alpha(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "globalCompositeOperation"
    )]
    #[doc = "Getter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn global_composite_operation(
        this: &CanvasRenderingContext2d,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "globalCompositeOperation"
    )]
    #[doc = "Setter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_global_composite_operation(
        this: &CanvasRenderingContext2d,
        value: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Getter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_style(this: &CanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[deprecated]
    pub fn set_stroke_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_stroke_style_str(this: &CanvasRenderingContext2d, value: &str);
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    pub fn set_stroke_style_canvas_gradient(
        this: &CanvasRenderingContext2d,
        value: &CanvasGradient,
    );
    #[cfg(feature = "CanvasPattern")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeStyle"
    )]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`*"]
    pub fn set_stroke_style_canvas_pattern(this: &CanvasRenderingContext2d, value: &CanvasPattern);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Getter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_style(this: &CanvasRenderingContext2d) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[deprecated]
    pub fn set_fill_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_fill_style_str(this: &CanvasRenderingContext2d, value: &str);
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    pub fn set_fill_style_canvas_gradient(this: &CanvasRenderingContext2d, value: &CanvasGradient);
    #[cfg(feature = "CanvasPattern")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillStyle"
    )]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`*"]
    pub fn set_fill_style_canvas_pattern(this: &CanvasRenderingContext2d, value: &CanvasPattern);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "filter"
    )]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn filter(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "filter"
    )]
    #[doc = "Setter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_filter(this: &CanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "imageSmoothingEnabled"
    )]
    #[doc = "Getter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn image_smoothing_enabled(this: &CanvasRenderingContext2d) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "imageSmoothingEnabled"
    )]
    #[doc = "Setter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_image_smoothing_enabled(this: &CanvasRenderingContext2d, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineWidth"
    )]
    #[doc = "Getter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_width(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineWidth"
    )]
    #[doc = "Setter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_width(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineCap"
    )]
    #[doc = "Getter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_cap(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineCap"
    )]
    #[doc = "Setter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_cap(this: &CanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineJoin"
    )]
    #[doc = "Getter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_join(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineJoin"
    )]
    #[doc = "Setter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_join(this: &CanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "miterLimit"
    )]
    #[doc = "Getter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn miter_limit(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "miterLimit"
    )]
    #[doc = "Setter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_miter_limit(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineDashOffset"
    )]
    #[doc = "Getter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_dash_offset(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "lineDashOffset"
    )]
    #[doc = "Setter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_dash_offset(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowOffsetX"
    )]
    #[doc = "Getter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_offset_x(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowOffsetX"
    )]
    #[doc = "Setter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_offset_x(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowOffsetY"
    )]
    #[doc = "Getter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_offset_y(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowOffsetY"
    )]
    #[doc = "Setter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_offset_y(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowBlur"
    )]
    #[doc = "Getter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_blur(this: &CanvasRenderingContext2d) -> f64;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowBlur"
    )]
    #[doc = "Setter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_blur(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowColor"
    )]
    #[doc = "Getter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn shadow_color(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "shadowColor"
    )]
    #[doc = "Setter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_shadow_color(this: &CanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "font"
    )]
    #[doc = "Getter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn font(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "font"
    )]
    #[doc = "Setter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_font(this: &CanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "textAlign"
    )]
    #[doc = "Getter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn text_align(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "textAlign"
    )]
    #[doc = "Setter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_text_align(this: &CanvasRenderingContext2d, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CanvasRenderingContext2D",
        js_name = "textBaseline"
    )]
    #[doc = "Getter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn text_baseline(this: &CanvasRenderingContext2d) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "CanvasRenderingContext2D",
        js_name = "textBaseline"
    )]
    #[doc = "Setter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_text_baseline(this: &CanvasRenderingContext2d, value: &str);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawWindow"
    )]
    #[doc = "The `drawWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
    pub fn draw_window(
        this: &CanvasRenderingContext2d,
        window: &Window,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        bg_color: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawWindow"
    )]
    #[doc = "The `drawWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
    pub fn draw_window_with_flags(
        this: &CanvasRenderingContext2d,
        window: &Window,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        bg_color: &str,
        flags: u32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SvgImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn draw_image_with_html_canvas_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlVideoElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ImageBitmap")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "OffscreenCanvas")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
    pub fn draw_image_with_offscreen_canvas(
        this: &CanvasRenderingContext2d,
        image: &OffscreenCanvas,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn draw_image_with_video_frame(
        this: &CanvasRenderingContext2d,
        image: &VideoFrame,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlImageElement")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn draw_image_with_html_canvas_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
    pub fn draw_image_with_offscreen_canvas_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn draw_image_with_video_frame_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
    pub fn draw_image_with_offscreen_canvas_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "drawImage"
    )]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn draw_image_with_video_frame_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
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
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "beginPath")]
    #[doc = "The `beginPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn begin_path(this: &CanvasRenderingContext2d);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn clip(this: &CanvasRenderingContext2d);
    #[cfg(feature = "CanvasWindingRule")]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "clip")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    pub fn clip_with_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "clip")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn clip_with_path_2d(this: &CanvasRenderingContext2d, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "clip")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    pub fn clip_with_path_2d_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill(this: &CanvasRenderingContext2d);
    #[cfg(feature = "CanvasWindingRule")]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "fill")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    pub fn fill_with_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        winding: CanvasWindingRule,
    );
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "fill")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn fill_with_path_2d(this: &CanvasRenderingContext2d, path: &Path2d);
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "fill")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    pub fn fill_with_path_2d_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        winding: CanvasWindingRule,
    );
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn is_point_in_path_with_f64(this: &CanvasRenderingContext2d, x: f64, y: f64) -> bool;
    #[cfg(feature = "CanvasWindingRule")]
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
    pub fn is_point_in_path_with_f64_and_canvas_winding_rule(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "isPointInPath"
    )]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
    pub fn is_point_in_path_with_path_2d_and_f64_and_winding(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
        winding: CanvasWindingRule,
    ) -> bool;
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "isPointInStroke"
    )]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn is_point_in_stroke_with_x_and_y(this: &CanvasRenderingContext2d, x: f64, y: f64)
        -> bool;
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "isPointInStroke"
    )]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn is_point_in_stroke_with_path_and_x_and_y(
        this: &CanvasRenderingContext2d,
        path: &Path2d,
        x: f64,
        y: f64,
    ) -> bool;
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke(this: &CanvasRenderingContext2d);
    #[cfg(feature = "Path2d")]
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "stroke")]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
    pub fn stroke_with_path(this: &CanvasRenderingContext2d, path: &Path2d);
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createLinearGradient"
    )]
    #[doc = "The `createLinearGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    pub fn create_linear_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> CanvasGradient;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlImageElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlImageElement`*"]
    pub fn create_pattern_with_html_image_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "SvgImageElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `SvgImageElement`*"]
    pub fn create_pattern_with_svg_image_element(
        this: &CanvasRenderingContext2d,
        image: &SvgImageElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlCanvasElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
    pub fn create_pattern_with_html_canvas_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlCanvasElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlVideoElement",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
    pub fn create_pattern_with_html_video_element(
        this: &CanvasRenderingContext2d,
        image: &HtmlVideoElement,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "ImageBitmap",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `ImageBitmap`*"]
    pub fn create_pattern_with_image_bitmap(
        this: &CanvasRenderingContext2d,
        image: &ImageBitmap,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "OffscreenCanvas",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
    pub fn create_pattern_with_offscreen_canvas(
        this: &CanvasRenderingContext2d,
        image: &OffscreenCanvas,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(all(feature = "CanvasPattern", feature = "VideoFrame",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createPattern"
    )]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `VideoFrame`*"]
    pub fn create_pattern_with_video_frame(
        this: &CanvasRenderingContext2d,
        image: &VideoFrame,
        repetition: &str,
    ) -> Result<Option<CanvasPattern>, JsValue>;
    #[cfg(feature = "CanvasGradient")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createRadialGradient"
    )]
    #[doc = "The `createRadialGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
    pub fn create_radial_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
    ) -> Result<CanvasGradient, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "addHitRegion"
    )]
    #[doc = "The `addHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn add_hit_region(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;
    #[cfg(feature = "HitRegionOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "addHitRegion"
    )]
    #[doc = "The `addHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HitRegionOptions`*"]
    pub fn add_hit_region_with_options(
        this: &CanvasRenderingContext2d,
        options: &HitRegionOptions,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "clearHitRegions"
    )]
    #[doc = "The `clearHitRegions()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearHitRegions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn clear_hit_regions(this: &CanvasRenderingContext2d);
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "removeHitRegion"
    )]
    #[doc = "The `removeHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/removeHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn remove_hit_region(this: &CanvasRenderingContext2d, id: &str);
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createImageData"
    )]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn create_image_data_with_sw_and_sh(
        this: &CanvasRenderingContext2d,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData, JsValue>;
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "createImageData"
    )]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn create_image_data_with_imagedata(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
    ) -> Result<ImageData, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "getImageData"
    )]
    #[doc = "The `getImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn get_image_data(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "getImageData"
    )]
    #[doc = "The `getImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_image_data(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn put_image_data(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: f64,
        dy: f64,
    ) -> Result<(), JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn put_image_data(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: i32,
        dy: i32,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageData")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "putImageData"
    )]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
        this: &CanvasRenderingContext2d,
        imagedata: &ImageData,
        dx: i32,
        dy: i32,
        dirty_x: i32,
        dirty_y: i32,
        dirty_width: i32,
        dirty_height: i32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn arc(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D", js_name = "arc")]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn arc_with_anticlockwise(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "arcTo"
    )]
    #[doc = "The `arcTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn arc_to(
        this: &CanvasRenderingContext2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "bezierCurveTo"
    )]
    #[doc = "The `bezierCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn bezier_curve_to(
        this: &CanvasRenderingContext2d,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "closePath")]
    #[doc = "The `closePath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn close_path(this: &CanvasRenderingContext2d);
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn ellipse(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "ellipse"
    )]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn ellipse_with_anticlockwise(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "lineTo")]
    #[doc = "The `lineTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn line_to(this: &CanvasRenderingContext2d, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "moveTo")]
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn move_to(this: &CanvasRenderingContext2d, x: f64, y: f64);
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "quadraticCurveTo"
    )]
    #[doc = "The `quadraticCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn quadratic_curve_to(this: &CanvasRenderingContext2d, cpx: f64, cpy: f64, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `rect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn round_rect(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn round_rect_with_f64(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomPointInit`*"]
    pub fn round_rect_with_dom_point_init(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &DomPointInit,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn round_rect_with_f64_sequence(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "roundRect"
    )]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn round_rect_with_dom_point_init_sequence(
        this: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "getLineDash")]
    #[doc = "The `getLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn get_line_dash(this: &CanvasRenderingContext2d) -> ::js_sys::Array;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "setLineDash"
    )]
    #[doc = "The `setLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_line_dash(
        this: &CanvasRenderingContext2d,
        segments: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "clearRect")]
    #[doc = "The `clearRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn clear_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "fillRect")]
    #[doc = "The `fillRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D", js_name = "strokeRect")]
    #[doc = "The `strokeRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn reset(this: &CanvasRenderingContext2d);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `restore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn restore(this: &CanvasRenderingContext2d);
    #[wasm_bindgen(method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `save()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn save(this: &CanvasRenderingContext2d);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillText"
    )]
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_text(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "fillText"
    )]
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn fill_text_with_max_width(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "TextMetrics")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "measureText"
    )]
    #[doc = "The `measureText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `TextMetrics`*"]
    pub fn measure_text(
        this: &CanvasRenderingContext2d,
        text: &str,
    ) -> Result<TextMetrics, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeText"
    )]
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_text(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "strokeText"
    )]
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn stroke_text_with_max_width(
        this: &CanvasRenderingContext2d,
        text: &str,
        x: f64,
        y: f64,
        max_width: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomMatrix")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "getTransform"
    )]
    #[doc = "The `getTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix`*"]
    pub fn get_transform(this: &CanvasRenderingContext2d) -> Result<DomMatrix, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "resetTransform"
    )]
    #[doc = "The `resetTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn reset_transform(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn rotate(this: &CanvasRenderingContext2d, angle: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn scale(this: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "setTransform"
    )]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_transform(
        this: &CanvasRenderingContext2d,
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
        js_class = "CanvasRenderingContext2D",
        js_name = "setTransform"
    )]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn set_transform_with_default_dom_matrix_2d_init(
        this: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomMatrix2dInit")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "setTransform"
    )]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix2dInit`*"]
    pub fn set_transform_with_dom_matrix_2d_init(
        this: &CanvasRenderingContext2d,
        transform: &DomMatrix2dInit,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `transform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn transform(
        this: &CanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CanvasRenderingContext2D")]
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub fn translate(this: &CanvasRenderingContext2d, x: f64, y: f64) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawCustomFocusRing"
    )]
    #[doc = "The `drawCustomFocusRing()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawCustomFocusRing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
    pub fn draw_custom_focus_ring(this: &CanvasRenderingContext2d, element: &Element) -> bool;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CanvasRenderingContext2D",
        js_name = "drawFocusIfNeeded"
    )]
    #[doc = "The `drawFocusIfNeeded()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
    pub fn draw_focus_if_needed(
        this: &CanvasRenderingContext2d,
        element: &Element,
    ) -> Result<(), JsValue>;
}
impl CanvasRenderingContext2d {
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_DRAW_CARET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_DO_NOT_FLUSH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_DRAW_VIEW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_USE_WIDGET_LAYERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_ASYNC_DECODE_IMAGES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
