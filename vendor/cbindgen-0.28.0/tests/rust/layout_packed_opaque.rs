#[repr(align(1), C)]
pub union Align1Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

#[repr(align(4), C)]
pub union Align4Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

#[repr(align(16), C)]
pub union Align16Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

#[repr(align(1), C)]
pub struct Align1Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

#[repr(align(2), C)]
pub struct Align2Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

#[repr(align(4), C)]
pub struct Align4Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

#[repr(align(8), C)]
pub struct Align8Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

#[repr(align(32), C)]
pub struct Align32Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Opaque because packed is not defined.
#[repr(packed, C)]
pub struct OpaquePackedStruct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Opaque because packed is not defined.
#[repr(packed, C)]
pub union OpaquePackedUnion {
    pub variant1: usize,
    pub variant2: *mut u8,
}
