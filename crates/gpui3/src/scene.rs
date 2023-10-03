use std::{iter::Peekable, mem};

use super::{Bounds, Hsla, Pixels, Point};
use crate::{AtlasTextureId, AtlasTile, Corners, Edges};
use bytemuck::{Pod, Zeroable};
use collections::BTreeMap;
use smallvec::SmallVec;

// Exported to metal
pub type PointF = Point<f32>;
pub type StackingOrder = SmallVec<[u32; 16]>;

#[derive(Debug)]
pub struct Scene {
    pub(crate) scale_factor: f32,
    pub(crate) layers: BTreeMap<StackingOrder, SceneLayer>,
}

impl Scene {
    pub fn new(scale_factor: f32) -> Scene {
        Scene {
            scale_factor,
            layers: BTreeMap::new(),
        }
    }

    pub fn take(&mut self) -> Scene {
        Scene {
            scale_factor: self.scale_factor,
            layers: mem::take(&mut self.layers),
        }
    }

    pub fn insert(&mut self, stacking_order: StackingOrder, primitive: impl Into<Primitive>) {
        let layer = self.layers.entry(stacking_order).or_default();

        let primitive = primitive.into();
        match primitive {
            Primitive::Quad(mut quad) => {
                quad.scale(self.scale_factor);
                layer.quads.push(quad);
            }
            Primitive::Sprite(sprite) => {
                layer.sprites.push(sprite);
            }
        }
    }

    pub(crate) fn layers(&mut self) -> impl Iterator<Item = &mut SceneLayer> {
        self.layers.values_mut()
    }
}

#[derive(Debug, Default)]
pub(crate) struct SceneLayer {
    pub quads: Vec<Quad>,
    pub sprites: Vec<MonochromeSprite>,
}

impl SceneLayer {
    pub fn batches(&mut self) -> impl Iterator<Item = PrimitiveBatch> {
        self.quads.sort_unstable();
        self.sprites.sort_unstable();

        BatchIterator::new(
            &self.quads,
            self.quads.iter().peekable(),
            &self.sprites,
            self.sprites.iter().peekable(),
        )
    }
}

struct BatchIterator<'a, Q, S>
where
    Q: Iterator<Item = &'a Quad>,
    S: Iterator<Item = &'a MonochromeSprite>,
{
    quads: &'a [Quad],
    sprites: &'a [MonochromeSprite],
    quads_start: usize,
    sprites_start: usize,
    quads_iter: Peekable<Q>,
    sprites_iter: Peekable<S>,
}

impl<'a, Q: 'a, S: 'a> Iterator for BatchIterator<'a, Q, S>
where
    Q: Iterator<Item = &'a Quad>,
    S: Iterator<Item = &'a MonochromeSprite>,
{
    type Item = PrimitiveBatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut kinds_and_orders = [
            (PrimitiveKind::Quad, self.quads_iter.peek().map(|q| q.order)),
            (
                PrimitiveKind::Sprite,
                self.sprites_iter.peek().map(|s| s.order),
            ),
        ];
        kinds_and_orders.sort_by_key(|(_, order)| order.unwrap_or(u32::MAX));

        let first = kinds_and_orders[0];
        let second = kinds_and_orders[1];
        let (batch_kind, max_order) = if first.1.is_some() {
            (first.0, second.1.unwrap_or(u32::MAX))
        } else {
            return None;
        };

        match batch_kind {
            PrimitiveKind::Quad => {
                let quads_start = self.quads_start;
                let quads_end = quads_start
                    + self
                        .quads_iter
                        .by_ref()
                        .take_while(|quad| quad.order <= max_order)
                        .count();
                self.quads_start = quads_end;
                Some(PrimitiveBatch::Quads(&self.quads[quads_start..quads_end]))
            }
            PrimitiveKind::Sprite => {
                let texture_id = self.sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.sprites_start;
                let sprites_end = sprites_start
                    + self
                        .sprites_iter
                        .by_ref()
                        .take_while(|sprite| {
                            sprite.order <= max_order && sprite.tile.texture_id == texture_id
                        })
                        .count();
                self.sprites_start = sprites_end;
                Some(PrimitiveBatch::Sprites {
                    texture_id,
                    sprites: &self.sprites[sprites_start..sprites_end],
                })
            }
        }
    }
}

impl<'a, Q: 'a, S: 'a> BatchIterator<'a, Q, S>
where
    Q: Iterator<Item = &'a Quad>,
    S: Iterator<Item = &'a MonochromeSprite>,
{
    fn new(
        quads: &'a [Quad],
        quads_iter: Peekable<Q>,
        sprites: &'a [MonochromeSprite],
        sprites_iter: Peekable<S>,
    ) -> Self {
        Self {
            quads,
            quads_start: 0,
            quads_iter,
            sprites,
            sprites_start: 0,
            sprites_iter,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveKind {
    Quad,
    Sprite,
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Quad(Quad),
    Sprite(MonochromeSprite),
}

pub(crate) enum PrimitiveBatch<'a> {
    Quads(&'a [Quad]),
    Sprites {
        texture_id: AtlasTextureId,
        sprites: &'a [MonochromeSprite],
    },
}

#[derive(Debug, Copy, Clone, Zeroable, Pod, Eq, PartialEq)]
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

impl Ord for Quad {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Quad {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct MonochromeSprite {
    pub order: u32,
    pub bounds: Bounds<Pixels>,
    pub color: Hsla,
    pub tile: AtlasTile,
}

impl Ord for MonochromeSprite {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            std::cmp::Ordering::Equal => self.tile.tile_id.cmp(&other.tile.tile_id),
            order => order,
        }
    }
}

impl PartialOrd for MonochromeSprite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<MonochromeSprite> for Primitive {
    fn from(sprite: MonochromeSprite) -> Self {
        Primitive::Sprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AtlasId(pub(crate) usize);
