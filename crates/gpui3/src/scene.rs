use crate::{
    AtlasTextureId, AtlasTile, Bounds, Corners, Edges, Hsla, Point, ScaledContentMask, ScaledPixels,
};
use collections::BTreeMap;
use etagere::euclid::{Point3D, Vector3D};
use plane_split::{BspSplitter, Polygon as BspPolygon};
use smallvec::SmallVec;
use std::{iter::Peekable, mem, slice};

// Exported to metal
pub type PointF = Point<f32>;
pub type StackingOrder = SmallVec<[u32; 16]>;
pub type LayerId = u32;
pub type DrawOrder = u32;

#[derive(Debug)]
pub struct Scene {
    pub(crate) scale_factor: f32,
    pub(crate) layers: BTreeMap<StackingOrder, LayerId>,
    pub shadows: Vec<Shadow>,
    pub quads: Vec<Quad>,
    pub underlines: Vec<Underline>,
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
}

impl Scene {
    pub fn new(scale_factor: f32) -> Scene {
        Scene {
            scale_factor,
            layers: BTreeMap::new(),
            shadows: Vec::new(),
            quads: Vec::new(),
            underlines: Vec::new(),
            monochrome_sprites: Vec::new(),
            polychrome_sprites: Vec::new(),
        }
    }

    pub fn take(&mut self) -> Scene {
        Scene {
            scale_factor: self.scale_factor,
            layers: mem::take(&mut self.layers),
            shadows: mem::take(&mut self.shadows),
            quads: mem::take(&mut self.quads),
            underlines: mem::take(&mut self.underlines),
            monochrome_sprites: mem::take(&mut self.monochrome_sprites),
            polychrome_sprites: mem::take(&mut self.polychrome_sprites),
        }
    }

    pub fn insert(&mut self, layer_id: StackingOrder, primitive: impl Into<Primitive>) {
        let next_id = self.layers.len() as LayerId;
        let layer_id = *self.layers.entry(layer_id).or_insert(next_id);
        let primitive = primitive.into();
        match primitive {
            Primitive::Shadow(mut shadow) => {
                shadow.order = layer_id;
                self.shadows.push(shadow);
            }
            Primitive::Quad(mut quad) => {
                quad.order = layer_id;
                self.quads.push(quad);
            }
            Primitive::Underline(mut underline) => {
                underline.order = layer_id;
                self.underlines.push(underline);
            }
            Primitive::MonochromeSprite(mut sprite) => {
                sprite.order = layer_id;
                self.monochrome_sprites.push(sprite);
            }
            Primitive::PolychromeSprite(mut sprite) => {
                sprite.order = layer_id;
                self.polychrome_sprites.push(sprite);
            }
        }
    }

    pub(crate) fn batches(&mut self) -> impl Iterator<Item = PrimitiveBatch> {
        // Map each layer id to a float between 0. and 1., with 1. closer to the viewer.
        let mut layer_z_values = vec![0.; self.layers.len()];
        for (ix, layer_id) in self.layers.values().enumerate() {
            layer_z_values[*layer_id as usize] = ix as f32 / self.layers.len() as f32;
        }

        // Add all primitives to the BSP splitter to determine draw order
        // todo!("reuse the same splitter")
        let mut splitter = BspSplitter::new();

        for (ix, shadow) in self.shadows.iter().enumerate() {
            let z = layer_z_values[shadow.order as LayerId as usize];
            splitter.add(shadow.bounds.to_bsp_polygon(z, (PrimitiveKind::Shadow, ix)));
        }

        for (ix, quad) in self.quads.iter().enumerate() {
            let z = layer_z_values[quad.order as LayerId as usize];
            splitter.add(quad.bounds.to_bsp_polygon(z, (PrimitiveKind::Quad, ix)));
        }

        for (ix, underline) in self.underlines.iter().enumerate() {
            let z = layer_z_values[underline.order as LayerId as usize];
            splitter.add(
                underline
                    .bounds
                    .to_bsp_polygon(z, (PrimitiveKind::Underline, ix)),
            );
        }

        for (ix, monochrome_sprite) in self.monochrome_sprites.iter().enumerate() {
            let z = layer_z_values[monochrome_sprite.order as LayerId as usize];
            splitter.add(
                monochrome_sprite
                    .bounds
                    .to_bsp_polygon(z, (PrimitiveKind::MonochromeSprite, ix)),
            );
        }

        for (ix, polychrome_sprite) in self.polychrome_sprites.iter().enumerate() {
            let z = layer_z_values[polychrome_sprite.order as LayerId as usize];
            splitter.add(
                polychrome_sprite
                    .bounds
                    .to_bsp_polygon(z, (PrimitiveKind::PolychromeSprite, ix)),
            );
        }

        // Sort all polygons, then reassign the order field of each primitive to `draw_order`
        // We need primitives to be repr(C), hence the weird reuse of the order field for two different types.
        for (draw_order, polygon) in splitter.sort(Vector3D::new(0., 0., 1.)).iter().enumerate() {
            match polygon.anchor {
                (PrimitiveKind::Shadow, ix) => self.shadows[ix].order = draw_order as DrawOrder,
                (PrimitiveKind::Quad, ix) => self.quads[ix].order = draw_order as DrawOrder,
                (PrimitiveKind::Underline, ix) => {
                    self.underlines[ix].order = draw_order as DrawOrder
                }
                (PrimitiveKind::MonochromeSprite, ix) => {
                    self.monochrome_sprites[ix].order = draw_order as DrawOrder
                }
                (PrimitiveKind::PolychromeSprite, ix) => {
                    self.polychrome_sprites[ix].order = draw_order as DrawOrder
                }
            }
        }

        // Sort the primitives
        self.shadows.sort_unstable();
        self.quads.sort_unstable();
        self.underlines.sort_unstable();
        self.monochrome_sprites.sort_unstable();
        self.polychrome_sprites.sort_unstable();

        BatchIterator {
            shadows: &self.shadows,
            shadows_start: 0,
            shadows_iter: self.shadows.iter().peekable(),
            quads: &self.quads,
            quads_start: 0,
            quads_iter: self.quads.iter().peekable(),
            underlines: &self.underlines,
            underlines_start: 0,
            underlines_iter: self.underlines.iter().peekable(),
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
    shadows: &'a [Shadow],
    shadows_start: usize,
    shadows_iter: Peekable<slice::Iter<'a, Shadow>>,
    underlines: &'a [Underline],
    underlines_start: usize,
    underlines_iter: Peekable<slice::Iter<'a, Underline>>,
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
        let mut orders_and_kinds = [
            (
                self.shadows_iter.peek().map(|s| s.order),
                PrimitiveKind::Shadow,
            ),
            (self.quads_iter.peek().map(|q| q.order), PrimitiveKind::Quad),
            (
                self.underlines_iter.peek().map(|u| u.order),
                PrimitiveKind::Underline,
            ),
            (
                self.monochrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::MonochromeSprite,
            ),
            (
                self.polychrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::PolychromeSprite,
            ),
        ];
        orders_and_kinds.sort_by_key(|(order, kind)| (order.unwrap_or(u32::MAX), *kind));

        let first = orders_and_kinds[0];
        let second = orders_and_kinds[1];
        let (batch_kind, max_order) = if first.0.is_some() {
            (first.1, second.0.unwrap_or(u32::MAX))
        } else {
            return None;
        };

        match batch_kind {
            PrimitiveKind::Shadow => {
                let shadows_start = self.shadows_start;
                let mut shadows_end = shadows_start;
                while self
                    .shadows_iter
                    .next_if(|shadow| shadow.order <= max_order)
                    .is_some()
                {
                    shadows_end += 1;
                }
                self.shadows_start = shadows_end;
                Some(PrimitiveBatch::Shadows(
                    &self.shadows[shadows_start..shadows_end],
                ))
            }
            PrimitiveKind::Quad => {
                let quads_start = self.quads_start;
                let mut quads_end = quads_start;
                while self
                    .quads_iter
                    .next_if(|quad| quad.order <= max_order)
                    .is_some()
                {
                    quads_end += 1;
                }
                self.quads_start = quads_end;
                Some(PrimitiveBatch::Quads(&self.quads[quads_start..quads_end]))
            }
            PrimitiveKind::Underline => {
                let underlines_start = self.underlines_start;
                let mut underlines_end = underlines_start;
                while self
                    .underlines_iter
                    .next_if(|underline| underline.order <= max_order)
                    .is_some()
                {
                    underlines_end += 1;
                }
                self.underlines_start = underlines_end;
                Some(PrimitiveBatch::Underlines(
                    &self.underlines[underlines_start..underlines_end],
                ))
            }
            PrimitiveKind::MonochromeSprite => {
                let texture_id = self.monochrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.monochrome_sprites_start;
                let mut sprites_end = sprites_start;
                while self
                    .monochrome_sprites_iter
                    .next_if(|sprite| {
                        sprite.order <= max_order && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.monochrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites: &self.monochrome_sprites[sprites_start..sprites_end],
                })
            }
            PrimitiveKind::PolychromeSprite => {
                let texture_id = self.polychrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.polychrome_sprites_start;
                let mut sprites_end = self.polychrome_sprites_start;
                while self
                    .polychrome_sprites_iter
                    .next_if(|sprite| {
                        sprite.order <= max_order && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.polychrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites: &self.polychrome_sprites[sprites_start..sprites_end],
                })
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum PrimitiveKind {
    Shadow,
    #[default]
    Quad,
    Underline,
    MonochromeSprite,
    PolychromeSprite,
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Shadow(Shadow),
    Quad(Quad),
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    PolychromeSprite(PolychromeSprite),
}

#[derive(Debug)]
pub(crate) enum PrimitiveBatch<'a> {
    Shadows(&'a [Shadow]),
    Quads(&'a [Quad]),
    Underlines(&'a [Underline]),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [MonochromeSprite],
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [PolychromeSprite],
    },
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Quad {
    pub order: u32, // Initially a LayerId, then a DrawOrder.
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ScaledContentMask,
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

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Underline {
    pub order: u32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ScaledContentMask,
    pub thickness: ScaledPixels,
    pub color: Hsla,
    pub wavy: bool,
}

impl Ord for Underline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Underline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Underline> for Primitive {
    fn from(underline: Underline) -> Self {
        Primitive::Underline(underline)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Shadow {
    pub order: u32,
    pub bounds: Bounds<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub content_mask: ScaledContentMask,
    pub color: Hsla,
    pub blur_radius: ScaledPixels,
}

impl Ord for Shadow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Shadow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Shadow> for Primitive {
    fn from(shadow: Shadow) -> Self {
        Primitive::Shadow(shadow)
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

impl Bounds<ScaledPixels> {
    fn to_bsp_polygon<A: Copy>(&self, z: f32, anchor: A) -> BspPolygon<A> {
        let upper_left = self.origin;
        let upper_right = self.upper_right();
        let lower_right = self.lower_right();
        let lower_left = self.lower_left();

        BspPolygon::from_points(
            [
                Point3D::new(upper_left.x.into(), upper_left.y.into(), z as f64),
                Point3D::new(upper_right.x.into(), upper_right.y.into(), z as f64),
                Point3D::new(lower_right.x.into(), lower_right.y.into(), z as f64),
                Point3D::new(lower_left.x.into(), lower_left.y.into(), z as f64),
            ],
            anchor,
        )
        .expect("Polygon should not be empty")
    }
}

#[cfg(test)]
mod tests {
    use crate::{point, size};

    use super::*;
    use smallvec::smallvec;

    #[test]
    fn test_scene() {
        let mut scene = Scene::new(1.0);
        assert_eq!(scene.layers.len(), 0);

        scene.insert(smallvec![1], quad());
        scene.insert(smallvec![2], shadow());
        scene.insert(smallvec![3], quad());

        let mut batches_count = 0;
        for _ in scene.batches() {
            batches_count += 1;
        }
        assert_eq!(batches_count, 3);
    }

    fn quad() -> Quad {
        Quad {
            order: 0,
            bounds: Bounds {
                origin: point(ScaledPixels(0.), ScaledPixels(0.)),
                size: size(ScaledPixels(100.), ScaledPixels(100.)),
            },
            content_mask: Default::default(),
            background: Default::default(),
            border_color: Default::default(),
            corner_radii: Default::default(),
            border_widths: Default::default(),
        }
    }

    fn shadow() -> Shadow {
        Shadow {
            order: Default::default(),
            bounds: Bounds {
                origin: point(ScaledPixels(0.), ScaledPixels(0.)),
                size: size(ScaledPixels(100.), ScaledPixels(100.)),
            },
            corner_radii: Default::default(),
            content_mask: Default::default(),
            color: Default::default(),
            blur_radius: Default::default(),
        }
    }
}
