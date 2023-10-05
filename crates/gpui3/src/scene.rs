use std::{iter::Peekable, mem, slice};

use super::{Bounds, Hsla, Point};
use crate::{AtlasTextureId, AtlasTile, Corners, Edges, ScaledContentMask, ScaledPixels};
use collections::BTreeMap;
use smallvec::SmallVec;

// Exported to metal
pub type PointF = Point<f32>;
pub type LayerId = SmallVec<[u32; 16]>;

#[derive(Debug)]
pub struct Scene {
    pub(crate) scale_factor: f32,
    pub(crate) layers: BTreeMap<LayerId, SceneLayer>,
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

    pub fn insert(&mut self, stacking_order: LayerId, primitive: impl Into<Primitive>) {
        let layer = self.layers.entry(stacking_order).or_default();

        let primitive = primitive.into();
        match primitive {
            Primitive::Quad(quad) => {
                layer.quads.push(quad);
            }
            Primitive::MonochromeSprite(sprite) => {
                layer.monochrome_sprites.push(sprite);
            }
            Primitive::PolychromeSprite(sprite) => {
                layer.polychrome_sprites.push(sprite);
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
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
}

impl SceneLayer {
    pub fn batches(&mut self) -> impl Iterator<Item = PrimitiveBatch> {
        self.quads.sort_unstable();
        self.monochrome_sprites.sort_unstable();
        self.polychrome_sprites.sort_unstable();
        BatchIterator {
            quads: &self.quads,
            quads_start: 0,
            quads_iter: self.quads.iter().peekable(),
            monochrome_sprites: &self.monochrome_sprites,
            monochrome_sprites_start: 0,
            monochrome_sprites_iter: self.monochrome_sprites.iter().peekable(),
            polychrome_sprites: &self.polychrome_sprites,
            polychrome_sprites_start: 0,
            polychrome_sprites_iter: self.polychrome_sprites.iter().peekable(),
        }
    }
}

struct BatchIterator<'a> {
    quads: &'a [Quad],
    quads_start: usize,
    quads_iter: Peekable<slice::Iter<'a, Quad>>,
    monochrome_sprites: &'a [MonochromeSprite],
    monochrome_sprites_start: usize,
    monochrome_sprites_iter: Peekable<slice::Iter<'a, MonochromeSprite>>,
    polychrome_sprites: &'a [PolychromeSprite],
    polychrome_sprites_start: usize,
    polychrome_sprites_iter: Peekable<slice::Iter<'a, PolychromeSprite>>,
}

impl<'a> Iterator for BatchIterator<'a> {
    type Item = PrimitiveBatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut kinds_and_orders = [
            (PrimitiveKind::Quad, self.quads_iter.peek().map(|q| q.order)),
            (
                PrimitiveKind::MonochromeSprite,
                self.monochrome_sprites_iter.peek().map(|s| s.order),
            ),
            (
                PrimitiveKind::PolychromeSprite,
                self.polychrome_sprites_iter.peek().map(|s| s.order),
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
            PrimitiveKind::MonochromeSprite => {
                let texture_id = self.monochrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.monochrome_sprites_start;
                let sprites_end = sprites_start
                    + self
                        .monochrome_sprites_iter
                        .by_ref()
                        .take_while(|sprite| {
                            sprite.order <= max_order && sprite.tile.texture_id == texture_id
                        })
                        .count();
                self.monochrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites: &self.monochrome_sprites[sprites_start..sprites_end],
                })
            }
            PrimitiveKind::PolychromeSprite => {
                let texture_id = self.polychrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.polychrome_sprites_start;
                let sprites_end = sprites_start
                    + self
                        .polychrome_sprites_iter
                        .by_ref()
                        .take_while(|sprite| {
                            sprite.order <= max_order && sprite.tile.texture_id == texture_id
                        })
                        .count();
                self.polychrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites: &self.polychrome_sprites[sprites_start..sprites_end],
                })
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveKind {
    Quad,
    MonochromeSprite,
    PolychromeSprite,
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Quad(Quad),
    MonochromeSprite(MonochromeSprite),
    PolychromeSprite(PolychromeSprite),
}

pub(crate) enum PrimitiveBatch<'a> {
    Quads(&'a [Quad]),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [MonochromeSprite],
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [PolychromeSprite],
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Quad {
    pub order: u32,
    pub bounds: Bounds<ScaledPixels>,
    pub clip_bounds: Bounds<ScaledPixels>,
    pub clip_corner_radii: Corners<ScaledPixels>,
    pub background: Hsla,
    pub border_color: Hsla,
    pub corner_radii: Corners<ScaledPixels>,
    pub border_widths: Edges<ScaledPixels>,
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
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ScaledContentMask,
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
        Primitive::MonochromeSprite(sprite)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PolychromeSprite {
    pub order: u32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ScaledContentMask,
    pub corner_radii: Corners<ScaledPixels>,
    pub tile: AtlasTile,
    pub grayscale: bool,
}

impl Ord for PolychromeSprite {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            std::cmp::Ordering::Equal => self.tile.tile_id.cmp(&other.tile.tile_id),
            order => order,
        }
    }
}

impl PartialOrd for PolychromeSprite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<PolychromeSprite> for Primitive {
    fn from(sprite: PolychromeSprite) -> Self {
        Primitive::PolychromeSprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AtlasId(pub(crate) usize);
