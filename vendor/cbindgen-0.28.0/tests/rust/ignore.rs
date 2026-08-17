/// cbindgen:ignore
#[no_mangle]
pub extern "C" fn root() {}

/// cbindgen:ignore
///
/// Something else.
#[no_mangle]
pub extern "C" fn another_root() {}

#[no_mangle]
pub extern "C" fn no_ignore_root() {}

/// cbindgen:ignore
#[repr(C)]
pub struct IgnoreStruct {}

pub struct IgnoreStructWithImpl;

/// cbindgen:ignore
impl IgnoreStructWithImpl {
    #[no_mangle]
    pub extern "C" fn ignore_associated_method() {}

    pub const IGNORE_INNER_CONST: u32 = 0;
}

/// cbindgen:ignore
pub const IGNORE_CONST: u32 = 0;

pub const NO_IGNORE_CONST: u32 = 0;

pub struct NoIgnoreStructWithImpl;

impl NoIgnoreStructWithImpl {
    /// cbindgen:ignore
    #[no_mangle]
    pub extern "C" fn ignore_associated_method() {}

    #[no_mangle]
    pub extern "C" fn no_ignore_associated_method() {}

    /// cbindgen:ignore
    pub const IGNORE_INNER_CONST: u32 = 0;

    pub const NO_IGNORE_INNER_CONST: u32 = 0;
}

/// cbindgen:ignore
enum IgnoreEnum {}

enum NoIgnoreEnum {}
