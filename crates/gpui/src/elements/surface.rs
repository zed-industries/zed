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
    /// A GPU texture handle (type-erased to avoid depending on wgpu).
    ///
    /// Expected to be `Arc<wgpu::Texture>` created on the window's
    /// [`Window::gpu_context`] device. Ported from gpui-ce
    /// ([#39](https://github.com/gpui-ce/gpui-ce/commit/6d043b22e477),
    /// [#121](https://github.com/gpui-ce/gpui-ce/pull/121)).
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
        match *self {
            #[cfg(target_os = "macos")]
            SurfaceSource::Surface(ref buf) => SurfaceSource::Surface(buf.clone()),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            SurfaceSource::Texture { ref texture, size } => SurfaceSource::Texture {
                texture: Arc::clone(texture),
                size,
            },
        }
    }
}

impl std::fmt::Debug for SurfaceSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            #[cfg(target_os = "macos")]
            SurfaceSource::Surface(ref buf) => f.debug_tuple("Surface").field(buf).finish(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            SurfaceSource::Texture { size, .. } => f
                .debug_struct("Texture")
                .field("size", &size)
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
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
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
        #[cfg_attr(
            not(any(target_os = "macos", target_os = "linux", target_os = "freebsd")),
            allow(unused_variables)
        )]
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        #[cfg_attr(
            not(any(target_os = "macos", target_os = "linux", target_os = "freebsd")),
            allow(unused_variables)
        )]
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
            #[allow(unreachable_patterns)]
            _ => {}
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

#[cfg(test)]
mod tests {
    use crate::{AppContext as _, EmptyView, TestAppContext};

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    mod linux {
        use super::super::*;
        use crate::{
            AnyWindowHandle, AppContext as _, ContentMask, Context, DevicePixels, IntoElement,
            PaintSurface, Render, ScaledPixels, Scene, TestAppContext, Window, bounds, canvas,
            point, px, size,
        };
        use std::sync::Arc;

        struct DummyTexture;

        #[test]
        fn texture_surface_inserts_into_scene() {
            let mut scene = Scene::default();
            let texture: Arc<dyn std::any::Any + Send + Sync> = Arc::new(DummyTexture);
            scene.insert_primitive(PaintSurface {
                order: 0,
                bounds: bounds(
                    point(ScaledPixels(0.), ScaledPixels(0.)),
                    size(ScaledPixels(64.), ScaledPixels(64.)),
                ),
                content_mask: ContentMask {
                    bounds: bounds(
                        point(ScaledPixels(0.), ScaledPixels(0.)),
                        size(ScaledPixels(64.), ScaledPixels(64.)),
                    ),
                },
                texture: Arc::clone(&texture),
                texture_size: size(DevicePixels(64), DevicePixels(64)),
            });
            assert_eq!(scene.surfaces.len(), 1);
            assert!(
                scene.surfaces[0]
                    .texture
                    .downcast_ref::<DummyTexture>()
                    .is_some()
            );
            assert_eq!(scene.surfaces[0].texture_size.width, DevicePixels(64));
        }

        struct TextureView {
            texture: Arc<dyn std::any::Any + Send + Sync>,
        }

        impl Render for TextureView {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                let texture = Arc::clone(&self.texture);
                canvas(|_, _, _| (), move |bounds, _, window, _| {
                    window.paint_surface(
                        bounds,
                        texture,
                        size(DevicePixels(64), DevicePixels(64)),
                    );
                })
                .w(px(64.))
                .h(px(64.))
            }
        }

        #[gpui::test]
        fn paint_texture_surface_reaches_the_scene(cx: &mut TestAppContext) {
            let texture: Arc<dyn std::any::Any + Send + Sync> = Arc::new(DummyTexture);
            let window = cx.add_window({
                let texture = Arc::clone(&texture);
                move |_, _| TextureView { texture }
            });
            let window = AnyWindowHandle::from(window);
            cx.update_window(window, |_, window, cx| window.draw(cx).clear(cx))
                .unwrap();
            cx.update_window(window, |_, window, _| {
                let surfaces = window.painted_surfaces();
                assert_eq!(surfaces.len(), 1);
                assert!(
                    surfaces[0]
                        .texture
                        .downcast_ref::<DummyTexture>()
                        .is_some()
                );
                assert_eq!(surfaces[0].texture_size, size(DevicePixels(64), DevicePixels(64)));
            })
            .unwrap();
        }
    }

    #[gpui::test]
    fn painted_surfaces_starts_empty(cx: &mut TestAppContext) {
        let window = cx.add_window(|_, _| EmptyView);
        cx.update_window(window.into(), |_, window, _| {
            assert!(window.painted_surfaces().is_empty());
        })
        .unwrap();
    }
}
