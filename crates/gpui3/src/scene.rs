use std::{iter::Peekable, mem};

use super::{Bounds, Hsla, Pixels, Point};
use crate::{AtlasTile, Corners, DevicePixels, Edges};
use bytemuck::{Pod, Zeroable};

// Exported to metal
pub type PointF = Point<f32>;
pub type StackingOrder = SmallVec<[u32; 16]>;

#[derive(Debug)]
pub struct Scene {
    scale_factor: f32,
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

    pub fn insert(&mut self, order: StackingOrder, primitive: impl Into<Primitive>) {
        let layer = self.layers.entry(order).or_default();

        let primitive = primitive.into();
        match primitive {
            Primitive::Quad(mut quad) => {
                quad.scale(self.scale_factor);
                layer.quads.push(quad);
            }
            Primitive::Sprite(mut sprite) => {
                sprite.scale(self.scale_factor);
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
        self.quads.sort_unstable_by(|a, b| a.order.cmp(&b.order));
        self.sprites.sort_unstable_by(|a, b| a.order.cmp(&b.order));

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
    next_batch_kind: Option<PrimitiveKind>,
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
        if let Some(batch_kind) = self.next_batch_kind.take() {
            match batch_kind {
                PrimitiveKind::Quad => {
                    let max_order = self
                        .next_order(Some(PrimitiveKind::Quad))
                        .unwrap_or(u32::MAX);
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
                    let max_order = self
                        .next_order(Some(PrimitiveKind::Sprite))
                        .unwrap_or(u32::MAX);
                    let sprites_start = self.sprites_start;
                    let sprites_end = sprites_start
                        + self
                            .sprites_iter
                            .by_ref()
                            .take_while(|sprite| sprite.order <= max_order)
                            .count();
                    self.sprites_start = sprites_end;
                    Some(PrimitiveBatch::Sprites(
                        &self.sprites[sprites_start..sprites_end],
                    ))
                }
            }
        } else {
            None
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
        let mut this = Self {
            quads,
            quads_start: 0,
            quads_iter,
            sprites,
            sprites_start: 0,
            sprites_iter,
            next_batch_kind: None,
        };
        this.next_order(None); // Called for its side effect of setting this.next_batch_kind
        this
    }

    fn next_order(&mut self, exclude_kind: Option<PrimitiveKind>) -> Option<u32> {
        let mut next_order = u32::MAX;

        if exclude_kind != Some(PrimitiveKind::Quad) {
            if let Some(next_quad) = self.quads_iter.peek() {
                self.next_batch_kind = Some(PrimitiveKind::Quad);
                next_order = next_quad.order;
            }
        }

        if exclude_kind != Some(PrimitiveKind::Sprite) {
            if let Some(next_sprite) = self.sprites_iter.peek() {
                if next_sprite.order < next_order {
                    self.next_batch_kind = Some(PrimitiveKind::Sprite);
                    next_order = next_sprite.order;
                }
            }
        }

        (next_order < u32::MAX).then_some(next_order)
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

pub enum PrimitiveBatch<'a> {
    Quads(&'a [Quad]),
    Sprites(&'a [MonochromeSprite]),
}

#[derive(Debug, Copy, Clone, Zeroable, Pod)]
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

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MonochromeSprite {
    pub order: u32,
    pub bounds: Bounds<Pixels>,
    pub color: Hsla,
    pub tile: AtlasTile,
}

impl MonochromeSprite {
    pub fn scale(&mut self, factor: f32) {
        self.bounds *= factor;
    }
}

impl From<MonochromeSprite> for Primitive {
    fn from(sprite: MonochromeSprite) -> Self {
        Primitive::Sprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AtlasId(pub(crate) usize);

use collections::BTreeMap;
use etagere::AllocId as TileId;
use smallvec::SmallVec;
