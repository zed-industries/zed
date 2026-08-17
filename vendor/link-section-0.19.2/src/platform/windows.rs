//! Windows: alignment markers at section bounds.

/// On Windows platforms we don't have start/end symbols, but we do have
/// section sorting so we drop a minimum-sized type with the same alignment
/// as T at the start and end of the section.
#[doc(hidden)]
#[macro_export]
macro_rules! __get_section_windows {
    // Movable sections have an additional backref section.
    (movable, name=$name:tt, type=$generic_ty:ty) => {
        {
            use $crate::__support::Alignment;
            use $crate::__support::PtrBounds;
            use $crate::__support::add_section_link_attribute;
            use core::mem;
            use $crate::__support::SyncUnsafeCell;

            if cfg!(miri) {
                // Miri doesn't support link section sorting
                $crate::__support::MovableBounds::new(
                    PtrBounds::new(::core::ptr::null(), ::core::ptr::null()),
                    PtrBounds::new(::core::ptr::null(), ::core::ptr::null()),
                )
            } else {
                add_section_link_attribute!(
                    item data start $name
                    #[link_section = __]
                    static __START: SyncUnsafeCell<Alignment<$generic_ty>> = SyncUnsafeCell::new(Alignment::new());
                );
                add_section_link_attribute!(
                    item data end $name
                    #[link_section = __]
                    static __END: SyncUnsafeCell<Alignment<$generic_ty>> = SyncUnsafeCell::new(Alignment::new());
                );
                add_section_link_attribute!(
                    backref data start $name
                    #[link_section = __]
                    static __REF_START: SyncUnsafeCell<Alignment<$crate::MovableBackref<$generic_ty>>> =
                        SyncUnsafeCell::new(Alignment::new());
                );
                add_section_link_attribute!(
                    backref data end $name
                    #[link_section = __]
                    static __REF_END: SyncUnsafeCell<Alignment<$crate::MovableBackref<$generic_ty>>> =
                        SyncUnsafeCell::new(Alignment::new());
                );

                let start = unsafe { &raw const __START as *const () };
                let start = unsafe {
                    start.cast::<u8>().add(mem::size_of::<Alignment<$generic_ty>>()) as *const()
                };
                let end = unsafe { &raw const __END as *const () };

                let ref_start = unsafe { &raw const __REF_START as *const () };
                let ref_start = unsafe {
                    ref_start.cast::<u8>().add(mem::size_of::<Alignment<$crate::MovableBackref<$generic_ty>>>()) as *const ()
                };
                let ref_end = unsafe { &raw const __REF_END as *const () };

                $crate::__support::MovableBounds::new(
                    PtrBounds::new(start, end),
                    PtrBounds::new(ref_start, ref_end),
                )
            }
        }
    };
    // Mutable sections must use UnsafeCell to match items
    (mutable, name=$ident:tt, type=$generic_ty:ty) => {
        {
            use $crate::__support::Alignment;
            use $crate::__support::PtrBounds;
            use $crate::__support::add_section_link_attribute;
            use core::mem;
            use $crate::__support::SyncUnsafeCell;

            if cfg!(miri) {
                // Miri doesn't support link section sorting
                PtrBounds::new(::core::ptr::null(), ::core::ptr::null())
            } else {
                add_section_link_attribute!(
                    item data start $ident
                    #[link_section = __]
                    static __START: SyncUnsafeCell<Alignment<$generic_ty>> = SyncUnsafeCell::new(Alignment::new());
                );
                let start = unsafe {
                    let start = &raw const __START;
                    start.cast::<u8>().add(mem::size_of::<Alignment<$generic_ty>>()) as *const()
                };
                add_section_link_attribute!(
                    item data end $ident
                    #[link_section = __]
                    static __END: SyncUnsafeCell<Alignment<$generic_ty>> = SyncUnsafeCell::new(Alignment::new());
                );
                let end = unsafe { &raw const __END as *const () };

                PtrBounds::new(start, end)
            }
        }
    };
    ($section_type:ident, name=$ident:tt, type=$generic_ty:ty) => {
        {
            use $crate::__support::Alignment;
            use $crate::__support::PtrBounds;
            use $crate::__support::add_section_link_attribute;
            use core::mem;

            if cfg!(miri) {
                // Miri doesn't support link section sorting
                PtrBounds::new(::core::ptr::null(), ::core::ptr::null())
            } else {
                add_section_link_attribute!(
                    item data start $ident
                    #[link_section = __]
                    static __START: Alignment<$generic_ty> = Alignment::new();
                );
                let start = unsafe {
                    let start = &raw const __START;
                    start.cast::<u8>().add(mem::size_of::<Alignment<$generic_ty>>()) as *const()
                };
                add_section_link_attribute!(
                    item data end $ident
                    #[link_section = __]
                    static __END: Alignment<$generic_ty> = Alignment::new();
                );
                let end = unsafe { &raw const __END as *const () };

                PtrBounds::new(start, end)
            }
        }
    }
}

pub use crate::__get_section_windows as get_section;

crate::__def_section_name! {
    __section_name_windows,
    {
        data bare =>    (".data" "$") __ ();
        data section => (".data" "$") __ ("$b");
        data start =>   (".data" "$") __ ("$a");
        data end =>     (".data" "$") __ ("$c");
        code bare =>    (".text" "$") __ ();
        code section => (".text" "$") __ ("$b");
        code start =>   (".text" "$") __ ("$a");
        code end =>     (".text" "$") __ ("$c");
    }
    AUXILIARY = "$d$";
    REFS = "$r$";
    MAX_LENGTH = 64;
    HASH_LENGTH = 10;
    VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
}
