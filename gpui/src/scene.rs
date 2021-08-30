use serde::Deserialize;
use serde_json::json;
use std::borrow::Cow;

use crate::{
    color::Color,
    fonts::{FontId, GlyphId},
    geometry::{rect::RectF, vector::Vector2F},
    json::ToJson,
};

pub struct Scene {
    scale_factor: f32,
    layers: Vec<Layer>,
    foreground_layers: Vec<Layer>,
    active_layer_stack: Vec<(usize, bool)>,
    pending_foreground_layers: usize,
}

#[derive(Default)]
pub struct Layer {
    clip_bounds: Option<RectF>,
    quads: Vec<Quad>,
    shadows: Vec<Shadow>,
    glyphs: Vec<Glyph>,
    icons: Vec<Icon>,
    paths: Vec<Path>,
}

#[derive(Default, Debug)]
pub struct Quad {
    pub bounds: RectF,
    pub background: Option<Color>,
    pub border: Border,
    pub corner_radius: f32,
}

#[derive(Debug)]
pub struct Shadow {
    pub bounds: RectF,
    pub corner_radius: f32,
    pub sigma: f32,
    pub color: Color,
}

#[derive(Debug)]
pub struct Glyph {
    pub font_id: FontId,
    pub font_size: f32,
    pub id: GlyphId,
    pub origin: Vector2F,
    pub color: Color,
}

pub struct Icon {
    pub bounds: RectF,
    pub svg: usvg::Tree,
    pub path: Cow<'static, str>,
    pub color: Color,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Border {
    pub width: f32,
    pub color: Color,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

impl<'de> Deserialize<'de> for Border {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BorderData {
            pub width: f32,
            pub color: Color,
            #[serde(default)]
            pub top: bool,
            #[serde(default)]
            pub right: bool,
            #[serde(default)]
            pub bottom: bool,
            #[serde(default)]
            pub left: bool,
        }

        let data = BorderData::deserialize(deserializer)?;
        let mut border = Border {
            width: data.width,
            color: data.color,
            top: data.top,
            bottom: data.bottom,
            left: data.left,
            right: data.right,
        };
        if !border.top && !border.bottom && !border.left && !border.right {
            border.top = true;
            border.bottom = true;
            border.left = true;
            border.right = true;
        }
        Ok(border)
    }
}

#[derive(Debug)]
pub struct Path {
    pub bounds: RectF,
    pub color: Color,
    pub vertices: Vec<PathVertex>,
}

#[derive(Debug)]
pub struct PathVertex {
    pub xy_position: Vector2F,
    pub st_position: Vector2F,
}

impl Scene {
    pub fn new(scale_factor: f32) -> Self {
        Scene {
            scale_factor,
            layers: vec![Layer::new(None)],
            foreground_layers: Default::default(),
            active_layer_stack: vec![(0, false)],
            pending_foreground_layers: 0,
        }
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.layers.iter().chain(self.foreground_layers.iter())
    }

    pub fn push_layer(&mut self, clip_bounds: Option<RectF>) {
        if self.pending_foreground_layers == 0 {
            let ix = self.layers.len();
            self.layers.push(Layer::new(clip_bounds));
            self.active_layer_stack.push((ix, false));
        } else {
            let ix = self.foreground_layers.len();
            self.foreground_layers.push(Layer::new(clip_bounds));
            self.active_layer_stack.push((ix, true));
        }
    }

    pub fn push_foreground_layer(&mut self, clip_bounds: Option<RectF>) {
        self.pending_foreground_layers += 1;
        self.push_layer(clip_bounds);
    }

    pub fn pop_layer(&mut self) {
        let (_, foreground) = self.active_layer_stack.pop().unwrap();
        if foreground {
            self.pending_foreground_layers -= 1;
        }
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.active_layer().push_quad(quad)
    }

    pub fn push_shadow(&mut self, shadow: Shadow) {
        self.active_layer().push_shadow(shadow)
    }

    pub fn push_glyph(&mut self, glyph: Glyph) {
        self.active_layer().push_glyph(glyph)
    }

    pub fn push_icon(&mut self, icon: Icon) {
        self.active_layer().push_icon(icon)
    }

    pub fn push_path(&mut self, path: Path) {
        self.active_layer().push_path(path);
    }

    fn active_layer(&mut self) -> &mut Layer {
        let (ix, foreground) = *self.active_layer_stack.last().unwrap();
        if foreground {
            &mut self.foreground_layers[ix]
        } else {
            &mut self.layers[ix]
        }
    }
}

impl Layer {
    pub fn new(clip_bounds: Option<RectF>) -> Self {
        Self {
            clip_bounds,
            quads: Vec::new(),
            shadows: Vec::new(),
            glyphs: Vec::new(),
            icons: Vec::new(),
            paths: Vec::new(),
        }
    }

    pub fn clip_bounds(&self) -> Option<RectF> {
        self.clip_bounds
    }

    fn push_quad(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    pub fn quads(&self) -> &[Quad] {
        self.quads.as_slice()
    }

    fn push_shadow(&mut self, shadow: Shadow) {
        self.shadows.push(shadow);
    }

    pub fn shadows(&self) -> &[Shadow] {
        self.shadows.as_slice()
    }

    fn push_glyph(&mut self, glyph: Glyph) {
        self.glyphs.push(glyph);
    }

    pub fn glyphs(&self) -> &[Glyph] {
        self.glyphs.as_slice()
    }

    pub fn push_icon(&mut self, icon: Icon) {
        self.icons.push(icon);
    }

    pub fn icons(&self) -> &[Icon] {
        self.icons.as_slice()
    }

    fn push_path(&mut self, path: Path) {
        if !path.bounds.is_empty() {
            self.paths.push(path);
        }
    }

    pub fn paths(&self) -> &[Path] {
        self.paths.as_slice()
    }
}

impl Border {
    pub fn new(width: f32, color: Color) -> Self {
        Self {
            width,
            color,
            top: false,
            left: false,
            bottom: false,
            right: false,
        }
    }

    pub fn all(width: f32, color: Color) -> Self {
        Self {
            width,
            color,
            top: true,
            left: true,
            bottom: true,
            right: true,
        }
    }

    pub fn top(width: f32, color: Color) -> Self {
        let mut border = Self::new(width, color);
        border.top = true;
        border
    }

    pub fn left(width: f32, color: Color) -> Self {
        let mut border = Self::new(width, color);
        border.left = true;
        border
    }

    pub fn bottom(width: f32, color: Color) -> Self {
        let mut border = Self::new(width, color);
        border.bottom = true;
        border
    }

    pub fn right(width: f32, color: Color) -> Self {
        let mut border = Self::new(width, color);
        border.right = true;
        border
    }

    pub fn with_sides(mut self, top: bool, left: bool, bottom: bool, right: bool) -> Self {
        self.top = top;
        self.left = left;
        self.bottom = bottom;
        self.right = right;
        self
    }

    pub fn top_width(&self) -> f32 {
        if self.top {
            self.width
        } else {
            0.0
        }
    }

    pub fn left_width(&self) -> f32 {
        if self.left {
            self.width
        } else {
            0.0
        }
    }
}

impl ToJson for Border {
    fn to_json(&self) -> serde_json::Value {
        let mut value = json!({});
        if self.top {
            value["top"] = json!(self.width);
        }
        if self.right {
            value["right"] = json!(self.width);
        }
        if self.bottom {
            value["bottom"] = json!(self.width);
        }
        if self.left {
            value["left"] = json!(self.width);
        }
        value
    }
}
