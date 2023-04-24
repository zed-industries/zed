mod mouse_event;
mod mouse_region;

#[cfg(debug_assertions)]
use collections::HashSet;
use serde::Deserialize;
use serde_json::json;
use std::{borrow::Cow, sync::Arc};

use crate::{
    color::Color,
    fonts::{FontId, GlyphId},
    geometry::{rect::RectF, vector::Vector2F},
    json::ToJson,
    platform::{current::Surface, CursorStyle},
    ImageData,
};
pub use mouse_event::*;
pub use mouse_region::*;

pub struct SceneBuilder {
    scale_factor: f32,
    stacking_contexts: Vec<StackingContext>,
    active_stacking_context_stack: Vec<usize>,
    #[cfg(debug_assertions)]
    mouse_region_ids: HashSet<MouseRegionId>,
}

pub struct Scene {
    scale_factor: f32,
    stacking_contexts: Vec<StackingContext>,
}

struct StackingContext {
    layers: Vec<Layer>,
    active_layer_stack: Vec<usize>,
    z_index: usize,
}

#[derive(Default)]
pub struct Layer {
    clip_bounds: Option<RectF>,
    quads: Vec<Quad>,
    underlines: Vec<Underline>,
    images: Vec<Image>,
    surfaces: Vec<Surface>,
    shadows: Vec<Shadow>,
    glyphs: Vec<Glyph>,
    image_glyphs: Vec<ImageGlyph>,
    icons: Vec<Icon>,
    paths: Vec<Path>,
    cursor_regions: Vec<CursorRegion>,
    mouse_regions: Vec<MouseRegion>,
}

#[derive(Copy, Clone)]
pub struct CursorRegion {
    pub bounds: RectF,
    pub style: CursorStyle,
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

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    pub font_id: FontId,
    pub font_size: f32,
    pub id: GlyphId,
    pub origin: Vector2F,
    pub color: Color,
}

#[derive(Debug)]
pub struct ImageGlyph {
    pub font_id: FontId,
    pub font_size: f32,
    pub id: GlyphId,
    pub origin: Vector2F,
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
    pub overlay: bool,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Underline {
    pub origin: Vector2F,
    pub width: f32,
    pub thickness: f32,
    pub color: Color,
    pub squiggly: bool,
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
            pub overlay: bool,
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
            overlay: data.overlay,
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

pub struct Image {
    pub bounds: RectF,
    pub border: Border,
    pub corner_radius: f32,
    pub grayscale: bool,
    pub data: Arc<ImageData>,
}

impl Scene {
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.stacking_contexts.iter().flat_map(|s| &s.layers)
    }

    pub fn cursor_regions(&self) -> Vec<CursorRegion> {
        self.layers()
            .flat_map(|layer| &layer.cursor_regions)
            .copied()
            .collect()
    }

    pub fn mouse_regions(&self) -> Vec<(MouseRegion, usize)> {
        self.stacking_contexts
            .iter()
            .flat_map(|context| {
                context
                    .layers
                    .iter()
                    .flat_map(|layer| &layer.mouse_regions)
                    .map(|region| (region.clone(), context.z_index))
            })
            .collect()
    }
}

impl SceneBuilder {
    pub fn new(scale_factor: f32) -> Self {
        let stacking_context = StackingContext::new(None, 0);
        SceneBuilder {
            scale_factor,
            stacking_contexts: vec![stacking_context],
            active_stacking_context_stack: vec![0],
            #[cfg(debug_assertions)]
            mouse_region_ids: Default::default(),
        }
    }

    pub fn build(mut self) -> Scene {
        self.stacking_contexts
            .sort_by_key(|context| context.z_index);
        Scene {
            scale_factor: self.scale_factor,
            stacking_contexts: self.stacking_contexts,
        }
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn paint_stacking_context<F>(
        &mut self,
        clip_bounds: Option<RectF>,
        z_index: Option<usize>,
        f: F,
    ) where
        F: FnOnce(&mut Self),
    {
        self.push_stacking_context(clip_bounds, z_index);
        f(self);
        self.pop_stacking_context();
    }

    pub fn push_stacking_context(&mut self, clip_bounds: Option<RectF>, z_index: Option<usize>) {
        let z_index = z_index.unwrap_or_else(|| self.active_stacking_context().z_index + 1);
        self.active_stacking_context_stack
            .push(self.stacking_contexts.len());
        self.stacking_contexts
            .push(StackingContext::new(clip_bounds, z_index))
    }

    pub fn pop_stacking_context(&mut self) {
        self.active_stacking_context_stack.pop();
        assert!(!self.active_stacking_context_stack.is_empty());
    }

    pub fn paint_layer<F>(&mut self, clip_bounds: Option<RectF>, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.push_layer(clip_bounds);
        f(self);
        self.pop_layer();
    }

    pub fn push_layer(&mut self, clip_bounds: Option<RectF>) {
        self.active_stacking_context().push_layer(clip_bounds);
    }

    pub fn pop_layer(&mut self) {
        self.active_stacking_context().pop_layer();
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.active_layer().push_quad(quad)
    }

    pub fn push_cursor_region(&mut self, region: CursorRegion) {
        if can_draw(region.bounds) {
            self.active_layer().push_cursor_region(region);
        }
    }

    pub fn push_mouse_region(&mut self, region: MouseRegion) {
        if can_draw(region.bounds) {
            // Ensure that Regions cannot be added to a scene with the same region id.
            #[cfg(debug_assertions)]
            let region_id;
            #[cfg(debug_assertions)]
            {
                region_id = region.id();
            }

            if self.active_layer().push_mouse_region(region) {
                #[cfg(debug_assertions)]
                {
                    if !self.mouse_region_ids.insert(region_id) {
                        let tag_name = region_id.tag_type_name();
                        panic!("Same MouseRegionId: {region_id:?} inserted multiple times to the same scene. \
                            Will cause problems! Look for MouseRegion that uses Tag: {tag_name}");
                    }
                }
            }
        }
    }

    pub fn push_image(&mut self, image: Image) {
        self.active_layer().push_image(image)
    }

    pub fn push_surface(&mut self, surface: Surface) {
        self.active_layer().push_surface(surface)
    }

    pub fn push_underline(&mut self, underline: Underline) {
        self.active_layer().push_underline(underline)
    }

    pub fn push_shadow(&mut self, shadow: Shadow) {
        self.active_layer().push_shadow(shadow)
    }

    pub fn push_glyph(&mut self, glyph: Glyph) {
        self.active_layer().push_glyph(glyph)
    }

    pub fn push_image_glyph(&mut self, image_glyph: ImageGlyph) {
        self.active_layer().push_image_glyph(image_glyph)
    }

    pub fn push_icon(&mut self, icon: Icon) {
        self.active_layer().push_icon(icon)
    }

    pub fn push_path(&mut self, path: Path) {
        self.active_layer().push_path(path);
    }

    fn active_stacking_context(&mut self) -> &mut StackingContext {
        let ix = *self.active_stacking_context_stack.last().unwrap();
        &mut self.stacking_contexts[ix]
    }

    fn active_layer(&mut self) -> &mut Layer {
        self.active_stacking_context().active_layer()
    }
}

impl StackingContext {
    fn new(clip_bounds: Option<RectF>, z_index: usize) -> Self {
        Self {
            layers: vec![Layer::new(clip_bounds)],
            active_layer_stack: vec![0],
            z_index,
        }
    }

    fn active_layer(&mut self) -> &mut Layer {
        &mut self.layers[*self.active_layer_stack.last().unwrap()]
    }

    fn push_layer(&mut self, clip_bounds: Option<RectF>) {
        let parent_clip_bounds = self.active_layer().clip_bounds();
        let clip_bounds = clip_bounds
            .map(|clip_bounds| {
                clip_bounds
                    .intersection(parent_clip_bounds.unwrap_or(clip_bounds))
                    .unwrap_or_else(|| {
                        if !clip_bounds.is_empty() {
                            log::warn!("specified clip bounds are disjoint from parent layer");
                        }
                        RectF::default()
                    })
            })
            .or(parent_clip_bounds);

        let ix = self.layers.len();
        self.layers.push(Layer::new(clip_bounds));
        self.active_layer_stack.push(ix);
    }

    fn pop_layer(&mut self) {
        self.active_layer_stack.pop().unwrap();
        assert!(!self.active_layer_stack.is_empty());
    }
}

impl Layer {
    pub fn new(clip_bounds: Option<RectF>) -> Self {
        Self {
            clip_bounds,
            quads: Default::default(),
            underlines: Default::default(),
            images: Default::default(),
            surfaces: Default::default(),
            shadows: Default::default(),
            image_glyphs: Default::default(),
            glyphs: Default::default(),
            icons: Default::default(),
            paths: Default::default(),
            cursor_regions: Default::default(),
            mouse_regions: Default::default(),
        }
    }

    pub fn clip_bounds(&self) -> Option<RectF> {
        self.clip_bounds
    }

    fn push_quad(&mut self, quad: Quad) {
        if can_draw(quad.bounds) {
            self.quads.push(quad);
        }
    }

    pub fn quads(&self) -> &[Quad] {
        self.quads.as_slice()
    }

    fn push_cursor_region(&mut self, region: CursorRegion) {
        if let Some(bounds) = region
            .bounds
            .intersection(self.clip_bounds.unwrap_or(region.bounds))
        {
            if can_draw(bounds) {
                self.cursor_regions.push(region);
            }
        }
    }

    fn push_mouse_region(&mut self, region: MouseRegion) -> bool {
        if let Some(bounds) = region
            .bounds
            .intersection(self.clip_bounds.unwrap_or(region.bounds))
        {
            if can_draw(bounds) {
                self.mouse_regions.push(region);
                return true;
            }
        }
        false
    }

    fn push_underline(&mut self, underline: Underline) {
        if underline.width > 0. {
            self.underlines.push(underline);
        }
    }

    pub fn underlines(&self) -> &[Underline] {
        self.underlines.as_slice()
    }

    fn push_image(&mut self, image: Image) {
        if can_draw(image.bounds) {
            self.images.push(image);
        }
    }

    pub fn images(&self) -> &[Image] {
        self.images.as_slice()
    }

    fn push_surface(&mut self, surface: Surface) {
        if can_draw(surface.bounds) {
            self.surfaces.push(surface);
        }
    }

    pub fn surfaces(&self) -> &[Surface] {
        self.surfaces.as_slice()
    }

    fn push_shadow(&mut self, shadow: Shadow) {
        if can_draw(shadow.bounds) {
            self.shadows.push(shadow);
        }
    }

    pub fn shadows(&self) -> &[Shadow] {
        self.shadows.as_slice()
    }

    fn push_image_glyph(&mut self, glyph: ImageGlyph) {
        self.image_glyphs.push(glyph);
    }

    pub fn image_glyphs(&self) -> &[ImageGlyph] {
        self.image_glyphs.as_slice()
    }

    fn push_glyph(&mut self, glyph: Glyph) {
        self.glyphs.push(glyph);
    }

    pub fn glyphs(&self) -> &[Glyph] {
        self.glyphs.as_slice()
    }

    pub fn push_icon(&mut self, icon: Icon) {
        if can_draw(icon.bounds) {
            self.icons.push(icon);
        }
    }

    pub fn icons(&self) -> &[Icon] {
        self.icons.as_slice()
    }

    fn push_path(&mut self, path: Path) {
        if can_draw(path.bounds) {
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
            overlay: false,
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
            overlay: false,
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

impl MouseRegion {
    pub fn id(&self) -> MouseRegionId {
        self.id
    }
}

fn can_draw(bounds: RectF) -> bool {
    let size = bounds.size();
    size.x() > 0. && size.y() > 0.
}
