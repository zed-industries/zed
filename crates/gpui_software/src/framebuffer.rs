use gpui::{DevicePixels, Size};

pub const BAND_HEIGHT: usize = 32;

pub struct Framebuffer {
    pixels: Vec<u32>,
    size: Size<DevicePixels>,
}

impl Framebuffer {
    pub fn new(size: Size<DevicePixels>) -> Self {
        let mut framebuffer = Self {
            pixels: Vec::new(),
            size,
        };
        framebuffer.allocate();
        framebuffer
    }

    pub fn resize(&mut self, size: Size<DevicePixels>) {
        self.size = size;
        self.allocate();
    }

    pub fn size(&self) -> Size<DevicePixels> {
        self.size
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn as_ptr(&self) -> *const u32 {
        self.pixels.as_ptr()
    }

    pub(crate) fn pixels_mut(&mut self) -> &mut [u32] {
        &mut self.pixels
    }

    fn allocate(&mut self) {
        let width = self.size.width.0.max(0) as usize;
        let height = self.size.height.0.max(0) as usize;
        self.pixels
            .resize(width.saturating_mul(height), 0xff00_0000);
        self.pixels.fill(0xff00_0000);
    }
}
