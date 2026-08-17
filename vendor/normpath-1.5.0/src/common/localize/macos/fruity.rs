//! Implementations copied and modified from Fruity.
//!
//! Sources:
//! - <https://github.com/nvzqz/fruity/tree/c1d067e8ec03e20bb6f52f7c3ab63cadd2a2b261>
//!
//! Copyrights:
//! - Copyright (c) 2020 Nikolai Vazquez<br>
//!   <https://github.com/nvzqz/fruity/blob/c1d067e8ec03e20bb6f52f7c3ab63cadd2a2b261/LICENSE-MIT#L3>
//! - Modifications copyright (c) 2020 dylni (<https://github.com/dylni>)<br>
//!   <https://github.com/dylni/normpath/blob/master/COPYRIGHT>

use std::os::raw::c_char;

macro_rules! __stringify_sel {
    ( $name:ident ) => {
        ::std::stringify!($name)
    };
    ( $($name:ident :)+ ) => {
        ::std::concat!($(::std::stringify!($name), ":"),+)
    };
}

macro_rules! selector {
    ( $($token:tt)* ) => {{
        let sel: *const _ = ::std::concat!(__stringify_sel!($($token)*), "\0");
        unsafe {
            $crate::localize::macos::fruity::sel_registerName(sel.cast())
        }
    }};
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {}

extern "C" {
    pub(super) fn sel_registerName(name: *const c_char) -> objc::SEL;
}

pub(super) mod objc {
    use std::cell::UnsafeCell;
    use std::ops::Deref;
    use std::os::raw::c_char;
    use std::os::raw::c_void;
    use std::ptr::NonNull;

    #[expect(clippy::upper_case_acronyms)]
    pub(super) type BOOL = c_char;

    #[expect(clippy::upper_case_acronyms)]
    pub(super) const NO: BOOL = 0;

    pub(in super::super) type NSUInteger = usize;

    #[expect(clippy::upper_case_acronyms)]
    #[repr(transparent)]
    pub(in super::super) struct SEL(NonNull<c_void>);

    #[repr(C)]
    pub(in super::super) struct Object(UnsafeCell<[u8; 0]>);

    #[repr(C)]
    pub(in super::super) struct Class(Object);

    impl Class {
        pub(super) fn alloc(&self) -> NSObject {
            extern "C" {
                fn objc_msgSend(obj: &Class, sel: SEL) -> NSObject;
            }

            let sel = selector!(alloc);

            unsafe { objc_msgSend(self, sel) }
        }
    }

    #[expect(non_camel_case_types)]
    #[repr(transparent)]
    pub(in super::super) struct id(NonNull<Object>);

    impl Deref for id {
        type Target = Object;

        fn deref(&self) -> &Self::Target {
            unsafe { self.0.as_ref() }
        }
    }

    impl Drop for id {
        fn drop(&mut self) {
            extern "C" {
                fn objc_release(obj: &Object);
            }

            unsafe { objc_release(self) }
        }
    }

    #[repr(transparent)]
    pub(in super::super) struct NSObject(id);

    impl Deref for NSObject {
        type Target = id;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

pub(super) mod foundation {
    use std::ops::Deref;

    use super::objc::Class;
    use super::objc::NSObject;
    use super::objc::NSUInteger;
    use super::objc::BOOL;
    use super::objc::NO;
    use super::objc::SEL;

    #[repr(transparent)]
    pub(in super::super) struct NSStringEncoding(NSUInteger);

    impl NSStringEncoding {
        pub(in super::super) const UTF8: Self = Self(4);
    }

    #[repr(transparent)]
    pub(in super::super) struct NSString(NSObject);

    impl NSString {
        fn class() -> &'static Class {
            extern "C" {
                #[link_name = "OBJC_CLASS_$_NSString"]
                static CLASS: Class;
            }
            unsafe { &CLASS }
        }

        pub(in super::super) unsafe fn from_str_no_copy(string: &str) -> Self {
            extern "C" {
                fn objc_msgSend(
                    obj: NSString,
                    sel: SEL,
                    bytes: *const u8,
                    length: NSUInteger,
                    encoding: NSStringEncoding,
                    free_when_done: BOOL,
                ) -> NSString;
            }

            let obj = Self(Self::class().alloc());
            let sel =
                selector!(initWithBytesNoCopy:length:encoding:freeWhenDone:);
            let bytes = string.as_ptr();
            let length = string.len();
            let encoding = NSStringEncoding::UTF8;
            let free_when_done = NO;

            unsafe {
                objc_msgSend(obj, sel, bytes, length, encoding, free_when_done)
            }
        }
    }

    impl Deref for NSString {
        type Target = NSObject;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
