use std::mem;

use super::{Bounds, Hsla, Pixels, Point};
use crate::{Corners, Edges};
use bytemuck::{Pod, Zeroable};
use collections::BTreeMap;

// Exported to metal
pub type PointF = Point<f32>;

pub struct Scene {
    layers: BTreeMap<u32, SceneLayer>,
    pub(crate) scale_factor: f32,
}

#[derive(Default)]
pub struct SceneLayer {
    pub quads: Vec<Quad>,
}

impl Scene {
    pub fn new(scale_factor: f32) -> Scene {
        Scene {
            layers: Default::default(),
            scale_factor,
        }
    }

    pub fn take(&mut self) -> Scene {
        Scene {
            layers: mem::take(&mut self.layers),
            scale_factor: self.scale_factor,
        }
    }

    pub fn insert(&mut self, primitive: impl Into<Primitive>) {
        let mut primitive = primitive.into();
        primitive.scale(self.scale_factor);
        let layer = self.layers.entry(primitive.order()).or_default();
        match primitive {
            Primitive::Quad(quad) => layer.quads.push(quad),
        }
    }

    pub fn layers(&self) -> impl Iterator<Item = &SceneLayer> {
        self.layers.values()
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Quad(Quad),
}

impl Primitive {
    pub fn order(&self) -> u32 {
        match self {
            Primitive::Quad(quad) => quad.order,
        }
    }

    pub fn is_transparent(&self) -> bool {
        match self {
            Primitive::Quad(quad) => {
                quad.background.is_transparent() && quad.border_color.is_transparent()
            }
        }
    }

    pub fn scale(&mut self, factor: f32) {
        match self {
            Primitive::Quad(quad) => {
                quad.scale(factor);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Quad {
    pub order: u32,
    pub bounds: Bounds<Pixels>,
    pub clip_bounds: Bounds<Pixels>,
    pub clip_corner_radii: Corners<Pixels>,
    pub background: Hsla,
    pub border_color: Hsla,
    pub corner_radii: Corners<Pixels>,
    pub border_widths: Edges<Pixels>,
}

impl Quad {
    pub fn vertices(&self) -> impl Iterator<Item = Point<Pixels>> {
        let x1 = self.bounds.origin.x;
        let y1 = self.bounds.origin.y;
        let x2 = x1 + self.bounds.size.width;
        let y2 = y1 + self.bounds.size.height;
        [
            Point::new(x1, y1),
            Point::new(x2, y1),
            Point::new(x2, y2),
            Point::new(x1, y2),
        ]
        .into_iter()
    }

    pub fn scale(&mut self, factor: f32) {
        self.bounds *= factor;
        self.clip_bounds *= factor;
        self.clip_corner_radii *= factor;
        self.corner_radii *= factor;
        self.border_widths *= factor;
    }
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}
