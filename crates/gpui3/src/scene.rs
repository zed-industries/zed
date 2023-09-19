use super::{Bounds, Hsla, Pixels, Point};
use bytemuck::{Pod, Zeroable};
use plane_split::BspSplitter;

pub struct Scene {
    opaque_primitives: PrimitiveBatch,
    transparent_primitives: slotmap::SlotMap<slotmap::DefaultKey, Primitive>,
    splitter: BspSplitter<slotmap::DefaultKey>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            opaque_primitives: PrimitiveBatch::default(),
            transparent_primitives: slotmap::SlotMap::new(),
            splitter: BspSplitter::new(),
        }
    }

    pub fn insert(&mut self, primitive: impl Into<Primitive>, is_transparent: bool) {
        if is_transparent {
            self.transparent_primitives.insert(primitive.into());
        } else {
            match primitive.into() {
                Primitive::Quad(quad) => self.opaque_primitives.quads.push(quad),
                Primitive::Glyph(glyph) => self.opaque_primitives.glyphs.push(glyph),
                Primitive::Underline(underline) => {
                    self.opaque_primitives.underlines.push(underline)
                }
            }
        }
    }

    pub fn opaque_primitives(&self) -> &PrimitiveBatch {
        &self.opaque_primitives
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Quad(Quad),
    Glyph(Glyph),
    Underline(Underline),
}

impl Primitive {
    pub fn is_transparent(&self) -> bool {
        match self {
            Primitive::Quad(quad) => {
                quad.background.is_transparent() && quad.border_color.is_transparent()
            }
            Primitive::Glyph(glyph) => glyph.color.is_transparent(),
            Primitive::Underline(underline) => underline.color.is_transparent(),
        }
    }
}

#[derive(Default)]
pub struct PrimitiveBatch {
    pub quads: Vec<Quad>,
    pub glyphs: Vec<Glyph>,
    pub underlines: Vec<Underline>,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Quad {
    pub order: f32,
    pub bounds: Bounds<Pixels>,
    pub background: Hsla,
    pub border_color: Hsla,
    pub corner_radius: Pixels,
    pub border_left: Pixels,
    pub border_right: Pixels,
    pub border_top: Pixels,
    pub border_bottom: Pixels,
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
}

unsafe impl Zeroable for Quad {}

unsafe impl Pod for Quad {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlyphId(u32);

#[derive(Clone, Debug)]
pub struct Glyph {
    pub id: GlyphId,
    pub position: Point<Pixels>,
    pub color: Hsla,
    pub index: usize,
    pub is_emoji: bool,
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}

impl From<Glyph> for Primitive {
    fn from(glyph: Glyph) -> Self {
        Primitive::Glyph(glyph)
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Underline {
    pub origin: Point<Pixels>,
    pub width: Pixels,
    pub thickness: Pixels,
    pub color: Hsla,
    pub squiggly: bool,
}

unsafe impl Zeroable for Underline {}

unsafe impl Pod for Underline {}

impl From<Underline> for Primitive {
    fn from(underline: Underline) -> Self {
        Primitive::Underline(underline)
    }
}
