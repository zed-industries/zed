use crate::{
    point, AtlasTextureId, AtlasTile, Bounds, ContentMask, Corners, Edges, Hsla, Pixels, Point,
    ScaledPixels, StackingOrder,
};
use collections::BTreeMap;
use etagere::euclid::{Point3D, Vector3D};
use plane_split::{BspSplitter, Polygon as BspPolygon};
use std::{fmt::Debug, iter::Peekable, mem, slice};

// Exported to metal
pub(crate) type PointF = Point<f32>;
#[allow(non_camel_case_types, unused)]
pub(crate) type PathVertex_ScaledPixels = PathVertex<ScaledPixels>;

pub type LayerId = u32;

pub type DrawOrder = u32;

pub(crate) struct SceneBuilder {
    last_order: Option<(StackingOrder, LayerId)>,
    layers_by_order: BTreeMap<StackingOrder, LayerId>,
    splitter: BspSplitter<(PrimitiveKind, usize)>,
    shadows: Vec<Shadow>,
    quads: Vec<Quad>,
    paths: Vec<Path<ScaledPixels>>,
    underlines: Vec<Underline>,
    monochrome_sprites: Vec<MonochromeSprite>,
    polychrome_sprites: Vec<PolychromeSprite>,
    surfaces: Vec<Surface>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        SceneBuilder {
            last_order: None,
            layers_by_order: BTreeMap::new(),
            splitter: BspSplitter::new(),
            shadows: Vec::new(),
            quads: Vec::new(),
            paths: Vec::new(),
            underlines: Vec::new(),
            monochrome_sprites: Vec::new(),
            polychrome_sprites: Vec::new(),
            surfaces: Vec::new(),
        }
    }
}

impl SceneBuilder {
    pub fn build(&mut self) -> Scene {
        // Map each layer id to a float between 0. and 1., with 1. closer to the viewer.
        let mut layer_z_values = vec![0.; self.layers_by_order.len()];
        for (ix, layer_id) in self.layers_by_order.values().enumerate() {
            layer_z_values[*layer_id as usize] = ix as f32 / self.layers_by_order.len() as f32;
        }
        self.layers_by_order.clear();
        self.last_order = None;

        // Add all primitives to the BSP splitter to determine draw order
        self.splitter.reset();

        for (ix, shadow) in self.shadows.iter().enumerate() {
            let z = layer_z_values[shadow.order as LayerId as usize];
            self.splitter
                .add(shadow.bounds.to_bsp_polygon(z, (PrimitiveKind::Shadow, ix)));
        }

        for (ix, quad) in self.quads.iter().enumerate() {
            let z = layer_z_values[quad.order as LayerId as usize];
            self.splitter
                .add(quad.bounds.to_bsp_polygon(z, (PrimitiveKind::Quad, ix)));
        }

        for (ix, path) in self.paths.iter().enumerate() {
            let z = layer_z_values[path.order as LayerId as usize];
            self.splitter
                .add(path.bounds.to_bsp_polygon(z, (PrimitiveKind::Path, ix)));
        }

        for (ix, underline) in self.underlines.iter().enumerate() {
            let z = layer_z_values[underline.order as LayerId as usize];
            self.splitter.add(
                underline
                    .bounds
                    .to_bsp_polygon(z, (PrimitiveKind::Underline, ix)),
            );
        }

        for (ix, monochrome_sprite) in self.monochrome_sprites.iter().enumerate() {
            let z = layer_z_values[monochrome_sprite.order as LayerId as usize];
            self.splitter.add(
                monochrome_sprite
                    .bounds
                    .to_bsp_polygon(z, (PrimitiveKind::MonochromeSprite, ix)),
            );
        }

        for (ix, polychrome_sprite) in self.polychrome_sprites.iter().enumerate() {
            let z = layer_z_values[polychrome_sprite.order as LayerId as usize];
            self.splitter.add(
                polychrome_sprite
                    .bounds
                    .to_bsp_polygon(z, (PrimitiveKind::PolychromeSprite, ix)),
            );
        }

        // Sort all polygons, then reassign the order field of each primitive to `draw_order`
        // We need primitives to be repr(C), hence the weird reuse of the order field for two different types.
        for (draw_order, polygon) in self
            .splitter
            .sort(Vector3D::new(0., 0., 1.))
            .iter()
            .enumerate()
        {
            match polygon.anchor {
                (PrimitiveKind::Shadow, ix) => self.shadows[ix].order = draw_order as DrawOrder,
                (PrimitiveKind::Quad, ix) => self.quads[ix].order = draw_order as DrawOrder,
                (PrimitiveKind::Path, ix) => self.paths[ix].order = draw_order as DrawOrder,
                (PrimitiveKind::Underline, ix) => {
                    self.underlines[ix].order = draw_order as DrawOrder
                }
                (PrimitiveKind::MonochromeSprite, ix) => {
                    self.monochrome_sprites[ix].order = draw_order as DrawOrder
                }
                (PrimitiveKind::PolychromeSprite, ix) => {
                    self.polychrome_sprites[ix].order = draw_order as DrawOrder
                }
                (PrimitiveKind::Surface, ix) => self.surfaces[ix].order = draw_order as DrawOrder,
            }
        }

        self.shadows.sort_unstable();
        self.quads.sort_unstable();
        self.paths.sort_unstable();
        self.underlines.sort_unstable();
        self.monochrome_sprites.sort_unstable();
        self.polychrome_sprites.sort_unstable();
        self.surfaces.sort_unstable();

        Scene {
            shadows: mem::take(&mut self.shadows),
            quads: mem::take(&mut self.quads),
            paths: mem::take(&mut self.paths),
            underlines: mem::take(&mut self.underlines),
            monochrome_sprites: mem::take(&mut self.monochrome_sprites),
            polychrome_sprites: mem::take(&mut self.polychrome_sprites),
            surfaces: mem::take(&mut self.surfaces),
        }
    }

    pub fn insert(&mut self, order: &StackingOrder, primitive: impl Into<Primitive>) {
        let primitive = primitive.into();
        let clipped_bounds = primitive
            .bounds()
            .intersect(&primitive.content_mask().bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return;
        }

        let layer_id = self.layer_id_for_order(order);
        match primitive {
            Primitive::Shadow(mut shadow) => {
                shadow.order = layer_id;
                self.shadows.push(shadow);
            }
            Primitive::Quad(mut quad) => {
                quad.order = layer_id;
                self.quads.push(quad);
            }
            Primitive::Path(mut path) => {
                path.order = layer_id;
                path.id = PathId(self.paths.len());
                self.paths.push(path);
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
            Primitive::Surface(mut surface) => {
                surface.order = layer_id;
                self.surfaces.push(surface);
            }
        }
    }

    fn layer_id_for_order(&mut self, order: &StackingOrder) -> u32 {
        if let Some((last_order, last_layer_id)) = self.last_order.as_ref() {
            if last_order == order {
                return *last_layer_id;
            }
        };

        let layer_id = if let Some(layer_id) = self.layers_by_order.get(order) {
            *layer_id
        } else {
            let next_id = self.layers_by_order.len() as LayerId;
            self.layers_by_order.insert(order.clone(), next_id);
            next_id
        };
        self.last_order = Some((order.clone(), layer_id));
        layer_id
    }
}

pub struct Scene {
    pub shadows: Vec<Shadow>,
    pub quads: Vec<Quad>,
    pub paths: Vec<Path<ScaledPixels>>,
    pub underlines: Vec<Underline>,
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
    pub surfaces: Vec<Surface>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn paths(&self) -> &[Path<ScaledPixels>] {
        &self.paths
    }

    pub(crate) fn batches(&self) -> impl Iterator<Item = PrimitiveBatch> {
        BatchIterator {
            shadows: &self.shadows,
            shadows_start: 0,
            shadows_iter: self.shadows.iter().peekable(),
            quads: &self.quads,
            quads_start: 0,
            quads_iter: self.quads.iter().peekable(),
            paths: &self.paths,
            paths_start: 0,
            paths_iter: self.paths.iter().peekable(),
            underlines: &self.underlines,
            underlines_start: 0,
            underlines_iter: self.underlines.iter().peekable(),
            monochrome_sprites: &self.monochrome_sprites,
            monochrome_sprites_start: 0,
            monochrome_sprites_iter: self.monochrome_sprites.iter().peekable(),
            polychrome_sprites: &self.polychrome_sprites,
            polychrome_sprites_start: 0,
            polychrome_sprites_iter: self.polychrome_sprites.iter().peekable(),
            surfaces: &self.surfaces,
            surfaces_start: 0,
            surfaces_iter: self.surfaces.iter().peekable(),
        }
    }
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
            PrimitiveKind::Path => {
                let paths_start = self.paths_start;
                let mut paths_end = paths_start;
                while self
                    .paths_iter
                    .next_if(|path| path.order <= max_order)
                    .is_some()
                {
                    paths_end += 1;
                }
                self.paths_start = paths_end;
                Some(PrimitiveBatch::Paths(&self.paths[paths_start..paths_end]))
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
            PrimitiveKind::Surface => {
                let surfaces_start = self.surfaces_start;
                let mut surfaces_end = surfaces_start;
                while self
                    .surfaces_iter
                    .next_if(|surface| surface.order <= max_order)
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
pub enum PrimitiveKind {
    Shadow,
    #[default]
    Quad,
    Path,
    Underline,
    MonochromeSprite,
    PolychromeSprite,
    Surface,
}

pub enum Primitive {
    Shadow(Shadow),
    Quad(Quad),
    Path(Path<ScaledPixels>),
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    PolychromeSprite(PolychromeSprite),
    Surface(Surface),
}

impl Primitive {
    pub fn bounds(&self) -> &Bounds<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.bounds,
            Primitive::Quad(quad) => &quad.bounds,
            Primitive::Path(path) => &path.bounds,
            Primitive::Underline(underline) => &underline.bounds,
            Primitive::MonochromeSprite(sprite) => &sprite.bounds,
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
            Primitive::PolychromeSprite(sprite) => &sprite.content_mask,
            Primitive::Surface(surface) => &surface.content_mask,
        }
    }
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
pub struct Quad {
    pub order: u32, // Initially a LayerId, then a DrawOrder.
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
    pub content_mask: ContentMask<ScaledPixels>,
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
    pub content_mask: ContentMask<ScaledPixels>,
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
    pub content_mask: ContentMask<ScaledPixels>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Surface {
    pub order: u32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
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

impl From<Surface> for Primitive {
    fn from(surface: Surface) -> Self {
        Primitive::Surface(surface)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathId(pub(crate) usize);

#[derive(Debug)]
pub struct Path<P: Clone + Default + Debug> {
    pub(crate) id: PathId,
    order: u32,
    pub(crate) bounds: Bounds<P>,
    pub(crate) content_mask: ContentMask<P>,
    pub(crate) vertices: Vec<PathVertex<P>>,
    pub(crate) color: Hsla,
    start: Point<P>,
    current: Point<P>,
    contour_count: usize,
}

impl Path<Pixels> {
    pub fn new(start: Point<Pixels>) -> Self {
        Self {
            id: PathId(0),
            order: 0,
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

impl From<Path<ScaledPixels>> for Primitive {
    fn from(path: Path<ScaledPixels>) -> Self {
        Primitive::Path(path)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct PathVertex<P: Clone + Default + Debug> {
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

// #[cfg(test)]
// mod tests {
//     use crate::{point, size};

//     use super::*;
//     use smallvec::smallvec;

//     #[test]
//     fn test_scene() {
//         let mut scene = SceneBuilder::new();
//         assert_eq!(scene.layers_by_order.len(), 0);

//         scene.insert(&smallvec![1].into(), quad());
//         scene.insert(&smallvec![2].into(), shadow());
//         scene.insert(&smallvec![3].into(), quad());

//         let mut batches_count = 0;
//         for _ in scene.build().batches() {
//             batches_count += 1;
//         }
//         assert_eq!(batches_count, 3);
//     }

//     fn quad() -> Quad {
//         Quad {
//             order: 0,
//             bounds: Bounds {
//                 origin: point(ScaledPixels(0.), ScaledPixels(0.)),
//                 size: size(ScaledPixels(100.), ScaledPixels(100.)),
//             },
//             content_mask: Default::default(),
//             background: Default::default(),
//             border_color: Default::default(),
//             corner_radii: Default::default(),
//             border_widths: Default::default(),
//         }
//     }

//     fn shadow() -> Shadow {
//         Shadow {
//             order: Default::default(),
//             bounds: Bounds {
//                 origin: point(ScaledPixels(0.), ScaledPixels(0.)),
//                 size: size(ScaledPixels(100.), ScaledPixels(100.)),
//             },
//             corner_radii: Default::default(),
//             content_mask: Default::default(),
//             color: Default::default(),
//             blur_radius: Default::default(),
//         }
//     }
// }
