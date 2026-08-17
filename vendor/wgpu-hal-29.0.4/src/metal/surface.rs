use alloc::borrow::ToOwned as _;

use objc2::{
    available,
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
    ClassType, Message,
};
use objc2_core_foundation::CGSize;
use objc2_foundation::NSObjectProtocol;
use objc2_metal::MTLTextureType;
use objc2_quartz_core::{CAMetalDrawable, CAMetalLayer};
use parking_lot::{Mutex, RwLock};

use super::OsFeatures;

impl super::Surface {
    pub fn new(layer: Retained<CAMetalLayer>) -> Self {
        Self {
            render_layer: Mutex::new(layer),
            swapchain_format: RwLock::new(None),
            extent: RwLock::new(wgt::Extent3d::default()),
        }
    }

    pub fn from_layer(layer: &CAMetalLayer) -> Self {
        assert!(layer.isKindOfClass(CAMetalLayer::class()));
        Self::new(layer.retain())
    }

    pub fn render_layer(&self) -> &Mutex<Retained<CAMetalLayer>> {
        &self.render_layer
    }

    /// Gets the current dimensions of the `Surface`.
    ///
    /// This function is safe to call off of the main thread. However, note that
    /// `bounds` and `contentsScale` may be modified by the main thread while
    /// this function is running, possibly resulting in the two values being out
    /// of sync. This is sound, as these properties are accessed atomically.
    /// See: <https://github.com/gfx-rs/wgpu/pull/7692>
    pub(super) fn dimensions(&self) -> wgt::Extent3d {
        let (size, scale) = {
            let render_layer = self.render_layer.lock();
            let bounds = render_layer.bounds();
            let contents_scale = render_layer.contentsScale();
            (bounds.size, contents_scale)
        };

        wgt::Extent3d {
            width: (size.width * scale) as u32,
            height: (size.height * scale) as u32,
            depth_or_array_layers: 1,
        }
    }
}

impl crate::Surface for super::Surface {
    type A = super::Api;

    unsafe fn configure(
        &self,
        device: &super::Device,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        log::debug!("build swapchain {config:?}");

        let caps = &device.shared.private_texture_format_caps;
        *self.swapchain_format.write() = Some(config.format);
        *self.extent.write() = config.extent;

        let render_layer = self.render_layer.lock();
        let framebuffer_only = config.usage == wgt::TextureUses::COLOR_TARGET;
        let display_sync = match config.present_mode {
            wgt::PresentMode::Fifo => true,
            wgt::PresentMode::Immediate => false,
            m => unreachable!("Unsupported present mode: {m:?}"),
        };
        let drawable_size = CGSize::new(config.extent.width as f64, config.extent.height as f64);

        match config.composite_alpha_mode {
            wgt::CompositeAlphaMode::Opaque => render_layer.setOpaque(true),
            wgt::CompositeAlphaMode::PostMultiplied => render_layer.setOpaque(false),
            _ => (),
        }

        let device_raw = &device.shared.device;
        render_layer.setDevice(Some(device_raw));
        render_layer.setPixelFormat(caps.map_format(config.format));
        render_layer.setFramebufferOnly(framebuffer_only);
        // opt-in to Metal EDR
        // EDR potentially more power used in display and more bandwidth, memory footprint.
        let wants_edr = config.format == wgt::TextureFormat::Rgba16Float;
        if wants_edr != render_layer.wantsExtendedDynamicRangeContent() {
            render_layer.setWantsExtendedDynamicRangeContent(wants_edr);
        }

        // this gets ignored on iOS for certain OS/device combinations (iphone5s iOS 10.3)
        render_layer.setMaximumDrawableCount(config.maximum_frame_latency as usize + 1);
        render_layer.setDrawableSize(drawable_size);
        // https://developer.apple.com/documentation/quartzcore/cametallayer/allowsnextdrawabletimeout
        if available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0) {
            render_layer.setAllowsNextDrawableTimeout(false);
        }
        if OsFeatures::display_sync() {
            render_layer.setDisplaySyncEnabled(display_sync);
        }

        Ok(())
    }

    unsafe fn unconfigure(&self, _device: &super::Device) {
        *self.swapchain_format.write() = None;
    }

    unsafe fn acquire_texture(
        &self,
        _timeout: Option<core::time::Duration>, // TODO
        _fence: &super::Fence,
    ) -> Result<crate::AcquiredSurfaceTexture<super::Api>, crate::SurfaceError> {
        let render_layer = self.render_layer.lock();

        #[cfg(target_os = "macos")]
        {
            // Workaround for https://github.com/gfx-rs/wgpu/issues/8309
            // When the window is occluded on macOS, presented drawables get stuck waiting
            // for vsync. Check the window's occlusion state and skip acquisition if
            // the window is not visible - this avoids a 1-second hang in nextDrawable().
            use objc2::rc::Retained;
            use objc2::runtime::NSObject;
            use objc2_quartz_core::CALayer;

            // The CAMetalLayer is typically a sublayer, so we need to traverse up
            // to find the root layer whose delegate is the NSView.
            let mut current_layer: Option<Retained<CALayer>> =
                Some(Retained::into_super(render_layer.clone()));

            while let Some(layer) = current_layer {
                if let Some(delegate) = layer.delegate() {
                    // Found a layer with a delegate - this should be the NSView
                    let window: Option<Retained<NSObject>> =
                        unsafe { objc2::msg_send![&*delegate, window] };

                    if let Some(window) = window {
                        const NS_WINDOW_OCCLUSION_STATE_VISIBLE: usize = 1 << 1;
                        let occlusion_state: usize =
                            unsafe { objc2::msg_send![&*window, occlusionState] };
                        let is_visible = (occlusion_state & NS_WINDOW_OCCLUSION_STATE_VISIBLE) != 0;

                        if !is_visible {
                            return Err(crate::SurfaceError::Occluded);
                        }
                    }
                    break;
                }
                current_layer = layer.superlayer();
            }
        }

        let (drawable, texture) = match autoreleasepool(|_| {
            render_layer
                .nextDrawable()
                .map(|drawable| (drawable.to_owned(), drawable.texture().to_owned()))
        }) {
            Some(pair) => pair,
            None => return Err(crate::SurfaceError::Timeout),
        };

        let swapchain_format = self.swapchain_format.read().unwrap();
        let extent = self.extent.read();
        let suf_texture = super::SurfaceTexture {
            texture: super::Texture {
                raw: texture,
                format: swapchain_format,
                raw_type: MTLTextureType::Type2D,
                array_layers: 1,
                mip_levels: 1,
                copy_size: crate::CopyExtent {
                    width: extent.width,
                    height: extent.height,
                    depth: 1,
                },
            },
            drawable: ProtocolObject::from_retained(drawable),
            present_with_transaction: render_layer.presentsWithTransaction(),
        };

        Ok(crate::AcquiredSurfaceTexture {
            texture: suf_texture,
            suboptimal: false,
        })
    }

    unsafe fn discard_texture(&self, _texture: super::SurfaceTexture) {}
}
