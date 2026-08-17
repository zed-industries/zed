use crate::link_to_wgpu_item;

/// Token of the user agreeing to access experimental features.
#[derive(Debug, Default, Copy, Clone)]
pub struct ExperimentalFeatures {
    enabled: bool,
}

impl ExperimentalFeatures {
    /// Uses of [`Features`] prefixed with "EXPERIMENTAL" are disallowed.
    ///
    #[doc = link_to_wgpu_item!(struct Features)]
    pub const fn disabled() -> Self {
        Self { enabled: false }
    }

    /// Uses of [`Features`] prefixed with "EXPERIMENTAL" may result
    /// in undefined behavior when used incorrectly. The exact bounds
    /// of these issues varies by the feature. These instances are
    /// inherently bugs in our implementation that we will eventually fix.
    ///
    /// By giving access to still work-in-progress APIs, users can get
    /// access to newer technology sooner, and we can work with users
    /// to fix bugs quicker.
    ///
    /// Look inside our repo at the [`api-specs`] for more information
    /// on various experimental apis.
    ///
    /// # Safety
    ///
    /// - You acknowledge that there may be UB-containing bugs in these
    ///   apis and those may be hit by calling otherwise safe code.
    /// - You agree to report any such bugs to us, if you find them.
    ///
    #[doc = link_to_wgpu_item!(struct Features)]
    /// [`api-specs`]: https://github.com/gfx-rs/wgpu/tree/trunk/docs/api-specs
    pub const unsafe fn enabled() -> Self {
        Self { enabled: true }
    }

    /// Returns true if the user has agreed to access experimental features.
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Token of the user agreeing to use [`LoadOp::DontCare`](crate::LoadOp::DontCare).
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LoadOpDontCare {
    // Private to prevent construction outside of the unsafe
    // enabled() function.
    _private: (),
}

impl LoadOpDontCare {
    /// Using [`LoadOp::DontCare`](crate::LoadOp::DontCare) will result
    /// in the render target having undefined contents at the start of the render pass.
    /// This may lead to undefined behavior if you read from the any of the
    /// render target pixels without first writing to them.
    ///
    /// Blending also becomes undefined behavior if the source
    /// pixels are undefined.
    ///
    /// All pixels in the render target must be written to before
    /// any blending or a [`StoreOp::Store`](crate::StoreOp::Store) occurs.
    ///
    /// # Safety
    ///
    /// - You acknowledge that using `LoadOp::DontCare` may lead to undefined behavior
    ///   if the above conditions are not met.
    pub const unsafe fn enabled() -> Self {
        Self { _private: () }
    }
}
