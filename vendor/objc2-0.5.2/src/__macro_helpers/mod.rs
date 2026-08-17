pub use core::borrow::{Borrow, BorrowMut};
pub use core::cell::UnsafeCell;
pub use core::convert::{AsMut, AsRef};
pub use core::marker::{PhantomData, Sized};
pub use core::mem::{size_of, ManuallyDrop, MaybeUninit};
pub use core::ops::{Deref, DerefMut};
pub use core::option::Option::{self, None, Some};
pub use core::primitive::{bool, isize, str, u8};
pub use core::{compile_error, concat, panic, stringify};
// TODO: Use `core::cell::LazyCell`
pub use std::sync::Once;

mod cache;
mod common_selectors;
mod convert;
mod declare_class;
pub(crate) mod declared_ivars;
mod method_family;
mod msg_send;
mod msg_send_id;
mod writeback;

pub use self::cache::{CachedClass, CachedSel};
pub use self::common_selectors::{alloc_sel, dealloc_sel, init_sel, new_sel};
pub use self::convert::{ConvertArgument, ConvertArguments, ConvertReturn, TupleExtender};
pub use self::declare_class::{
    assert_mutability_matches_superclass_mutability, ClassBuilderHelper,
    ClassProtocolMethodsBuilder, IdReturnValue, MaybeOptionId, MessageRecieveId,
    ValidSubclassMutability,
};
pub use self::declared_ivars::DeclaredIvarsHelper;
pub use self::method_family::{
    retain_semantics, Alloc, CopyOrMutCopy, Init, New, Other, RetainSemantics,
};
pub use self::msg_send::MsgSend;
pub use self::msg_send_id::{MaybeUnwrap, MsgSendId, MsgSendSuperId};

/// Disallow using this passed in value in const and statics for forwards
/// compatibility (this function is not a `const` function).
#[inline]
pub fn disallow_in_static<T>(item: &'static T) -> &'static T {
    item
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "objc2-proc-macros")]
    use crate::__hash_idents;

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_different() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(def));
    }

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_same_no_equal() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(abc));
        assert_ne!(__hash_idents!(abc def ghi), __hash_idents!(abc def ghi));
    }

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_exact_same_ident() {
        macro_rules! x {
            ($x:ident) => {
                (__hash_idents!($x), __hash_idents!($x))
            };
        }
        let (ident1, ident2) = x!(abc);
        // This is a limitation of `__hash_idents`, ideally we'd like these
        // to be different!
        assert_eq!(ident1, ident2);
    }

    #[test]
    #[cfg_attr(
        not(all(target_vendor = "apple", target_os = "macos", target_arch = "x86")),
        ignore = "Only relevant on macOS 32-bit"
    )]
    fn ensure_size_of_module_info() {
        assert_eq!(core::mem::size_of::<ModuleInfo>(), 16);
    }
}
