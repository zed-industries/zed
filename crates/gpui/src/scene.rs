// todo("windows"): remove
#![cfg_attr(windows, allow(dead_code))]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AtlasTextureId, AtlasTile, Background, Bounds, ContentMask, Corners, Edges, Hsla, Pixels,
    Point, Radians, ScaledPixels, Size, bounds_tree::BoundsTree, point,
};
use std::{
    fmt::Debug,
    iter::Peekable,
    ops::{Add, Range, Sub},
    slice,
};

#[allow(non_camel_case_types, unused)]
#[expect(missing_docs)]
pub type PathVertex_ScaledPixels = PathVertex<ScaledPixels>;

#[expect(missing_docs)]
pub type DrawOrder = u32;

/// A boolean stored as a `u32` so that GPU-facing structs contain no
/// compiler-inserted padding bytes, which would be undefined behavior to
/// reinterpret as `&[u8]` when writing instance buffers. Guaranteed to be
/// `0` or `1` by construction; shaders read it as a `u32`/`uint`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PaddedBool32(u32);

impl From<bool> for PaddedBool32 {
    fn from(value: bool) -> Self {
        PaddedBool32(value as u32)
    }
}

#[derive(Default)]
#[expect(missing_docs)]
pub struct Scene {
    pub(crate) paint_operations: Vec<PaintOperation>,
    primitive_bounds: BoundsTree<ScaledPixels>,
    layer_stack: Vec<DrawOrder>,
    pub shadows: Vec<Shadow>,
    pub quads: Vec<Quad>,
    pub paths: Vec<Path<ScaledPixels>>,
    pub underlines: Vec<Underline>,
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub subpixel_sprites: Vec<SubpixelSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
    pub surfaces: Vec<PaintSurface>,
    path_indices: Vec<usize>,
    surface_indices: Vec<usize>,
    #[cfg(debug_assertions)]
    consumed_segments: Vec<Range<usize>>,
}

#[expect(missing_docs)]
impl Scene {
    pub fn clear(&mut self) {
        self.paint_operations.clear();
        self.primitive_bounds.clear();
        self.layer_stack.clear();
        self.paths.clear();
        self.shadows.clear();
        self.quads.clear();
        self.underlines.clear();
        self.monochrome_sprites.clear();
        self.subpixel_sprites.clear();
        self.polychrome_sprites.clear();
        self.surfaces.clear();
        self.path_indices.clear();
        self.surface_indices.clear();
        #[cfg(debug_assertions)]
        self.consumed_segments.clear();
    }

    pub fn len(&self) -> usize {
        self.paint_operations.len()
    }

    pub fn push_layer(&mut self, bounds: Bounds<ScaledPixels>) {
        let order = self.primitive_bounds.insert(bounds);
        self.layer_stack.push(order);
        self.paint_operations
            .push(PaintOperation::StartLayer(bounds));
    }

    pub fn pop_layer(&mut self) {
        self.layer_stack.pop();
        self.paint_operations.push(PaintOperation::EndLayer);
    }

    pub fn insert_primitive(&mut self, primitive: impl Into<Primitive>) {
        if let Some(operation) = self.prepare_primitive(primitive.into()) {
            self.paint_operations.push(operation);
        }
    }

    fn prepare_primitive(&mut self, primitive: Primitive) -> Option<PaintOperation> {
        let clipped_bounds = primitive
            .bounds()
            .intersect(&primitive.content_mask().bounds);

        if clipped_bounds.is_empty() {
            return None;
        }

        let order = self
            .layer_stack
            .last()
            .copied()
            .unwrap_or_else(|| self.primitive_bounds.insert(clipped_bounds));
        let primitive = match primitive {
            Primitive::Shadow(mut shadow) => {
                shadow.order = order;
                self.shadows.push(shadow);
                RecordedPrimitive::Shadow(shadow)
            }
            Primitive::Quad(mut quad) => {
                quad.order = order;
                self.quads.push(quad);
                RecordedPrimitive::Quad(quad)
            }
            Primitive::Path(mut path) => {
                path.order = order;
                path.id = PathId(self.paths.len());
                let record = RecordedPrimitive::Path {
                    id: path.id,
                    bounds: path.bounds,
                    content_mask: path.content_mask,
                };
                self.paths.push(path);
                record
            }
            Primitive::Underline(mut underline) => {
                underline.order = order;
                self.underlines.push(underline);
                RecordedPrimitive::Underline(underline)
            }
            Primitive::MonochromeSprite(mut sprite) => {
                sprite.order = order;
                self.monochrome_sprites.push(sprite);
                RecordedPrimitive::MonochromeSprite(sprite)
            }
            Primitive::SubpixelSprite(mut sprite) => {
                sprite.order = order;
                self.subpixel_sprites.push(sprite);
                RecordedPrimitive::SubpixelSprite(sprite)
            }
            Primitive::PolychromeSprite(mut sprite) => {
                sprite.order = order;
                self.polychrome_sprites.push(sprite);
                RecordedPrimitive::PolychromeSprite(sprite)
            }
            Primitive::Surface(mut surface) => {
                surface.order = order;
                let record = RecordedPrimitive::Surface {
                    index: self.surfaces.len(),
                    bounds: surface.bounds,
                    content_mask: surface.content_mask,
                };
                self.surfaces.push(surface);
                record
            }
        };
        Some(PaintOperation::Primitive(primitive))
    }

    pub fn replay(
        &mut self,
        range: Range<usize>,
        prev_scene: &mut Scene,
        #[cfg(any(test, feature = "bench-support"))] stats: &mut crate::ViewReuseStats,
    ) {
        #[cfg(debug_assertions)]
        if !range.is_empty() {
            debug_assert!(
                prev_scene
                    .consumed_segments
                    .iter()
                    .all(|prior| { range.end <= prior.start || prior.end <= range.start }),
                "scene segment consumed twice"
            );
            prev_scene.consumed_segments.push(range.clone());
        }
        let start = self.paint_operations.len();
        self.paint_operations
            .extend_from_slice(&prev_scene.paint_operations[range]);
        for index in start..self.paint_operations.len() {
            let operation = self.paint_operations[index];
            #[cfg(any(test, feature = "bench-support"))]
            match operation {
                PaintOperation::Primitive(primitive) => {
                    match primitive {
                        RecordedPrimitive::Shadow(value) => {
                            stats.scene_primitives[0].copied(std::slice::from_ref(&value))
                        }
                        RecordedPrimitive::Quad(value) => {
                            stats.scene_primitives[1].copied(std::slice::from_ref(&value))
                        }
                        RecordedPrimitive::Path { id, .. } => {
                            stats.scene_primitives[2]
                                .moved(std::slice::from_ref(prev_scene.path(id)));
                            stats.index_fixups += 1;
                        }
                        RecordedPrimitive::Underline(value) => {
                            stats.scene_primitives[3].copied(std::slice::from_ref(&value))
                        }
                        RecordedPrimitive::MonochromeSprite(value) => {
                            stats.scene_primitives[4].copied(std::slice::from_ref(&value))
                        }
                        RecordedPrimitive::SubpixelSprite(value) => {
                            stats.scene_primitives[5].copied(std::slice::from_ref(&value))
                        }
                        RecordedPrimitive::PolychromeSprite(value) => {
                            stats.scene_primitives[6].copied(std::slice::from_ref(&value))
                        }
                        RecordedPrimitive::Surface { index, .. } => stats.scene_primitives[7]
                            .cloned(std::slice::from_ref(prev_scene.surface(index))),
                    }
                    stats.index_fixups += 1;
                }
                PaintOperation::StartLayer(_) | PaintOperation::EndLayer => {
                    stats.scene_layer_operations += 1;
                }
            }
            match operation {
                PaintOperation::Primitive(primitive) => {
                    let primitive = match primitive {
                        RecordedPrimitive::Shadow(value) => Primitive::Shadow(value),
                        RecordedPrimitive::Quad(value) => Primitive::Quad(value),
                        RecordedPrimitive::Underline(value) => Primitive::Underline(value),
                        RecordedPrimitive::MonochromeSprite(value) => {
                            Primitive::MonochromeSprite(value)
                        }
                        RecordedPrimitive::SubpixelSprite(value) => {
                            Primitive::SubpixelSprite(value)
                        }
                        RecordedPrimitive::PolychromeSprite(value) => {
                            Primitive::PolychromeSprite(value)
                        }
                        RecordedPrimitive::Path {
                            id,
                            bounds,
                            content_mask,
                        } => {
                            let source_index = prev_scene.path_index(id);
                            let path = &mut prev_scene.paths[source_index];
                            debug_assert_eq!(path.bounds, bounds);
                            debug_assert_eq!(path.content_mask, content_mask);
                            Primitive::Path(Path {
                                id: path.id,
                                order: path.order,
                                bounds: path.bounds,
                                content_mask: path.content_mask,
                                vertices: std::mem::take(&mut path.vertices),
                                color: path.color,
                                start: path.start,
                                current: path.current,
                                contour_count: path.contour_count,
                            })
                        }
                        RecordedPrimitive::Surface {
                            index,
                            bounds,
                            content_mask,
                        } => {
                            let surface = prev_scene.surface(index);
                            debug_assert_eq!(surface.bounds, bounds);
                            debug_assert_eq!(surface.content_mask, content_mask);
                            Primitive::Surface(surface.clone())
                        }
                    };
                    if let Some(operation) = self.prepare_primitive(primitive) {
                        self.paint_operations[index] = operation;
                    }
                }
                PaintOperation::StartLayer(bounds) => {
                    let order = self.primitive_bounds.insert(bounds);
                    self.layer_stack.push(order);
                }
                PaintOperation::EndLayer => {
                    self.layer_stack.pop();
                }
            }
        }
    }

    fn path_index(&self, id: PathId) -> usize {
        self.path_indices.get(id.0).copied().unwrap_or(id.0)
    }

    #[cfg(any(test, feature = "bench-support"))]
    fn path(&self, id: PathId) -> &Path<ScaledPixels> {
        &self.paths[self.path_index(id)]
    }

    fn surface(&self, index: usize) -> &PaintSurface {
        &self.surfaces[self.surface_indices.get(index).copied().unwrap_or(index)]
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn reuse_snapshot(&self) -> Vec<String> {
        self.paint_operations
            .iter()
            .map(|operation| match operation {
                PaintOperation::Primitive(primitive) => match primitive {
                    RecordedPrimitive::Shadow(value) => format!("shadow:{value:?}"),
                    RecordedPrimitive::Quad(value) => format!("quad:{value:?}"),
                    RecordedPrimitive::Path { id, .. } => format!("path:{:?}", self.path(*id)),
                    RecordedPrimitive::Underline(value) => format!("underline:{value:?}"),
                    RecordedPrimitive::MonochromeSprite(value) => format!("monochrome:{value:?}"),
                    RecordedPrimitive::SubpixelSprite(value) => format!("subpixel:{value:?}"),
                    RecordedPrimitive::PolychromeSprite(value) => format!("polychrome:{value:?}"),
                    RecordedPrimitive::Surface { index, .. } => {
                        format!("surface:{:?}", self.surface(*index))
                    }
                },
                PaintOperation::StartLayer(bounds) => format!("layer:{bounds:?}"),
                PaintOperation::EndLayer => "end-layer".into(),
            })
            .collect()
    }

    pub fn finish(&mut self) {
        self.shadows.sort_by_key(|shadow| shadow.order);
        self.quads.sort_by_key(|quad| quad.order);
        self.paths.sort_by_key(|path| path.order);
        self.path_indices.resize(self.paths.len(), 0);
        for (index, path) in self.paths.iter().enumerate() {
            self.path_indices[path.id.0] = index;
        }
        self.underlines.sort_by_key(|underline| underline.order);
        self.monochrome_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.subpixel_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.polychrome_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        if !self.surfaces.is_empty() {
            let mut surfaces: Vec<_> = self.surfaces.drain(..).enumerate().collect();
            surfaces.sort_by_key(|(_, surface)| surface.order);
            self.surface_indices
                .extend(self.surface_indices.len()..surfaces.len());
            let mut permutation = vec![0; surfaces.len()];
            for (index, (old_index, surface)) in surfaces.into_iter().enumerate() {
                permutation[old_index] = index;
                self.surfaces.push(surface);
            }
            for index in &mut self.surface_indices {
                *index = permutation[*index];
            }
        }
    }

    #[cfg_attr(
        all(
            any(target_os = "linux", target_os = "freebsd"),
            not(any(feature = "x11", feature = "wayland"))
        ),
        allow(dead_code)
    )]
    pub fn batches(&self) -> impl Iterator<Item = PrimitiveBatch> + '_ {
        BatchIterator {
            shadows_start: 0,
            shadows_iter: self.shadows.iter().peekable(),
            quads_start: 0,
            quads_iter: self.quads.iter().peekable(),
            paths_start: 0,
            paths_iter: self.paths.iter().peekable(),
            underlines_start: 0,
            underlines_iter: self.underlines.iter().peekable(),
            monochrome_sprites_start: 0,
            monochrome_sprites_iter: self.monochrome_sprites.iter().peekable(),
            subpixel_sprites_start: 0,
            subpixel_sprites_iter: self.subpixel_sprites.iter().peekable(),
            polychrome_sprites_start: 0,
            polychrome_sprites_iter: self.polychrome_sprites.iter().peekable(),
            surfaces_start: 0,
            surfaces_iter: self.surfaces.iter().peekable(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
pub(crate) enum PrimitiveKind {
    Shadow,
    #[default]
    Quad,
    Path,
    Underline,
    MonochromeSprite,
    SubpixelSprite,
    PolychromeSprite,
    Surface,
}

#[derive(Clone, Copy)]
pub(crate) enum PaintOperation {
    Primitive(RecordedPrimitive),
    StartLayer(Bounds<ScaledPixels>),
    EndLayer,
}

#[derive(Clone, Copy)]
pub(crate) enum RecordedPrimitive {
    Shadow(Shadow),
    Quad(Quad),
    Path {
        id: PathId,
        bounds: Bounds<ScaledPixels>,
        content_mask: ContentMask<ScaledPixels>,
    },
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    SubpixelSprite(SubpixelSprite),
    PolychromeSprite(PolychromeSprite),
    Surface {
        index: usize,
        bounds: Bounds<ScaledPixels>,
        content_mask: ContentMask<ScaledPixels>,
    },
}

#[derive(Clone)]
#[expect(missing_docs)]
pub enum Primitive {
    Shadow(Shadow),
    Quad(Quad),
    Path(Path<ScaledPixels>),
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    SubpixelSprite(SubpixelSprite),
    PolychromeSprite(PolychromeSprite),
    Surface(PaintSurface),
}

#[expect(missing_docs)]
impl Primitive {
    pub fn bounds(&self) -> &Bounds<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.bounds,
            Primitive::Quad(quad) => &quad.bounds,
            Primitive::Path(path) => &path.bounds,
            Primitive::Underline(underline) => &underline.bounds,
            Primitive::MonochromeSprite(sprite) => &sprite.bounds,
            Primitive::SubpixelSprite(sprite) => &sprite.bounds,
            Primitive::PolychromeSprite(sprite) => &sprite.bounds,
            Primitive::Surface(surface) => &surface.bounds,
        }
    }

    pub fn content_mask(&self) -> &ContentMask<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.content_mask,
            Primitive::Quad(quad) => &quad.content_mask,
            Primitive::Path(path) => &path.content_mask,
            Primitive::Underline(underline) => &underline.content_mask,
            Primitive::MonochromeSprite(sprite) => &sprite.content_mask,
            Primitive::SubpixelSprite(sprite) => &sprite.content_mask,
            Primitive::PolychromeSprite(sprite) => &sprite.content_mask,
            Primitive::Surface(surface) => &surface.content_mask,
        }
    }
}

#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
struct BatchIterator<'a> {
    shadows_start: usize,
    shadows_iter: Peekable<slice::Iter<'a, Shadow>>,
    quads_start: usize,
    quads_iter: Peekable<slice::Iter<'a, Quad>>,
    paths_start: usize,
    paths_iter: Peekable<slice::Iter<'a, Path<ScaledPixels>>>,
    underlines_start: usize,
    underlines_iter: Peekable<slice::Iter<'a, Underline>>,
    monochrome_sprites_start: usize,
    monochrome_sprites_iter: Peekable<slice::Iter<'a, MonochromeSprite>>,
    subpixel_sprites_start: usize,
    subpixel_sprites_iter: Peekable<slice::Iter<'a, SubpixelSprite>>,
    polychrome_sprites_start: usize,
    polychrome_sprites_iter: Peekable<slice::Iter<'a, PolychromeSprite>>,
    surfaces_start: usize,
    surfaces_iter: Peekable<slice::Iter<'a, PaintSurface>>,
}

impl<'a> Iterator for BatchIterator<'a> {
    type Item = PrimitiveBatch;

    fn next(&mut self) -> Option<Self::Item> {
        let mut orders_and_kinds = [
            (
                self.shadows_iter.peek().map(|s| s.order),
                PrimitiveKind::Shadow,
            ),
            (self.quads_iter.peek().map(|q| q.order), PrimitiveKind::Quad),
            (self.paths_iter.peek().map(|q| q.order), PrimitiveKind::Path),
            (
                self.underlines_iter.peek().map(|u| u.order),
                PrimitiveKind::Underline,
            ),
            (
                self.monochrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::MonochromeSprite,
            ),
            (
                self.subpixel_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::SubpixelSprite,
            ),
            (
                self.polychrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::PolychromeSprite,
            ),
            (
                self.surfaces_iter.peek().map(|s| s.order),
                PrimitiveKind::Surface,
            ),
        ];
        orders_and_kinds.sort_by_key(|(order, kind)| (order.unwrap_or(u32::MAX), *kind));

        let first = orders_and_kinds[0];
        let second = orders_and_kinds[1];
        let (batch_kind, max_order_and_kind) = if first.0.is_some() {
            (first.1, (second.0.unwrap_or(u32::MAX), second.1))
        } else {
            return None;
        };

        match batch_kind {
            PrimitiveKind::Shadow => {
                let shadows_start = self.shadows_start;
                let mut shadows_end = shadows_start + 1;
                self.shadows_iter.next();
                while self
                    .shadows_iter
                    .next_if(|shadow| (shadow.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    shadows_end += 1;
                }
                self.shadows_start = shadows_end;
                Some(PrimitiveBatch::Shadows(shadows_start..shadows_end))
            }
            PrimitiveKind::Quad => {
                let quads_start = self.quads_start;
                let mut quads_end = quads_start + 1;
                self.quads_iter.next();
                while self
                    .quads_iter
                    .next_if(|quad| (quad.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    quads_end += 1;
                }
                self.quads_start = quads_end;
                Some(PrimitiveBatch::Quads(quads_start..quads_end))
            }
            PrimitiveKind::Path => {
                let paths_start = self.paths_start;
                let mut paths_end = paths_start + 1;
                self.paths_iter.next();
                while self
                    .paths_iter
                    .next_if(|path| (path.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    paths_end += 1;
                }
                self.paths_start = paths_end;
                Some(PrimitiveBatch::Paths(paths_start..paths_end))
            }
            PrimitiveKind::Underline => {
                let underlines_start = self.underlines_start;
                let mut underlines_end = underlines_start + 1;
                self.underlines_iter.next();
                while self
                    .underlines_iter
                    .next_if(|underline| (underline.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    underlines_end += 1;
                }
                self.underlines_start = underlines_end;
                Some(PrimitiveBatch::Underlines(underlines_start..underlines_end))
            }
            PrimitiveKind::MonochromeSprite => {
                let texture_id = self.monochrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.monochrome_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.monochrome_sprites_iter.next();
                while self
                    .monochrome_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.monochrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    range: sprites_start..sprites_end,
                })
            }
            PrimitiveKind::SubpixelSprite => {
                let texture_id = self.subpixel_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.subpixel_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.subpixel_sprites_iter.next();
                while self
                    .subpixel_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.subpixel_sprites_start = sprites_end;
                Some(PrimitiveBatch::SubpixelSprites {
                    texture_id,
                    range: sprites_start..sprites_end,
                })
            }
            PrimitiveKind::PolychromeSprite => {
                let texture_id = self.polychrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.polychrome_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.polychrome_sprites_iter.next();
                while self
                    .polychrome_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.polychrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    range: sprites_start..sprites_end,
                })
            }
            PrimitiveKind::Surface => {
                let surfaces_start = self.surfaces_start;
                let mut surfaces_end = surfaces_start + 1;
                self.surfaces_iter.next();
                while self
                    .surfaces_iter
                    .next_if(|surface| (surface.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    surfaces_end += 1;
                }
                self.surfaces_start = surfaces_end;
                Some(PrimitiveBatch::Surfaces(surfaces_start..surfaces_end))
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
#[allow(missing_docs)]
pub enum PrimitiveBatch {
    Shadows(Range<usize>),
    Quads(Range<usize>),
    Paths(Range<usize>),
    Underlines(Range<usize>),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    SubpixelSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    Surfaces(Range<usize>),
}

impl PrimitiveBatch {
    #[expect(missing_docs)]
    pub fn label(&self) -> String {
        match self {
            Self::Shadows(range) => format!("shadows ({})", range.len()),
            Self::Quads(range) => format!("quads ({})", range.len()),
            Self::Paths(range) => format!("paths ({})", range.len()),
            Self::Underlines(range) => format!("underlines ({})", range.len()),
            Self::MonochromeSprites { texture_id, range } => {
                format!(
                    "monochrome sprites ({}) on atlas {}",
                    range.len(),
                    texture_id.index
                )
            }
            Self::SubpixelSprites { texture_id, range } => {
                format!(
                    "subpixel sprites ({}) on atlas {}",
                    range.len(),
                    texture_id.index
                )
            }
            Self::PolychromeSprites { texture_id, range } => {
                format!(
                    "polychrome sprites ({}) on atlas {}",
                    range.len(),
                    texture_id.index
                )
            }
            Self::Surfaces(range) => format!("surfaces ({})", range.len()),
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
#[expect(missing_docs)]
pub struct Quad {
    pub order: DrawOrder,
    pub border_style: BorderStyle,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub background: Background,
    pub border_color: Hsla,
    pub corner_radii: Corners<ScaledPixels>,
    pub border_widths: Edges<ScaledPixels>,
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[expect(missing_docs)]
pub struct Underline {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub thickness: ScaledPixels,
    pub wavy: PaddedBool32,
}

impl From<Underline> for Primitive {
    fn from(underline: Underline) -> Self {
        Primitive::Underline(underline)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[expect(missing_docs)]
pub struct Shadow {
    pub order: DrawOrder,
    pub blur_radius: ScaledPixels,
    pub bounds: Bounds<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub element_bounds: Bounds<ScaledPixels>,
    pub element_corner_radii: Corners<ScaledPixels>,
    /// 0 = drop shadow (rendered outside the element), 1 = inset shadow (rendered inside).
    pub inset: u32,
    pub pad: u32, // align to 8 bytes
}

impl From<Shadow> for Primitive {
    fn from(shadow: Shadow) -> Self {
        Primitive::Shadow(shadow)
    }
}

/// The style of a border.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[repr(C)]
pub enum BorderStyle {
    /// A solid border.
    #[default]
    Solid = 0,
    /// A dashed border.
    Dashed = 1,
}

/// A data type representing a 2 dimensional transformation that can be applied to an element.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TransformationMatrix {
    /// 2x2 matrix containing rotation and scale,
    /// stored row-major
    pub rotation_scale: [[f32; 2]; 2],
    /// translation vector
    pub translation: [f32; 2],
}

impl Eq for TransformationMatrix {}

impl TransformationMatrix {
    /// The unit matrix, has no effect.
    pub fn unit() -> Self {
        Self {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [0.0, 0.0],
        }
    }

    /// Move the origin by a given point
    pub fn translate(mut self, point: Point<ScaledPixels>) -> Self {
        self.compose(Self {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [point.x.0, point.y.0],
        })
    }

    /// Clockwise rotation in radians around the origin
    pub fn rotate(self, angle: Radians) -> Self {
        self.compose(Self {
            rotation_scale: [
                [angle.0.cos(), -angle.0.sin()],
                [angle.0.sin(), angle.0.cos()],
            ],
            translation: [0.0, 0.0],
        })
    }

    /// Scale around the origin
    pub fn scale(self, size: Size<f32>) -> Self {
        self.compose(Self {
            rotation_scale: [[size.width, 0.0], [0.0, size.height]],
            translation: [0.0, 0.0],
        })
    }

    /// Perform matrix multiplication with another transformation
    /// to produce a new transformation that is the result of
    /// applying both transformations: first, `other`, then `self`.
    #[inline]
    pub fn compose(self, other: TransformationMatrix) -> TransformationMatrix {
        if other == Self::unit() {
            return self;
        }
        // Perform matrix multiplication
        TransformationMatrix {
            rotation_scale: [
                [
                    self.rotation_scale[0][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][0],
                    self.rotation_scale[0][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][1],
                ],
                [
                    self.rotation_scale[1][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][0],
                    self.rotation_scale[1][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][1],
                ],
            ],
            translation: [
                self.translation[0]
                    + self.rotation_scale[0][0] * other.translation[0]
                    + self.rotation_scale[0][1] * other.translation[1],
                self.translation[1]
                    + self.rotation_scale[1][0] * other.translation[0]
                    + self.rotation_scale[1][1] * other.translation[1],
            ],
        }
    }

    /// Apply transformation to a point, mainly useful for debugging
    pub fn apply(&self, point: Point<Pixels>) -> Point<Pixels> {
        let input = [point.x.0, point.y.0];
        let mut output = self.translation;
        for (i, output_cell) in output.iter_mut().enumerate() {
            for (k, input_cell) in input.iter().enumerate() {
                *output_cell += self.rotation_scale[i][k] * *input_cell;
            }
        }
        Point::new(output[0].into(), output[1].into())
    }
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self::unit()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct MonochromeSprite {
    pub order: DrawOrder,
    pub pad: u32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub tile: AtlasTile,
    pub transformation: TransformationMatrix,
}

impl From<MonochromeSprite> for Primitive {
    fn from(sprite: MonochromeSprite) -> Self {
        Primitive::MonochromeSprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct SubpixelSprite {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub tile: AtlasTile,
    pub transformation: TransformationMatrix,
}

impl From<SubpixelSprite> for Primitive {
    fn from(sprite: SubpixelSprite) -> Self {
        Primitive::SubpixelSprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct PolychromeSprite {
    pub order: DrawOrder,
    pub pad: u32,
    pub grayscale: PaddedBool32,
    pub opacity: f32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub tile: AtlasTile,
}

impl From<PolychromeSprite> for Primitive {
    fn from(sprite: PolychromeSprite) -> Self {
        Primitive::PolychromeSprite(sprite)
    }
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct PaintSurface {
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub image_buffer: core_video::pixel_buffer::CVPixelBuffer,
}

impl From<PaintSurface> for Primitive {
    fn from(surface: PaintSurface) -> Self {
        Primitive::Surface(surface)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[expect(missing_docs)]
pub struct PathId(pub usize);

/// A line made up of a series of vertices and control points.
#[derive(Clone, Debug)]
#[expect(missing_docs)]
pub struct Path<P: Clone + Debug + Default + PartialEq> {
    pub id: PathId,
    pub order: DrawOrder,
    pub bounds: Bounds<P>,
    pub content_mask: ContentMask<P>,
    pub vertices: Vec<PathVertex<P>>,
    pub color: Background,
    start: Point<P>,
    current: Point<P>,
    contour_count: usize,
}

impl Path<Pixels> {
    /// Create a new path with the given starting point.
    pub fn new(start: Point<Pixels>) -> Self {
        Self {
            id: PathId(0),
            order: DrawOrder::default(),
            vertices: Vec::new(),
            start,
            current: start,
            bounds: Bounds {
                origin: start,
                size: Default::default(),
            },
            content_mask: Default::default(),
            color: Default::default(),
            contour_count: 0,
        }
    }

    /// Scale this path by the given factor.
    pub fn scale(&self, factor: f32) -> Path<ScaledPixels> {
        Path {
            id: self.id,
            order: self.order,
            bounds: self.bounds.scale(factor),
            content_mask: self.content_mask.scale(factor),
            vertices: self
                .vertices
                .iter()
                .map(|vertex| vertex.scale(factor))
                .collect(),
            start: self.start.map(|start| start.scale(factor)),
            current: self.current.scale(factor),
            contour_count: self.contour_count,
            color: self.color,
        }
    }

    /// Move the start, current point to the given point.
    pub fn move_to(&mut self, to: Point<Pixels>) {
        self.contour_count += 1;
        self.start = to;
        self.current = to;
    }

    /// Draw a straight line from the current point to the given point.
    pub fn line_to(&mut self, to: Point<Pixels>) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(
                (self.start, self.current, to),
                (point(0., 1.), point(0., 1.), point(0., 1.)),
            );
        }
        self.current = to;
    }

    /// Draw a curve from the current point to the given point, using the given control point.
    pub fn curve_to(&mut self, to: Point<Pixels>, ctrl: Point<Pixels>) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(
                (self.start, self.current, to),
                (point(0., 1.), point(0., 1.), point(0., 1.)),
            );
        }

        self.push_triangle(
            (self.current, ctrl, to),
            (point(0., 0.), point(0.5, 0.), point(1., 1.)),
        );
        self.current = to;
    }

    /// Push a triangle to the Path.
    pub fn push_triangle(
        &mut self,
        xy: (Point<Pixels>, Point<Pixels>, Point<Pixels>),
        st: (Point<f32>, Point<f32>, Point<f32>),
    ) {
        self.bounds = self
            .bounds
            .union(&Bounds {
                origin: xy.0,
                size: Default::default(),
            })
            .union(&Bounds {
                origin: xy.1,
                size: Default::default(),
            })
            .union(&Bounds {
                origin: xy.2,
                size: Default::default(),
            });

        self.vertices.push(PathVertex {
            xy_position: xy.0,
            st_position: st.0,
            content_mask: Default::default(),
        });
        self.vertices.push(PathVertex {
            xy_position: xy.1,
            st_position: st.1,
            content_mask: Default::default(),
        });
        self.vertices.push(PathVertex {
            xy_position: xy.2,
            st_position: st.2,
            content_mask: Default::default(),
        });
    }
}

impl<T> Path<T>
where
    T: Clone + Debug + Default + PartialEq + PartialOrd + Add<T, Output = T> + Sub<Output = T>,
{
    #[allow(unused)]
    #[expect(missing_docs)]
    pub fn clipped_bounds(&self) -> Bounds<T> {
        self.bounds.intersect(&self.content_mask.bounds)
    }
}

impl From<Path<ScaledPixels>> for Primitive {
    fn from(path: Path<ScaledPixels>) -> Self {
        Primitive::Path(path)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct PathVertex<P: Clone + Debug + Default + PartialEq> {
    pub xy_position: Point<P>,
    pub st_position: Point<f32>,
    pub content_mask: ContentMask<P>,
}

#[expect(missing_docs)]
impl PathVertex<Pixels> {
    pub fn scale(&self, factor: f32) -> PathVertex<ScaledPixels> {
        PathVertex {
            xy_position: self.xy_position.scale(factor),
            st_position: self.st_position,
            content_mask: self.content_mask.scale(factor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{px, size};

    fn quad() -> Quad {
        let bounds = Bounds::new(point(px(0.), px(0.)), size(px(20.), px(20.))).scale(1.);
        Quad {
            bounds,
            content_mask: ContentMask { bounds },
            ..Quad::default()
        }
    }

    fn path() -> Path<ScaledPixels> {
        let mut path = Path::new(point(px(0.), px(0.)));
        path.line_to(point(px(10.), px(0.)));
        path.line_to(point(px(0.), px(10.)));
        path.content_mask = ContentMask {
            bounds: Bounds::new(point(px(0.), px(0.)), size(px(20.), px(20.))),
        };
        path.scale(1.)
    }

    #[test]
    fn replay_moves_paths_and_recomputes_order_in_nested_layers() {
        let mut source = Scene::default();
        source.insert_primitive(quad());
        source.push_layer(quad().bounds);
        source.insert_primitive(path());
        source.push_layer(quad().bounds);
        source.insert_primitive(path());
        source.pop_layer();
        source.pop_layer();
        source.insert_primitive(path());
        source.finish();
        let old_bounds: Vec<_> = source
            .paths
            .iter()
            .map(|path| (path.id, path.bounds))
            .collect();
        let allocations: Vec<_> = source
            .paths
            .iter()
            .map(|path| path.vertices.as_ptr())
            .collect();
        let mut target = Scene::default();
        let mut fresh = Scene::default();
        for scene in [&mut target, &mut fresh] {
            scene.insert_primitive(quad());
            scene.insert_primitive(quad());
        }
        let mut stats = crate::ViewReuseStats::default();
        target.replay(0..source.len(), &mut source, &mut stats);
        fresh.insert_primitive(quad());
        fresh.push_layer(quad().bounds);
        fresh.insert_primitive(path());
        fresh.push_layer(quad().bounds);
        fresh.insert_primitive(path());
        fresh.pop_layer();
        fresh.pop_layer();
        fresh.insert_primitive(path());
        target.finish();
        fresh.finish();
        assert_eq!(target.reuse_snapshot(), fresh.reuse_snapshot());
        assert_eq!(
            target
                .paths
                .iter()
                .map(|path| path.vertices.as_ptr())
                .collect::<Vec<_>>(),
            allocations
        );
        assert!(source.paths.iter().all(|path| path.vertices.is_empty()));
        assert_eq!(
            source
                .paths
                .iter()
                .map(|path| (path.id, path.bounds))
                .collect::<Vec<_>>(),
            old_bounds
        );
        assert_eq!(stats.scene_primitives[2].cloned_elements, 0);
        assert_eq!(stats.scene_primitives[2].moved_elements, 3);
        assert_eq!(stats.scene_primitives[1].copied_elements, 1);

        let mut next = Scene::default();
        next.replay(0..target.len(), &mut target, &mut stats);
        next.finish();
        assert_eq!(next.reuse_snapshot(), fresh.reuse_snapshot());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "scene segment consumed twice")]
    fn replay_rejects_double_scene_consumption() {
        let mut source = Scene::default();
        source.insert_primitive(path());
        source.finish();
        let mut target = Scene::default();
        let mut stats = crate::ViewReuseStats::default();
        target.replay(0..1, &mut source, &mut stats);
        target.replay(0..1, &mut source, &mut stats);
    }

    #[test]
    fn replay_resolves_path_ids_after_painter_order_sorting() {
        let mut source = Scene::default();
        source.insert_primitive(quad());
        source.insert_primitive(path());
        let mut separate = path();
        separate.bounds.origin.x = ScaledPixels(100.);
        separate.content_mask.bounds = separate.bounds;
        source.insert_primitive(separate);
        source.finish();
        source.finish();
        assert_eq!(source.paths[0].id, PathId(1));
        let expected = source.reuse_snapshot();
        let mut target = Scene::default();
        let mut stats = crate::ViewReuseStats::default();
        target.replay(0..source.len(), &mut source, &mut stats);
        target.finish();
        assert_eq!(target.reuse_snapshot(), expected);
    }
}
