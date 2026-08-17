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

#[repr(packed, C)]
pub struct PackedStruct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

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

#[repr(packed, C)]
pub union PackedUnion {
    pub variant1: usize,
    pub variant2: *mut u8,
}

// #[repr(packed(n), C)] structs are currently unsupported.
#[repr(packed(4), C)]
pub struct UnsupportedPacked4Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// #[repr(packed(n), C)] unions are currently unsupported.
#[repr(packed(4), C)]
pub union UnsupportedPacked4Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

// #[repr(align(n), C)] enums are currently unsupported.
#[repr(align(4), C)]
pub enum UnsupportedAlign4Enum {
    Variant1,
    Variant2,
}

// Non-repr(C) structs aren't translated.
#[repr(align(4))]
pub struct RustAlign4Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Non-repr(C) structs aren't translated.
#[repr(packed)]
pub struct RustPackedStruct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Non-repr(C) unions aren't translated.
#[repr(align(4))]
pub struct RustAlign4Union {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Non-repr(C) unions aren't translated.
#[repr(packed)]
pub struct RustPackedUnion {
    pub arg1: usize,
    pub arg2: *mut u8,
}
