#[repr(packed, C)]
pub struct PackedStruct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

#[repr(packed, C)]
pub union PackedUnion {
    pub variant1: usize,
    pub variant2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(1), C)]
pub union OpaqueAlign1Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(4), C)]
pub union OpaqueAlign4Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(16), C)]
pub union OpaqueAlign16Union {
    pub variant1: usize,
    pub variant2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(1), C)]
pub struct OpaqueAlign1Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(2), C)]
pub struct OpaqueAlign2Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(4), C)]
pub struct OpaqueAlign4Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(8), C)]
pub struct OpaqueAlign8Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}

// Opaque because aligned_n is not defined.
#[repr(align(32), C)]
pub struct OpaqueAlign32Struct {
    pub arg1: usize,
    pub arg2: *mut u8,
}
