//! macOS-specific extensions to [`Window`].
//!
//! Window APIs that only make sense on one platform live beside it rather than
//! on the cross-platform type, so that the base surface names no operating
//! system types. Import the extension trait to reach them.

use crate::{Bounds, Pixels, Window};
use core_video::pixel_buffer::CVPixelBuffer;

/// macOS-specific drawing on a [`Window`].
pub trait MacWindowExt {
    /// Paint a CoreVideo surface into the scene for the next frame at the
    /// current z-index.
    ///
    /// This method should only be called as part of the paint phase of element
    /// drawing.
    fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVPixelBuffer);
}

impl MacWindowExt for Window<'_> {
    fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVPixelBuffer) {
        use crate::PaintSurface;

        self.core.invalidator.debug_assert_paint();

        let bounds = self.snap_bounds(bounds);
        let content_mask = self.snapped_content_mask();
        self.frame_state
            .next_frame
            .scene
            .insert_primitive(PaintSurface {
                order: 0,
                bounds,
                content_mask,
                image_buffer,
            });
    }
}
