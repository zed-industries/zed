use core::ptr;

use alloc::rc::Weak as RcWeak;
use alloc::sync::Weak;

use crate::RefCnt;

unsafe impl<T> RefCnt for Weak<T> {
    type Base = T;
    fn as_ptr(me: &Self) -> *mut T {
        if Weak::ptr_eq(&Weak::new(), me) {
            ptr::null_mut()
        } else {
            Weak::as_ptr(me) as *mut T
        }
    }
    fn into_ptr(me: Self) -> *mut T {
        if Weak::ptr_eq(&Weak::new(), &me) {
            ptr::null_mut()
        } else {
            Weak::into_raw(me) as *mut T
        }
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        if ptr.is_null() {
            Weak::new()
        } else {
            Weak::from_raw(ptr)
        }
    }
}

unsafe impl<T> RefCnt for RcWeak<T> {
    type Base = T;
    fn as_ptr(me: &Self) -> *mut T {
        if RcWeak::ptr_eq(&RcWeak::new(), me) {
            ptr::null_mut()
        } else {
            RcWeak::as_ptr(me) as *mut T
        }
    }
    fn into_ptr(me: Self) -> *mut T {
        if RcWeak::ptr_eq(&RcWeak::new(), &me) {
            ptr::null_mut()
        } else {
            RcWeak::into_raw(me) as *mut T
        }
    }
    unsafe fn from_ptr(ptr: *const T) -> Self {
        if ptr.is_null() {
            RcWeak::new()
        } else {
            RcWeak::from_raw(ptr)
        }
    }
}

macro_rules! t {
    ($name: ident, $strategy: ty) => {
        #[cfg(test)]
        mod $name {
            use alloc::sync::{Arc, Weak};

            use crate::ArcSwapAny;

            #[allow(deprecated)] // We use "deprecated" testing strategies in here.
            type ArcSwapWeak<T> = ArcSwapAny<Weak<T>, $strategy>;

            // Convert to weak, push it through the shared and pull it out again.
            #[test]
            fn there_and_back() {
                let data = Arc::new("Hello");
                let shared = ArcSwapWeak::new(Arc::downgrade(&data));
                assert_eq!(1, Arc::strong_count(&data));
                assert_eq!(1, Arc::weak_count(&data));
                let weak = shared.load();
                assert_eq!("Hello", *weak.upgrade().unwrap());
                assert!(Arc::ptr_eq(&data, &weak.upgrade().unwrap()));
            }

            // Replace a weak pointer with a NULL one
            #[test]
            fn reset() {
                let data = Arc::new("Hello");
                let shared = ArcSwapWeak::new(Arc::downgrade(&data));
                assert_eq!(1, Arc::strong_count(&data));
                assert_eq!(1, Arc::weak_count(&data));

                // An empty weak (eg. NULL)
                shared.store(Weak::new());
                assert_eq!(1, Arc::strong_count(&data));
                assert_eq!(0, Arc::weak_count(&data));

                let weak = shared.load();
                assert!(weak.upgrade().is_none());
            }

            // Destroy the underlying data while the weak is still stored inside. Should make it go
            // NULL-ish
            #[test]
            fn destroy() {
                let data = Arc::new("Hello");
                let shared = ArcSwapWeak::new(Arc::downgrade(&data));

                drop(data);
                let weak = shared.load();
                assert!(weak.upgrade().is_none());
            }
        }
    };
}

t!(tests_default, crate::DefaultStrategy);
#[cfg(feature = "internal-test-strategies")]
t!(
    tests_full_slots,
    crate::strategy::test_strategies::FillFastSlots
);
