use crate::geometry::vector::{vec2i, Vector2I};
use image::{Bgra, ImageBuffer};
use std::sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    Arc,
};

pub struct ImageData {
    pub id: usize,
    data: ImageBuffer<Bgra<u8>, Vec<u8>>,
}

impl ImageData {
    pub fn new(data: ImageBuffer<Bgra<u8>, Vec<u8>>) -> Arc<Self> {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        Arc::new(Self {
            id: NEXT_ID.fetch_add(1, SeqCst),
            data,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn size(&self) -> Vector2I {
        let (width, height) = self.data.dimensions();
        vec2i(width as i32, height as i32)
    }
}
