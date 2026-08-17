use core::cmp::Ordering;
use core::ptr;

use crate::{kCFBooleanFalse, kCFBooleanTrue, CFBoolean, CFNumber, CFNumberType, CFRetained};

impl CFBoolean {
    pub fn new(value: bool) -> &'static CFBoolean {
        if value {
            unsafe { kCFBooleanTrue }.unwrap()
        } else {
            unsafe { kCFBooleanFalse }.unwrap()
        }
    }

    pub fn as_bool(&self) -> bool {
        self.value()
    }
}

macro_rules! def_new_fn {
    {$(
        $(#[$($m:meta)*])*
        ($fn_name:ident($fn_inp:ty); $type:ident),
    )*} => {$(
        $(#[$($m)*])*
        #[inline]
        pub fn $fn_name(val: $fn_inp) -> CFRetained<Self> {
            let ptr: *const $fn_inp = &val;
            unsafe { Self::new(None, CFNumberType::$type, ptr.cast()).expect("failed creating CFNumber") }
        }
    )*}
}

impl CFNumber {
    def_new_fn! {
        (new_i8(i8); SInt8Type),
        (new_i16(i16); SInt16Type),
        (new_i32(i32); SInt32Type),
        (new_i64(i64); SInt64Type),
        (new_isize(isize); NSIntegerType),

        (new_f32(f32); Float32Type),
        (new_f64(f64); Float64Type),
    }

    #[cfg(feature = "CFCGTypes")]
    #[inline]
    pub fn new_cgfloat(val: crate::CGFloat) -> CFRetained<Self> {
        #[cfg(not(target_pointer_width = "64"))]
        {
            Self::new_f32(val)
        }
        #[cfg(target_pointer_width = "64")]
        {
            Self::new_f64(val)
        }
    }
}

macro_rules! def_get_fn {
    {$(
        $(#[$($m:meta)*])*
        ($fn_name:ident -> $fn_ret:ty; $type:ident),
    )*} => {$(
        $(#[$($m)*])*
        #[inline]
        pub fn $fn_name(&self) -> Option<$fn_ret> {
            let mut value: $fn_ret = <$fn_ret>::default();
            let ptr: *mut $fn_ret = &mut value;
            let ret = unsafe { self.value(CFNumberType::$type, ptr.cast()) };
            if ret {
                Some(value)
            } else {
                None
            }
        }
    )*}
}

impl CFNumber {
    def_get_fn! {
        (as_i8 -> i8; SInt8Type),
        (as_i16 -> i16; SInt16Type),
        (as_i32 -> i32; SInt32Type),
        (as_i64 -> i64; SInt64Type),
        (as_isize -> isize; NSIntegerType),

        (as_f32 -> f32; Float32Type),
        (as_f64 -> f64; Float64Type),
    }

    #[cfg(feature = "CFCGTypes")]
    #[inline]
    pub fn as_cgfloat(&self) -> Option<crate::CGFloat> {
        #[cfg(not(target_pointer_width = "64"))]
        {
            self.as_f32()
        }
        #[cfg(target_pointer_width = "64")]
        {
            self.as_f64()
        }
    }
}

impl PartialOrd for CFNumber {
    #[inline]
    #[doc(alias = "CFNumberCompare")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CFNumber {
    #[inline]
    #[doc(alias = "CFNumberCompare")]
    fn cmp(&self, other: &Self) -> Ordering {
        // Documented that one should pass NULL here.
        let context = ptr::null_mut();
        unsafe { self.compare(Some(other), context) }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_bool() {
        let cffalse = CFBoolean::new(false);
        let cftrue = CFBoolean::new(true);
        assert_ne!(cffalse, cftrue);
        assert_eq!(cftrue, CFBoolean::new(true));
        assert_eq!(cffalse, CFBoolean::new(false));
        assert!(!cffalse.as_bool());
        assert!(cftrue.as_bool());
    }

    #[test]
    fn to_from_number() {
        let n = CFNumber::new_i32(442);
        if cfg!(all(target_os = "macos", target_arch = "x86")) {
            // Seems to return `None` in the old runtime.
            assert_eq!(n.as_i8(), None);
        } else {
            assert_eq!(n.as_i8(), Some(442i32 as i8));
        }
        assert_eq!(n.as_i16(), Some(442));
        assert_eq!(n.as_i32(), Some(442));
        assert_eq!(n.as_i64(), Some(442));
        assert_eq!(n.as_f32(), Some(442.0));
        assert_eq!(n.as_f64(), Some(442.0));
    }

    #[test]
    fn cmp_number() {
        assert!(CFNumber::new_i32(2) < CFNumber::new_i32(3));
        assert!(CFNumber::new_i32(3) == CFNumber::new_i32(3));
        assert!(CFNumber::new_i32(4) > CFNumber::new_i32(3));
    }
}
