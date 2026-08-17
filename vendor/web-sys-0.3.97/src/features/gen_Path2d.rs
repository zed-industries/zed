#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Path2D",
        typescript_type = "Path2D"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Path2d` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub type Path2d;
    #[wasm_bindgen(catch, constructor, js_class = "Path2D")]
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn new() -> Result<Path2d, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Path2D")]
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn new_with_other(other: &Path2d) -> Result<Path2d, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Path2D")]
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn new_with_path_string(path_string: &str) -> Result<Path2d, JsValue>;
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "addPath")]
    #[doc = "The `addPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn add_path(this: &Path2d, path: &Path2d);
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "addPath")]
    #[doc = "The `addPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`, `SvgMatrix`*"]
    pub fn add_path_with_transformation(this: &Path2d, path: &Path2d, transformation: &SvgMatrix);
    #[wasm_bindgen(catch, method, js_class = "Path2D")]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn arc(
        this: &Path2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "arc")]
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn arc_with_anticlockwise(
        this: &Path2d,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "arcTo")]
    #[doc = "The `arcTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arcTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn arc_to(
        this: &Path2d,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "bezierCurveTo")]
    #[doc = "The `bezierCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/bezierCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn bezier_curve_to(
        this: &Path2d,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    );
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "closePath")]
    #[doc = "The `closePath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/closePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn close_path(this: &Path2d);
    #[wasm_bindgen(catch, method, js_class = "Path2D")]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn ellipse(
        this: &Path2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "ellipse")]
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn ellipse_with_anticlockwise(
        this: &Path2d,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "lineTo")]
    #[doc = "The `lineTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/lineTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn line_to(this: &Path2d, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "moveTo")]
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn move_to(this: &Path2d, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Path2D", js_name = "quadraticCurveTo")]
    #[doc = "The `quadraticCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/quadraticCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn quadratic_curve_to(this: &Path2d, cpx: f64, cpy: f64, x: f64, y: f64);
    #[wasm_bindgen(method, js_class = "Path2D")]
    #[doc = "The `rect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/rect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn rect(this: &Path2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "roundRect")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn round_rect(this: &Path2d, x: f64, y: f64, w: f64, h: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "roundRect")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn round_rect_with_f64(
        this: &Path2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: f64,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "roundRect")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `Path2d`*"]
    pub fn round_rect_with_dom_point_init(
        this: &Path2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &DomPointInit,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "roundRect")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn round_rect_with_f64_sequence(
        this: &Path2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Path2D", js_name = "roundRect")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
    pub fn round_rect_with_dom_point_init_sequence(
        this: &Path2d,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
}
