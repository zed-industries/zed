pub use core::borrow::Borrow;
pub use core::cell::UnsafeCell;
pub use core::cmp::{Eq, PartialEq};
pub use core::convert::AsRef;
pub use core::default::Default;
pub use core::fmt;
pub use core::hash::{Hash, Hasher};
pub use core::marker::{PhantomData, Sized};
pub use core::mem::{size_of, transmute, ManuallyDrop, MaybeUninit};
pub use core::ops::Deref;
pub use core::option::Option::{self, None, Some};
pub use core::primitive::{bool, isize, str, u8};
pub use core::{compile_error, concat, env, module_path, panic, stringify};
// TODO: Use `core::cell::LazyCell`
pub use std::sync::Once;

mod cache;
mod class;
mod common_selectors;
mod convert;
mod define_class;
pub(crate) mod defined_ivars;
mod image_info;
mod method_family;
mod module_info;
mod msg_send_retained;
mod null_error;
mod os_version;
mod retain_semantics;
mod sync_unsafe_cell;
mod writeback;

pub use self::cache::{CachedClass, CachedSel};
pub use self::class::{DoesNotImplDrop, MainThreadOnlyDoesNotImplSendSync, ValidThreadKind};
pub use self::common_selectors::{alloc_sel, dealloc_sel, init_sel, new_sel};
pub use self::convert::{ConvertArgument, ConvertArguments, ConvertReturn, TupleExtender};
pub use self::define_class::{
    ClassBuilderHelper, ClassProtocolMethodsBuilder, MaybeOptionRetained, MessageReceiveRetained,
    RetainedReturnValue, ThreadKindAutoTraits,
};
pub use self::defined_ivars::DefinedIvarsHelper;
pub use self::image_info::ImageInfo;
pub use self::method_family::{
    method_family, method_family_import, AllocFamily, AutoreleaseSelector, CopyFamily,
    DeallocSelector, InitFamily, MethodFamily, MutableCopyFamily, NewFamily, NoneFamily,
    ReleaseSelector, RetainSelector,
};
pub use self::module_info::ModuleInfo;
pub use self::msg_send_retained::{MsgSend, MsgSendError, MsgSendSuper, MsgSendSuperError};
pub use self::os_version::{is_available, AvailableVersion, OSVersion};
pub use self::retain_semantics::{
    KindDefined, KindSendMessage, KindSendMessageSuper, RetainSemantics,
};
pub use self::sync_unsafe_cell::SyncUnsafeCell;

/// Disallow using this passed in value in const and statics for forwards
/// compatibility (this function is not a `const` function).
#[inline]
pub fn disallow_in_static<T>(item: &'static T) -> &'static T {
    item
}

#[deprecated = "having the `impl` inside `extern_methods!` is deprecated, move it outside instead"]
pub const fn extern_methods_unsafe_impl() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
    use crate::__hash_idents;

    #[test]
    #[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
    fn hash_idents_different() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(def));
    }

    #[test]
    #[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
    fn hash_idents_same_no_equal() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(abc));
        assert_ne!(__hash_idents!(abc def ghi), __hash_idents!(abc def ghi));
    }

    #[test]
    #[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
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
