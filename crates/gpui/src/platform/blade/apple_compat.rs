use super::{BladeContext, BladeRenderer, BladeSurfaceConfig};
use blade_graphics as gpu;
use std::{ffi::c_void, ptr::NonNull};

#[derive(Clone)]
pub struct Context {
    inner: BladeContext,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            inner: BladeContext::new().unwrap(),
        }
    }
}

pub type Renderer = BladeRenderer;

pub unsafe fn new_renderer(
    context: Context,
    _native_window: *mut c_void,
    native_view: *mut c_void,
    bounds: crate::Size<f32>,
    transparent: bool,
) -> Renderer {
    use raw_window_handle as rwh;
    struct RawWindow {
        view: *mut c_void,
    }

    impl rwh::HasWindowHandle for RawWindow {
        fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
            let view = NonNull::new(self.view).unwrap();
            let handle = rwh::AppKitWindowHandle::new(view);
            Ok(unsafe { rwh::WindowHandle::borrow_raw(handle.into()) })
        }
    }
    impl rwh::HasDisplayHandle for RawWindow {
        fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
            let handle = rwh::AppKitDisplayHandle::new();
            Ok(unsafe { rwh::DisplayHandle::borrow_raw(handle.into()) })
        }
    }

    BladeRenderer::new(
        &context.inner,
        &RawWindow {
            view: native_view as *mut _,
        },
        BladeSurfaceConfig {
            size: gpu::Extent {
                width: bounds.width as u32,
                height: bounds.height as u32,
                depth: 1,
            },
            transparent,
        },
    )
    .unwrap()
}
