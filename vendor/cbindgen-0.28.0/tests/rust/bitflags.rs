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
        const ALIAS = Self::END.bits();
        /// 'flex-start'
        const FLEX_START = 1 << 3;
        const MIXED = 1 << 4 | AlignFlags::FLEX_START.bits() | AlignFlags::END.bits();
        const MIXED_SELF = 1 << 5 | AlignFlags::FLEX_START.bits() | AlignFlags::END.bits();
    }
}

bitflags! {
    #[repr(C)]
    pub struct DebugFlags: u32 {
        /// Flag with the topmost bit set of the u32
        const BIGGEST_ALLOWED = 1 << 31;
    }
}

bitflags! {
    #[repr(C)]
    pub struct LargeFlags: u64 {
        /// Flag with a very large shift that usually would be narrowed.
        const LARGE_SHIFT = 1u64 << 44;
        const INVERTED = !Self::LARGE_SHIFT.bits();
    }
}

// bitflags 2 allows to define types out-of-line for custom derives
// #[derive(SomeTrait)]
#[repr(C)]
pub struct OutOfLine(u32);

bitflags! {
    impl OutOfLine: u32 {
        const A = 1;
        const B = 2;
        const AB = Self::A.bits() | Self::B.bits();
    }
}

#[no_mangle]
pub extern "C" fn root(
    flags: AlignFlags,
    bigger_flags: DebugFlags,
    largest_flags: LargeFlags,
    out_of_line: OutOfLine,
) {
}
