use super::Ordered;

/// A platform neutral representation of all geometry to be drawn for the current frame.
pub struct Scene {
    quads: Vec<Ordered<Quad>>,
    glyphs: Vec<Ordered<Glyph>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            quads: Vec::new(),
            glyphs: Vec::new(),
        }
    }

    pub fn add(&mut self, order: u32, primitive: impl Into<Primitive>) {
        match primitive.into() {
            Primitive::Quad(primitive) => self.quads.push(Ordered { order, primitive }),
            Primitive::Glyph(primitive) => self.glyphs.push(Ordered { order, primitive }),
        }
    }

    pub fn draw(&mut self) {
        self.quads.sort_unstable();
        self.glyphs.sort_unstable();
    }
}

pub enum Primitive {
    Quad(Quad),
    Glyph(Glyph),
    // Icon(Icon),
    // Image(Image),
    // Shadow(Shadow),
    // Curve(Curve),
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Quad {}
pub struct Glyph {}
// pub struct Icon {}
// pub struct Image {}
// pub struct Shadow {}
// pub struct Curve {}
