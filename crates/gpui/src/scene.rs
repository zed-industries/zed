mod primitives;

use crate::{
    AtlasTextureId, BoundsSearchResult, BoundsTree, EntityId, Point, ScaledPixels, SharedString,
};
use collections::{FxHashMap, FxHashSet};
pub use primitives::*;
use std::{cmp::Reverse, fmt::Debug, iter::Peekable, slice};

#[derive(Default)]
pub(crate) struct Scene {
    pub(crate) shadows: PrimitiveSet<Shadow>,
    pub(crate) quads: PrimitiveSet<Quad>,
    pub(crate) paths: PrimitiveSet<Path<ScaledPixels>>,
    pub(crate) underlines: PrimitiveSet<Underline>,
    pub(crate) monochrome_sprites: PrimitiveSet<MonochromeSprite>,
    pub(crate) polychrome_sprites: PrimitiveSet<PolychromeSprite>,
    pub(crate) surfaces: PrimitiveSet<Surface>,
    bounds_tree: BoundsTree<ScaledPixels, PrimitiveIndex>,
    hovered_bounds: Vec<BoundsSearchResult<ScaledPixels, PrimitiveIndex>>,
}

impl Scene {
    pub fn clear(&mut self) {
        self.shadows.clear();
        self.quads.clear();
        self.paths.clear();
        self.underlines.clear();
        self.monochrome_sprites.clear();
        self.polychrome_sprites.clear();
        self.surfaces.clear();
        self.bounds_tree.clear();
    }

    pub fn paths(&self) -> &[Path<ScaledPixels>] {
        &self.paths.primitives
    }

    pub(crate) fn batches(&self) -> impl Iterator<Item = PrimitiveBatch> {
        BatchIterator {
            shadows: &self.shadows.primitives,
            shadows_start: 0,
            shadows_iter: self.shadows.primitives.iter().peekable(),
            quads: &self.quads.primitives,
            quads_start: 0,
            quads_iter: self.quads.primitives.iter().peekable(),
            paths: &self.paths.primitives,
            paths_start: 0,
            paths_iter: self.paths.primitives.iter().peekable(),
            underlines: &self.underlines.primitives,
            underlines_start: 0,
            underlines_iter: self.underlines.primitives.iter().peekable(),
            monochrome_sprites: &self.monochrome_sprites.primitives,
            monochrome_sprites_start: 0,
            monochrome_sprites_iter: self.monochrome_sprites.primitives.iter().peekable(),
            polychrome_sprites: &self.polychrome_sprites.primitives,
            polychrome_sprites_start: 0,
            polychrome_sprites_iter: self.polychrome_sprites.primitives.iter().peekable(),
            surfaces: &self.surfaces.primitives,
            surfaces_start: 0,
            surfaces_iter: self.surfaces.primitives.iter().peekable(),
        }
    }

    pub(crate) fn insert_shadow(
        &mut self,
        shadow: Shadow,
        hover: Option<Shadow>,
        group_hover: Option<(SharedString, Option<Shadow>)>,
    ) -> Option<u32> {
        let clipped_bounds = shadow.bounds.intersect(&shadow.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::Shadow,
                index: self.shadows.primitives.len(),
            },
        );
        self.shadows
            .insert(Shadow { order, ..shadow }, false, hover, group_hover);
        Some(order)
    }

    pub(crate) fn insert_quad(
        &mut self,
        quad: Quad,
        hover: Option<Quad>,
        group_hover: Option<(SharedString, Option<Quad>)>,
    ) -> Option<u32> {
        let clipped_bounds = quad.bounds.intersect(&quad.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::Quad,
                index: self.quads.primitives.len(),
            },
        );
        self.quads.insert(
            Quad { order, ..quad },
            quad.background.is_opaque(),
            hover.map(|quad| Quad { order, ..quad }),
            group_hover.map(|(group_id, quad)| (group_id, quad.map(|quad| Quad { order, ..quad }))),
        );
        Some(order)
    }

    pub(crate) fn insert_path(
        &mut self,
        path: Path<ScaledPixels>,
        hover: Option<Path<ScaledPixels>>,
        group_hover: Option<(SharedString, Option<Path<ScaledPixels>>)>,
    ) -> Option<u32> {
        let clipped_bounds = path.bounds.intersect(&path.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::Path,
                index: self.paths.primitives.len(),
            },
        );
        self.paths
            .insert(Path { order, ..path }, false, hover, group_hover);
        Some(order)
    }

    pub(crate) fn insert_underline(
        &mut self,
        underline: Underline,
        hover: Option<Underline>,
        group_hover: Option<(SharedString, Option<Underline>)>,
    ) -> Option<u32> {
        let clipped_bounds = underline.bounds.intersect(&underline.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::Underline,
                index: self.underlines.primitives.len(),
            },
        );
        self.underlines
            .insert(Underline { order, ..underline }, false, hover, group_hover);
        Some(order)
    }

    pub(crate) fn insert_monochrome_sprite(
        &mut self,
        monochrome_sprite: MonochromeSprite,
        hover: Option<MonochromeSprite>,
        group_hover: Option<(SharedString, Option<MonochromeSprite>)>,
    ) -> Option<u32> {
        let clipped_bounds = monochrome_sprite
            .bounds
            .intersect(&monochrome_sprite.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::MonochromeSprite,
                index: self.monochrome_sprites.primitives.len(),
            },
        );
        self.monochrome_sprites.insert(
            MonochromeSprite {
                order,
                ..monochrome_sprite
            },
            false,
            hover,
            group_hover,
        );
        Some(order)
    }

    pub(crate) fn insert_polychrome_sprite(
        &mut self,
        polychrome_sprite: PolychromeSprite,
        hover: Option<PolychromeSprite>,
        group_hover: Option<(SharedString, Option<PolychromeSprite>)>,
    ) -> Option<u32> {
        let clipped_bounds = polychrome_sprite
            .bounds
            .intersect(&polychrome_sprite.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::PolychromeSprite,
                index: self.polychrome_sprites.primitives.len(),
            },
        );
        self.polychrome_sprites.insert(
            PolychromeSprite {
                order,
                ..polychrome_sprite
            },
            false,
            hover,
            group_hover,
        );
        Some(order)
    }

    pub(crate) fn insert_surface(&mut self, surface: Surface, occludes_hover: bool) -> Option<u32> {
        let clipped_bounds = surface.bounds.intersect(&surface.content_mask.bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return None;
        }

        let order = self.bounds_tree.insert(
            clipped_bounds,
            PrimitiveIndex {
                kind: PrimitiveKind::Surface,
                index: self.surfaces.len(),
            },
        );
        self.surfaces
            .insert(Surface { order, ..surface }, occludes_hover, None, None);
        Some(order)
    }

    pub fn reuse_views(&mut self, views: &FxHashSet<EntityId>, prev_scene: &mut Self) {
        // for shadow in prev_scene.shadows.drain(..) {
        //     if views.contains(&shadow.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&shadow.layer_id];
        //         self.insert(order, shadow);
        //     }
        // }

        // for quad in prev_scene.quads.drain(..) {
        //     if views.contains(&quad.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&quad.layer_id];
        //         self.insert(order, quad);
        //     }
        // }

        // for path in prev_scene.paths.drain(..) {
        //     if views.contains(&path.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&path.layer_id];
        //         self.insert(order, path);
        //     }
        // }

        // for underline in prev_scene.underlines.drain(..) {
        //     if views.contains(&underline.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&underline.layer_id];
        //         self.insert(order, underline);
        //     }
        // }

        // for sprite in prev_scene.monochrome_sprites.drain(..) {
        //     if views.contains(&sprite.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&sprite.layer_id];
        //         self.insert(order, sprite);
        //     }
        // }

        // for sprite in prev_scene.polychrome_sprites.drain(..) {
        //     if views.contains(&sprite.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&sprite.layer_id];
        //         self.insert(order, sprite);
        //     }
        // }

        // for surface in prev_scene.surfaces.drain(..) {
        //     if views.contains(&surface.view_id.into()) {
        //         let order = &prev_scene.orders_by_layer[&surface.layer_id];
        //         self.insert(order, surface);
        //     }
        // }
    }

    pub fn finish(&mut self, mouse_position: Point<ScaledPixels>) {
        self.hovered_bounds.clear();
        self.bounds_tree
            .find_containing(&mouse_position, &mut self.hovered_bounds);
        self.hovered_bounds
            .sort_unstable_by_key(|hovered| Reverse(hovered.order));

        for PrimitiveIndex { kind, index } in
            self.hovered_bounds.iter().map(|bounds| bounds.data.clone())
        {
            let occludes_hover = match kind {
                PrimitiveKind::Shadow => {
                    self.shadows.hover(index);
                    self.shadows.occludes_hover(index)
                }
                PrimitiveKind::Quad => {
                    self.quads.hover(index);
                    self.quads.occludes_hover(index)
                }
                PrimitiveKind::Path => {
                    self.paths.hover(index);
                    self.paths.occludes_hover(index)
                }
                PrimitiveKind::Underline => {
                    self.underlines.hover(index);
                    self.underlines.occludes_hover(index)
                }
                PrimitiveKind::MonochromeSprite => {
                    self.monochrome_sprites.hover(index);
                    self.monochrome_sprites.occludes_hover(index)
                }
                PrimitiveKind::PolychromeSprite => {
                    self.polychrome_sprites.hover(index);
                    self.polychrome_sprites.occludes_hover(index)
                }
                PrimitiveKind::Surface => self.surfaces.occludes_hover(index),
            };

            if occludes_hover {
                break;
            }
        }

        // TODO: Handle group-hover

        self.shadows
            .primitives
            .sort_unstable_by_key(|shadow| shadow.order);
        self.quads
            .primitives
            .sort_unstable_by_key(|quad| quad.order);
        self.paths
            .primitives
            .sort_unstable_by_key(|path| path.order);
        self.underlines
            .primitives
            .sort_unstable_by_key(|underline| underline.order);
        self.monochrome_sprites
            .primitives
            .sort_unstable_by_key(|sprite| sprite.order);
        self.polychrome_sprites
            .primitives
            .sort_unstable_by_key(|sprite| sprite.order);
        self.surfaces
            .primitives
            .sort_unstable_by_key(|surface| surface.order);
    }
}

pub(crate) struct PrimitiveSet<P> {
    pub(crate) primitives: Vec<P>,
    primitive_metadata: FxHashMap<usize, PrimitiveMetadata<P>>,
}

impl<P: Debug> PrimitiveSet<P> {
    /// Returns the number of primitives in the set.
    pub(crate) fn len(&self) -> usize {
        self.primitives.len()
    }

    /// Inserts a primitive into the set and associates hover and group hover information with it.
    ///
    /// # Arguments
    ///
    /// * `primitive` - The primitive to insert.
    /// * `occludes_hover` - A boolean indicating if the primitive occludes hover state.
    /// * `hover` - An optional hover state for the primitive.
    /// * `group_hover` - An optional group hover state for the primitive, with an associated shared string.
    fn insert(
        &mut self,
        primitive: P,
        occludes_hover: bool,
        hover: Option<P>,
        group_hover: Option<(SharedString, Option<P>)>,
    ) {
        let index = self.primitives.len();
        self.primitives.push(primitive);
        if occludes_hover || hover.is_some() || group_hover.is_some() {
            self.primitive_metadata.insert(
                index,
                PrimitiveMetadata {
                    occludes_hover,
                    hover,
                    group_hover,
                },
            );
        }
    }

    /// Overrides the primitive at the given index with its hovered variant, if a variant exists.
    fn hover(&mut self, index: usize) {
        if let Some(hovered_primitive) = self
            .primitive_metadata
            .get_mut(&index)
            .and_then(|data| data.hover.take())
        {
            self.primitives[index] = hovered_primitive;
        }
    }

    fn occludes_hover(&self, index: usize) -> bool {
        self.primitive_metadata
            .get(&index)
            .map_or(false, |data| data.occludes_hover)
    }

    /// Clears all primitives and associated hover information from the set.
    fn clear(&mut self) {
        self.primitives.clear();
        self.primitive_metadata.clear();
    }
}

impl<P> Default for PrimitiveSet<P> {
    fn default() -> Self {
        Self {
            primitives: Vec::new(),
            primitive_metadata: FxHashMap::default(),
        }
    }
}

struct PrimitiveMetadata<P> {
    occludes_hover: bool,
    hover: Option<P>,
    group_hover: Option<(SharedString, Option<P>)>,
}

struct BatchIterator<'a> {
    shadows: &'a [Shadow],
    shadows_start: usize,
    shadows_iter: Peekable<slice::Iter<'a, Shadow>>,
    quads: &'a [Quad],
    quads_start: usize,
    quads_iter: Peekable<slice::Iter<'a, Quad>>,
    paths: &'a [Path<ScaledPixels>],
    paths_start: usize,
    paths_iter: Peekable<slice::Iter<'a, Path<ScaledPixels>>>,
    underlines: &'a [Underline],
    underlines_start: usize,
    underlines_iter: Peekable<slice::Iter<'a, Underline>>,
    monochrome_sprites: &'a [MonochromeSprite],
    monochrome_sprites_start: usize,
    monochrome_sprites_iter: Peekable<slice::Iter<'a, MonochromeSprite>>,
    polychrome_sprites: &'a [PolychromeSprite],
    polychrome_sprites_start: usize,
    polychrome_sprites_iter: Peekable<slice::Iter<'a, PolychromeSprite>>,
    surfaces: &'a [Surface],
    surfaces_start: usize,
    surfaces_iter: Peekable<slice::Iter<'a, Surface>>,
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
                Some(PrimitiveBatch::Shadows(
                    &self.shadows[shadows_start..shadows_end],
                ))
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
                Some(PrimitiveBatch::Quads(&self.quads[quads_start..quads_end]))
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
                Some(PrimitiveBatch::Paths(&self.paths[paths_start..paths_end]))
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
                Some(PrimitiveBatch::Underlines(
                    &self.underlines[underlines_start..underlines_end],
                ))
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
                    sprites: &self.monochrome_sprites[sprites_start..sprites_end],
                })
            }
            PrimitiveKind::PolychromeSprite => {
                let texture_id = self.polychrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.polychrome_sprites_start;
                let mut sprites_end = self.polychrome_sprites_start + 1;
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
                    sprites: &self.polychrome_sprites[sprites_start..sprites_end],
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
                Some(PrimitiveBatch::Surfaces(
                    &self.surfaces[surfaces_start..surfaces_end],
                ))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub(crate) enum PrimitiveKind {
    Shadow,
    #[default]
    Quad,
    Path,
    Underline,
    MonochromeSprite,
    PolychromeSprite,
    Surface,
}

#[derive(Clone, Debug)]
struct PrimitiveIndex {
    kind: PrimitiveKind,
    index: usize,
}

#[derive(Debug)]
pub(crate) enum PrimitiveBatch<'a> {
    Shadows(&'a [Shadow]),
    Quads(&'a [Quad]),
    Paths(&'a [Path<ScaledPixels>]),
    Underlines(&'a [Underline]),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [MonochromeSprite],
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [PolychromeSprite],
    },
    Surfaces(&'a [Surface]),
}
