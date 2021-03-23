use crate::geometry::vector::Vector2I;
use etagere::BucketedAtlasAllocator;

struct SpriteCache {
    atlasses: Vec<etagere::BucketedAtlasAllocator>,
}

impl SpriteCache {
    fn new(size: Vector2I) -> Self {
        let size = etagere::Size::new(size.x(), size.y());
        Self {
            atlasses: vec![BucketedAtlasAllocator::new(size)],
        }
    }

    fn render_glyph(&mut self) {
        self.atlasses.last().unwrap()
    }
}
