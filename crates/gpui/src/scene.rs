// todo!("windows"): remove
#![cfg_attr(windows, allow(dead_code))]

use crate::{
    bounds_tree::BoundsTree, point, AtlasTextureId, AtlasTile, Bounds, ContentMask, Corners, Edges,
    Hsla, Pixels, Point, ScaledPixels,
};
use std::{fmt::Debug, iter, slice};

#[allow(non_camel_case_types, unused)]
pub(crate) type PathVertex_ScaledPixels = PathVertex<ScaledPixels>;

pub(crate) type DrawOrder = u32;

#[derive(Default)]
pub(crate) struct Scene {
    pub(crate) primitives: Vec<Primitive>,
    primitive_bounds: BoundsTree<ScaledPixels, ()>,
    paths: Vec<Path<ScaledPixels>>,
}

impl Scene {
    pub fn clear(&mut self) {
        self.primitives.clear();
        self.primitive_bounds.clear();
        self.paths.clear();
    }

    pub fn paths(&self) -> &[Path<ScaledPixels>] {
        &self.paths
    }

    pub fn len(&self) -> usize {
        self.primitives.len()
    }

    pub(crate) fn push(&mut self, primitive: impl Into<Primitive>) {
        let mut primitive = primitive.into();
        let clipped_bounds = primitive
            .bounds()
            .intersect(&primitive.content_mask().bounds);
        if clipped_bounds.size.width <= ScaledPixels(0.)
            || clipped_bounds.size.height <= ScaledPixels(0.)
        {
            return;
        }

        let order = self.primitive_bounds.insert(clipped_bounds, ());
        match &mut primitive {
            Primitive::Shadow(shadow) => shadow.order = order,
            Primitive::Quad(quad) => quad.order = order,
            Primitive::Path(path) => {
                path.order = order;
                path.id = PathId(self.paths.len());
                self.paths.push(path.clone());
            }
            Primitive::Underline(underline) => underline.order = order,
            Primitive::MonochromeSprite(sprite) => sprite.order = order,
            Primitive::PolychromeSprite(sprite) => sprite.order = order,
            Primitive::Surface(surface) => surface.order = order,
        }
        self.primitives.push(primitive);
    }

    pub fn finish(&mut self) {
        self.primitives.sort_unstable();
    }

    pub(crate) fn batches(&self) -> PrimitiveBatches {
        PrimitiveBatches {
            primitives: self.primitives.iter().peekable(),
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum Primitive {
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

pub(crate) struct PrimitiveBatches<'a> {
    primitives: iter::Peekable<slice::Iter<'a, Primitive>>,
    shadows: Vec<Shadow>,
    quads: Vec<Quad>,
    paths: Vec<Path<ScaledPixels>>,
    underlines: Vec<Underline>,
    monochrome_sprites: Vec<MonochromeSprite>,
    polychrome_sprites: Vec<PolychromeSprite>,
    surfaces: Vec<Surface>,
}

impl<'a> PrimitiveBatches<'a> {
    pub fn next(&mut self) -> Option<PrimitiveBatch> {
        let primitive = self.primitives.next()?;
        match primitive {
            Primitive::Shadow(shadow) => {
                self.shadows.clear();
                self.shadows.push(shadow.clone());
                while let Some(Primitive::Shadow(next_shadow)) = self.primitives.peek() {
                    self.shadows.push(next_shadow.clone());
                    self.primitives.next();
                }
                Some(PrimitiveBatch::Shadows(&self.shadows))
            }
            Primitive::Quad(quad) => {
                self.quads.clear();
                self.quads.push(quad.clone());
                while let Some(Primitive::Quad(next_quad)) = self.primitives.peek() {
                    self.quads.push(next_quad.clone());
                    self.primitives.next();
                }
                Some(PrimitiveBatch::Quads(&self.quads))
            }
            Primitive::Path(path) => {
                self.paths.clear();
                self.paths.push(path.clone());
                while let Some(Primitive::Path(next_path)) = self.primitives.peek() {
                    self.paths.push(next_path.clone());
                    self.primitives.next();
                }
                Some(PrimitiveBatch::Paths(&self.paths))
            }
            Primitive::Underline(underline) => {
                self.underlines.clear();
                self.underlines.push(underline.clone());
                while let Some(Primitive::Underline(next_underline)) = self.primitives.peek() {
                    self.underlines.push(next_underline.clone());
                    self.primitives.next();
                }
                Some(PrimitiveBatch::Underlines(&self.underlines))
            }
            Primitive::MonochromeSprite(sprite) => {
                let texture_id = sprite.tile.texture_id;
                self.monochrome_sprites.clear();
                self.monochrome_sprites.push(sprite.clone());
                while let Some(Primitive::MonochromeSprite(next_sprite)) = self.primitives.peek() {
                    if next_sprite.tile.texture_id == texture_id {
                        self.monochrome_sprites.push(next_sprite.clone());
                        self.primitives.next();
                    } else {
                        break;
                    }
                }
                Some(PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites: &self.monochrome_sprites,
                })
            }
            Primitive::PolychromeSprite(sprite) => {
                let texture_id = sprite.tile.texture_id;
                self.polychrome_sprites.clear();
                self.polychrome_sprites.push(sprite.clone());
                while let Some(Primitive::PolychromeSprite(next_sprite)) = self.primitives.peek() {
                    if next_sprite.tile.texture_id == texture_id {
                        self.polychrome_sprites.push(next_sprite.clone());
                        self.primitives.next();
                    } else {
                        break;
                    }
                }
                Some(PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites: &self.polychrome_sprites,
                })
            }
            Primitive::Surface(surface) => {
                self.surfaces.clear();
                self.surfaces.push(surface.clone());
                while let Some(Primitive::Surface(next_surface)) = self.primitives.peek() {
                    self.surfaces.push(next_surface.clone());
                    self.primitives.next();
                }
                Some(PrimitiveBatch::Surfaces(&self.surfaces))
            }
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
pub(crate) struct Quad {
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

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct Underline {
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

impl From<Underline> for Primitive {
    fn from(underline: Underline) -> Self {
        Primitive::Underline(underline)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct Shadow {
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

impl From<Shadow> for Primitive {
    fn from(shadow: Shadow) -> Self {
        Primitive::Shadow(shadow)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct MonochromeSprite {
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

impl From<MonochromeSprite> for Primitive {
    fn from(sprite: MonochromeSprite) -> Self {
        Primitive::MonochromeSprite(sprite)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct PolychromeSprite {
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

impl From<PolychromeSprite> for Primitive {
    fn from(sprite: PolychromeSprite) -> Self {
        Primitive::PolychromeSprite(sprite)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Surface {
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

impl From<Surface> for Primitive {
    fn from(surface: Surface) -> Self {
        Primitive::Surface(surface)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathId(pub(crate) usize);

/// A line made up of a series of vertices and control points.
#[derive(Clone, Debug)]
pub struct Path<P: Clone + Default + Debug> {
    pub(crate) id: PathId,
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

impl From<Path<ScaledPixels>> for Primitive {
    fn from(path: Path<ScaledPixels>) -> Self {
        Primitive::Path(path)
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
