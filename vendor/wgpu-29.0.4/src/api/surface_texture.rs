use crate::*;

/// Surface texture that can be rendered to.
/// Result of a successful call to [`Surface::get_current_texture`].
///
/// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
/// the [`GPUCanvasContext`](https://gpuweb.github.io/gpuweb/#canvas-context) provides
/// a texture without any additional information.
#[derive(Debug, Clone)]
pub struct SurfaceTexture {
    /// Accessible view of the frame.
    pub texture: Texture,
    pub(crate) presented: bool,
    pub(crate) detail: dispatch::DispatchSurfaceOutputDetail,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(SurfaceTexture: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(SurfaceTexture => .texture.inner);

impl SurfaceTexture {
    /// Schedule this texture to be presented on the owning surface.
    ///
    /// Needs to be called after any work on the texture is scheduled via [`Queue::submit`].
    ///
    /// # Platform dependent behavior
    ///
    /// On Wayland, `present` will attach a `wl_buffer` to the underlying `wl_surface` and commit the new surface
    /// state. If it is desired to do things such as request a frame callback, scale the surface using the viewporter
    /// or synchronize other double buffered state, then these operations should be done before the call to `present`.
    pub fn present(mut self) {
        self.presented = true;
        self.detail.present();
    }

    #[cfg(custom)]
    /// Returns custom implementation of SurfaceTexture (if custom backend and is internally T)
    pub fn as_custom<T: crate::custom::SurfaceOutputDetailInterface>(&self) -> Option<&T> {
        self.detail.as_custom()
    }
}

impl Drop for SurfaceTexture {
    fn drop(&mut self) {
        if !self.presented && !thread_panicking() {
            self.detail.texture_discard();
        }
    }
}

/// Result of a call to [`Surface::get_current_texture`].
///
/// See variant documentation for how to handle each case.
#[derive(Debug)]
pub enum CurrentSurfaceTexture {
    /// Successfully acquired a surface texture with no issues.
    Success(SurfaceTexture),
    /// Successfully acquired a surface texture, but texture no longer matches the properties of the underlying surface.
    /// It's highly recommended to call [`Surface::configure`] again for optimal performance.
    Suboptimal(SurfaceTexture),
    /// A timeout was encountered while trying to acquire the next frame.
    ///
    /// Applications should skip the current frame and try again later.
    Timeout,
    /// The window is occluded (e.g. minimized or behind another window).
    ///
    /// Applications should skip the current frame and try again once the window
    /// is no longer occluded.
    Occluded,
    /// The underlying surface has changed, and therefore the surface configuration is outdated.
    ///
    /// Call [`Surface::configure()`] and try again.
    Outdated,
    /// The surface has been lost and needs to be recreated.
    ///
    /// If the device as a whole is lost (see [`set_device_lost_callback()`][crate::Device::set_device_lost_callback]), then
    /// you need to recreate the device and all resources.
    /// Otherwise, call [`Instance::create_surface()`] to recreate the surface,
    /// then [`Surface::configure()`], and try again.
    Lost,
    /// A validation error inside [`Surface::get_current_texture()`] was raised
    /// and caught by an [error scope](crate::Device::push_error_scope) or
    /// [`on_uncaptured_error()`][crate::Device::on_uncaptured_error].
    ///
    /// Applications should attend to the validation error and try again.
    Validation,
}

fn thread_panicking() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(std)] {
            std::thread::panicking()
        } else if #[cfg(panic = "abort")] {
            // If `panic = "abort"` then a thread _cannot_ be observably panicking by definition.
            false
        } else {
            // TODO: This is potentially overly pessimistic; it may be appropriate to instead allow a
            // texture to not be discarded.
            // Alternatively, this could _also_ be a `panic!`, since we only care if the thread is panicking
            // when the surface has not been presented.
            compile_error!(
                "cannot determine if a thread is panicking without either `panic = \"abort\"` or `std`"
            );
        }
    }
}
