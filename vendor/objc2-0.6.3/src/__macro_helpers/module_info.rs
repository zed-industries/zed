/// Helper struct for emitting the module info that macOS 32-bit requires.
///
/// <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCMac.cpp#L5211-L5234>
#[repr(C)]
#[derive(Debug)]
pub struct ModuleInfo {
    version: usize,
    size: usize,
    name: *const u8,
    symtab: *const (),
}

// SAFETY: ModuleInfo is immutable.
unsafe impl Sync for ModuleInfo {}

impl ModuleInfo {
    /// This is hardcoded in clang as 7.
    const VERSION: usize = 7;

    pub const fn new(name: *const u8) -> Self {
        Self {
            version: Self::VERSION,
            size: core::mem::size_of::<Self>(),
            name,
            // We don't expose any symbols
            symtab: core::ptr::null(),
        }
    }
}
