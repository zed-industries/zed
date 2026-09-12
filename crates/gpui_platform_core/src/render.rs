//! Rendering asset identifiers shared by `gpui` and its platform backends.

use gpui_shared_string::SharedString;
use gpui_types::{DevicePixels, Size};

/// A unique identifier for the image cache
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(pub usize);

#[derive(PartialEq, Eq, Hash, Clone)]
#[expect(missing_docs)]
pub struct RenderImageParams {
    pub image_id: ImageId,
    pub frame_index: usize,
}

#[derive(Clone, PartialEq, Hash, Eq)]
#[expect(missing_docs)]
pub struct RenderSvgParams {
    pub path: SharedString,
    pub size: Size<DevicePixels>,
}
