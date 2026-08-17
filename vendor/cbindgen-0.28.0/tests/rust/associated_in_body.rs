bitflags! {
    /// Constants shared by multiple CSS Box Alignment properties
    ///
    /// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
    #[derive(MallocSizeOf)]
    #[repr(C)]
    pub struct AlignFlags: u8 {
        /// 'auto'
        const AUTO = 0;
        /// 'normal'
        const NORMAL = 1;
        /// 'start'
        const START = 1 << 1;
        /// 'end'
        const END = 1 << 2;
        const ALIAS = Self::END.bits;
        /// 'flex-start'
        const FLEX_START = 1 << 3;
        const MIXED = 1 << 4 | AlignFlags::FLEX_START.bits | AlignFlags::END.bits;
        const MIXED_SELF = 1 << 5 | AlignFlags::FLEX_START.bits | AlignFlags::END.bits;
    }
}

/// An arbitrary identifier for a native (OS compositor) surface
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NativeSurfaceId(pub u64);

impl NativeSurfaceId {
    /// A special id for the native surface that is used for debug / profiler overlays.
    pub const DEBUG_OVERLAY: NativeSurfaceId = NativeSurfaceId(u64::MAX);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "capture", derive(Serialize))]
#[cfg_attr(feature = "replay", derive(Deserialize))]
pub struct NativeTileId {
    pub surface_id: NativeSurfaceId,
    pub x: i32,
    pub y: i32,
}

impl NativeTileId {
    /// A special id for the native surface that is used for debug / profiler overlays.
    pub const DEBUG_OVERLAY: NativeTileId = NativeTileId {
        surface_id: NativeSurfaceId::DEBUG_OVERLAY,
        x: 0,
        y: 0,
    };
}

#[no_mangle]
pub extern "C" fn root(flags: AlignFlags, tile: NativeTileId) {}
