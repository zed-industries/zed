// TODO: Move this to `objc2` once we can detect simulator targets without a
// build script.

/// Note: While `objc2` relies on this, you can freely break this, since it is
/// only used behind experimental features (`unstable-static-*`).
#[repr(C)]
#[doc(hidden)]
#[allow(missing_copy_implementations)]
pub struct __ImageInfo {
    // These are not actually `unsigned int`, even though the docs say so
    /// The version of the image info struct.
    version: u32,
    flags: u32,
}

#[allow(unused)]
impl __ImageInfo {
    /// Unused
    const FIX_AND_CONTINUE: u32 = 1 << 0;
    const SUPPORTS_GARBAGE_COLLECTED: u32 = 1 << 1;
    const REQUIRES_GARBAGE_COLLECTION: u32 = 1 << 2;
    const OPTIMIZED_BY_DYLD: u32 = 1 << 3; // TODO
    /// Unused
    const CORRECTED_SYNTHESIZE: u32 = 1 << 4;
    /// Whether we're compiling this to run on a simulator.
    const IMAGE_IS_SIMULATED: u32 = 1 << 5;
    /// Whether we are generating class properties.
    const CLASS_PROPERTIES: u32 = 1 << 6;
    const DYLD_PREOPTIMIZED: u32 = 1 << 7;

    const SWIFT_ABI_VERSION_SHIFT: u32 = 8;
    const SWIFT_ABI_VERSION_MASK: u32 = 0xff << Self::SWIFT_ABI_VERSION_SHIFT;
    const SWIFT_MINOR_VERSION_SHIFT: u32 = 16;
    const SWIFT_MINOR_VERSION_MASK: u32 = 0xff << Self::SWIFT_MINOR_VERSION_SHIFT;
    const SWIFT_MAJOR_VERSION_SHIFT: u32 = 24;
    const SWIFT_MAJOR_VERSION_MASK: u32 = 0xff << Self::SWIFT_MAJOR_VERSION_SHIFT;

    /// Fetches the image info for the current runtime + target combination
    #[inline]
    pub const fn system() -> Self {
        // We don't currently do anything relating to class properties, but
        // let's mimic what Clang does!
        let mut flags = Self::CLASS_PROPERTIES;

        if cfg!(target_simulator) {
            flags |= Self::IMAGE_IS_SIMULATED;
        }

        Self { version: 0, flags }
    }
}
