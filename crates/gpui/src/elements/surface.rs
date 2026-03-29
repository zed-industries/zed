use crate::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    ObjectFit, Pixels, Style, StyleRefinement, Styled, Window,
};
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use crate::{DevicePixels, Size};
#[cfg(target_os = "macos")]
use core_video::pixel_buffer::CVPixelBuffer;
use refineable::Refineable;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use std::sync::Arc;

/// A source of a surface's content.
pub enum SurfaceSource {
    /// A macOS image buffer from CoreVideo
    #[cfg(target_os = "macos")]
    Surface(CVPixelBuffer),
    /// A GPU texture handle (type-erased to avoid depending on wgpu)
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    Texture {
        /// The GPU texture, type-erased (expected to be `Arc<wgpu::Texture>`)
        texture: Arc<dyn std::any::Any + Send + Sync>,
        /// Dimensions of the texture in device pixels
        size: Size<DevicePixels>,
    },
}

impl Clone for SurfaceSource {
    fn clone(&self) -> Self {
        match self {
            #[cfg(target_os = "macos")]
            SurfaceSource::Surface(buf) => SurfaceSource::Surface(buf.clone()),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            SurfaceSource::Texture { texture, size } => SurfaceSource::Texture {
                texture: Arc::clone(texture),
                size: *size,
            },
        }
    }
}

impl std::fmt::Debug for SurfaceSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(target_os = "macos")]
            SurfaceSource::Surface(buf) => f.debug_tuple("Surface").field(buf).finish(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            SurfaceSource::Texture { size, .. } => f
                .debug_struct("Texture")
                .field("size", size)
                .finish_non_exhaustive(),
        }
    }
}

#[cfg(target_os = "macos")]
impl From<CVPixelBuffer> for SurfaceSource {
    fn from(value: CVPixelBuffer) -> Self {
        SurfaceSource::Surface(value)
    }
}

/// A surface element.
pub struct Surface {
    source: SurfaceSource,
    object_fit: ObjectFit,
    style: StyleRefinement,
}

/// Create a new surface element.
pub fn surface(source: impl Into<SurfaceSource>) -> Surface {
    Surface {
        source: source.into(),
        object_fit: ObjectFit::Contain,
        style: Default::default(),
    }
}

impl Surface {
    /// Set the object fit for the image.
    pub fn object_fit(mut self, object_fit: ObjectFit) -> Self {
        self.object_fit = object_fit;
        self
    }
}

impl Element for Surface {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        _: &mut App,
    ) {
        match &self.source {
            #[cfg(target_os = "macos")]
            SurfaceSource::Surface(surface) => {
                let size = crate::size(surface.get_width().into(), surface.get_height().into());
                let new_bounds = self.object_fit.get_bounds(bounds, size);
                // TODO: Add support for corner_radii
                window.paint_surface(new_bounds, surface.clone());
            }
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            SurfaceSource::Texture { texture, size } => {
                let new_bounds = self.object_fit.get_bounds(bounds, *size);
                window.paint_surface(new_bounds, Arc::clone(texture), *size);
            }
        }
    }
}

impl IntoElement for Surface {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for Surface {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
