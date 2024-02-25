use crate::{
    point, AtlasTextureId, AtlasTile, Bounds, BoundsTree, ContentMask, Corners, Edges, EntityId,
    Hsla, Pixels, Point, ScaledPixels, SharedString,
};
use collections::{FxHashMap, FxHashSet, HashMap};
use std::{fmt::Debug, iter::Peekable, slice};

#[allow(non_camel_case_types, unused)]
pub(crate) type PathVertex_ScaledPixels = PathVertex<ScaledPixels>;

pub(crate) type LayerId = u32;
pub(crate) type DrawOrder = u32;

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub(crate) struct ViewId {
    low_bits: u32,
    high_bits: u32,
}

impl From<EntityId> for ViewId {
    fn from(value: EntityId) -> Self {
        let value = value.as_u64();
        Self {
            low_bits: value as u32,
            high_bits: (value >> 32) as u32,
        }
    }
}

impl From<ViewId> for EntityId {
    fn from(value: ViewId) -> Self {
        let value = (value.low_bits as u64) | ((value.high_bits as u64) << 32);
        value.into()
    }
}

#[derive(Default)]
pub(crate) struct Scene {
    pub(crate) shadows: PrimitiveSet<Shadow>,
    pub(crate) quads: PrimitiveSet<Quad>,
    pub(crate) paths: PrimitiveSet<Path<ScaledPixels>>,
    pub(crate) underlines: PrimitiveSet<Underline>,
    pub(crate) monochrome_sprites: PrimitiveSet<MonochromeSprite>,
    pub(crate) polychrome_sprites: PrimitiveSet<PolychromeSprite>,
    pub(crate) surfaces: Vec<Surface>,
    bounds_tree: BoundsTree<ScaledPixels, PrimitiveIndex>,
    hovered_bounds: Vec<(Bounds<ScaledPixels>, PrimitiveIndex)>,
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
            surfaces: &self.surfaces,
            surfaces_start: 0,
            surfaces_iter: self.surfaces.iter().peekable(),
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
            .insert(Shadow { order, ..shadow }, hover, group_hover);
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
        self.quads
            .insert(Quad { order, ..quad }, hover, group_hover);
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
            .insert(Path { order, ..path }, hover, group_hover);
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
            .insert(Underline { order, ..underline }, hover, group_hover);
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
            hover,
            group_hover,
        );
        Some(order)
    }

    pub(crate) fn insert_surface(&mut self, surface: Surface) -> Option<u32> {
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
        self.surfaces.push(Surface { order, ..surface });
        Some(order)
    }

    pub fn reuse_views(&mut self, views: &FxHashSet<EntityId>, prev_scene: &mut Self) {
        todo!()
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

        todo!("replace hovered primitives");
        todo!("replace group-hovered primitives. i think we'll need to index group membership to do so.");

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
        self.surfaces.sort_unstable_by_key(|surface| surface.order);
    }
}

pub(crate) struct PrimitiveSet<P> {
    pub(crate) primitives: Vec<P>,
    hovers: FxHashMap<usize, PrimitiveHover<P>>,
}

impl<P> PrimitiveSet<P> {
    /// Returns the number of primitives in the set.
    pub(crate) fn len(&self) -> usize {
        self.primitives.len()
    }

    /// Inserts a primitive into the set and associates hover and group hover information with it.
    ///
    /// # Arguments
    ///
    /// * `primitive` - The primitive to insert.
    /// * `hover` - An optional hover state for the primitive.
    /// * `group_hover` - An optional group hover state for the primitive, with an associated shared string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zed::SharedString;
    /// let mut primitive_set = PrimitiveSet::default();
    /// let primitive = Quad::default();
    /// let hover = Some(Quad::default());
    /// let group_hover = Some((SharedString::from("group"), Some(Quad::default())));
    ///
    /// primitive_set.insert(primitive, hover, group_hover);
    /// ```
    fn insert(
        &mut self,
        primitive: P,
        hover: Option<P>,
        group_hover: Option<(SharedString, Option<P>)>,
    ) {
        let index = self.primitives.len();
        self.primitives.push(primitive);
        if hover.is_some() || group_hover.is_some() {
            self.hovers
                .insert(index, PrimitiveHover { hover, group_hover });
        }
    }

    /// Clears all primitives and associated hover information from the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # let mut primitive_set = PrimitiveSet::default();
    /// primitive_set.clear();
    /// assert!(primitive_set.len() == 0);
    /// ```
    fn clear(&mut self) {
        self.primitives.clear();
        self.hovers.clear();
    }
}

impl<P> Default for PrimitiveSet<P> {
    fn default() -> Self {
        Self {
            primitives: Vec::new(),
            hovers: HashMap::default(),
        }
    }
}

struct PrimitiveHover<P> {
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

#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct Quad {
    pub view_id: ViewId,
    pub layer_id: LayerId,
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
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

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct Underline {
    pub view_id: ViewId,
    pub layer_id: LayerId,
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub thickness: ScaledPixels,
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

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct Shadow {
    pub view_id: ViewId,
    pub layer_id: LayerId,
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub blur_radius: ScaledPixels,
    pub pad: u32, // align to 8 bytes
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct MonochromeSprite {
    pub view_id: ViewId,
    pub layer_id: LayerId,
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct PolychromeSprite {
    pub view_id: ViewId,
    pub layer_id: LayerId,
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub tile: AtlasTile,
    pub grayscale: bool,
    pub pad: u32, // align to 8 bytes
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Surface {
    pub view_id: ViewId,
    pub layer_id: LayerId,
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    #[cfg(target_os = "macos")]
    pub image_buffer: media::core_video::CVImageBuffer,
}

impl Ord for Surface {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Surface {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathId(pub(crate) usize);

/// A line made up of a series of vertices and control points.
#[derive(Debug)]
pub struct Path<P: Clone + Default + Debug> {
    pub(crate) id: PathId,
    pub(crate) view_id: ViewId,
    layer_id: LayerId,
    order: DrawOrder,
    pub(crate) bounds: Bounds<P>,
    pub(crate) content_mask: ContentMask<P>,
    pub(crate) vertices: Vec<PathVertex<P>>,
    pub(crate) color: Hsla,
    start: Point<P>,
    current: Point<P>,
    contour_count: usize,
}

impl Path<Pixels> {
    /// Create a new path with the given starting point.
    pub fn new(start: Point<Pixels>) -> Self {
        Self {
            id: PathId(0),
            view_id: ViewId::default(),
            layer_id: LayerId::default(),
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
            view_id: self.view_id,
            layer_id: self.layer_id,
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

    fn push_triangle(
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

impl Eq for Path<ScaledPixels> {}

impl PartialEq for Path<ScaledPixels> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Ord for Path<ScaledPixels> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Path<ScaledPixels> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct PathVertex<P: Clone + Default + Debug> {
    pub(crate) xy_position: Point<P>,
    pub(crate) st_position: Point<f32>,
    pub(crate) content_mask: ContentMask<P>,
}

impl PathVertex<Pixels> {
    pub fn scale(&self, factor: f32) -> PathVertex<ScaledPixels> {
        PathVertex {
            xy_position: self.xy_position.scale(factor),
            st_position: self.st_position,
            content_mask: self.content_mask.scale(factor),
        }
    }
}
