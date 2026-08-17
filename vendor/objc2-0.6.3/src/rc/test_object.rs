//! Note: This file is included in the `tests` crate viaa symlink as well.
use core::cell::RefCell;
use core::ptr;

use crate::rc::{Allocated, DefaultRetained, Retained};
use crate::runtime::{NSObject, NSObjectProtocol, NSZone};
use crate::{define_class, msg_send, ClassType};

// TODO: Put tests that use this in another crate
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
#[doc(hidden)]
pub(crate) struct ThreadTestData {
    pub(crate) alloc: usize,
    pub(crate) drop: usize,
    pub(crate) init: usize,
    pub(crate) retain: usize,
    pub(crate) copy: usize,
    pub(crate) mutable_copy: usize,
    pub(crate) release: usize,
    pub(crate) autorelease: usize,
    pub(crate) try_retain: usize,
    pub(crate) try_retain_fail: usize,
    // TODO: Is there some way we can test weak pointers? Or is that implemented entirely in Foundation?
    // Maybe `_setWeaklyReferenced` can be useful?
}

impl ThreadTestData {
    /// Get the amount of method calls performed on the current thread.
    pub(crate) fn current() -> Self {
        TEST_DATA.with(|data| data.borrow().clone())
    }

    #[track_caller]
    #[allow(clippy::missing_panics_doc)]
    #[allow(dead_code)]
    pub(crate) fn assert_current(&self) {
        let current = Self::current();
        let mut expected = self.clone();
        if cfg!(feature = "gnustep-1-7") {
            // GNUStep doesn't have `tryRetain`, it uses `retain` directly
            let retain_diff = expected.try_retain - current.try_retain;
            expected.retain += retain_diff;
            expected.try_retain -= retain_diff;

            // GNUStep doesn't call `autorelease` if it's overridden
            expected.autorelease = 0;
        }
        if current != expected {
            panic!(
                "got differing amounts of calls:
   current: `{current:?}`,
  expected: `{expected:?}`"
            )
        }
    }
}

std::thread_local! {
    static TEST_DATA: RefCell<ThreadTestData> = RefCell::default();
}

define_class!(
    /// A helper object that counts how many times various reference-counting
    /// primitives are called.
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[doc(hidden)]
    #[unsafe(super(NSObject))]
    #[name = "__RcTestObject"]
    pub(crate) struct RcTestObject;

    impl RcTestObject {
        #[unsafe(method_id(newReturningNull))]
        fn new_returning_null() -> Option<Retained<Self>> {
            None
        }

        #[unsafe(method_id(newMethodOnInstance))]
        fn new_method_on_instance(&self) -> Retained<Self> {
            Self::new()
        }

        #[unsafe(method_id(newMethodOnInstanceNull))]
        fn new_method_on_instance_null(&self) -> Option<Retained<Self>> {
            None
        }

        #[unsafe(method(alloc))]
        fn alloc_() -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().alloc += 1);
            let superclass = NSObject::class().metaclass();
            let zone: *const NSZone = ptr::null();
            unsafe { msg_send![super(Self::class(), superclass), allocWithZone: zone] }
        }

        #[unsafe(method(allocWithZone:))]
        fn alloc_with_zone(zone: *const NSZone) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().alloc += 1);
            let superclass = NSObject::class().metaclass();
            unsafe { msg_send![super(Self::class(), superclass), allocWithZone: zone] }
        }

        #[unsafe(method(allocReturningNull))]
        fn alloc_returning_null() -> *mut Self {
            ptr::null_mut()
        }

        #[unsafe(method_id(init))]
        unsafe fn init(this: Allocated<Self>) -> Retained<Self> {
            TEST_DATA.with(|data| data.borrow_mut().init += 1);
            let this = this.set_ivars(());
            unsafe { msg_send![super(this), init] }
        }

        #[unsafe(method_id(initReturningNull))]
        fn init_returning_null(_this: Allocated<Self>) -> Option<Retained<Self>> {
            None
        }

        #[unsafe(method(retain))]
        fn retain(&self) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().retain += 1);
            unsafe { msg_send![super(self), retain] }
        }

        #[unsafe(method(release))]
        fn release(&self) {
            TEST_DATA.with(|data| data.borrow_mut().release += 1);
            unsafe { msg_send![super(self), release] }
        }

        #[unsafe(method(autorelease))]
        fn autorelease(&self) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().autorelease += 1);
            unsafe { msg_send![super(self), autorelease] }
        }

        #[unsafe(method(_tryRetain))]
        unsafe fn try_retain(&self) -> bool {
            TEST_DATA.with(|data| data.borrow_mut().try_retain += 1);
            let res: bool = unsafe { msg_send![super(self), _tryRetain] };
            if !res {
                TEST_DATA.with(|data| data.borrow_mut().try_retain -= 1);
                TEST_DATA.with(|data| data.borrow_mut().try_retain_fail += 1);
            }
            res
        }

        #[unsafe(method_id(copyWithZone:))]
        fn copy_with_zone(&self, _zone: *const NSZone) -> Retained<Self> {
            TEST_DATA.with(|data| data.borrow_mut().copy += 1);
            Self::new()
        }

        #[unsafe(method_id(mutableCopyWithZone:))]
        fn mutable_copy_with_zone(&self, _zone: *const NSZone) -> Retained<Self> {
            TEST_DATA.with(|data| data.borrow_mut().mutable_copy += 1);
            Self::new()
        }

        #[unsafe(method_id(copyReturningNull))]
        fn copy_returning_null(_this: &Self) -> Option<Retained<Self>> {
            None
        }

        #[unsafe(method_id(methodReturningNull))]
        fn method_returning_null(self: &Self) -> Option<Retained<Self>> {
            None
        }

        #[unsafe(method_id(aMethod:))]
        fn a_method(&self, param: bool) -> Option<Retained<Self>> {
            param.then(Self::new)
        }

        #[unsafe(method(boolAndShouldError:error:))]
        fn class_error_bool(should_error: bool, err: Option<&mut *mut RcTestObject>) -> bool {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                false
            } else {
                true
            }
        }

        #[unsafe(method(boolAndShouldError:error:))]
        fn instance_error_bool(
            &self,
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> bool {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                false
            } else {
                true
            }
        }

        #[unsafe(method_id(idAndShouldError:error:))]
        fn class_error_retained(
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> Option<Retained<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                None
            } else {
                Some(Self::new())
            }
        }

        #[unsafe(method_id(idAndShouldError:error:))]
        fn instance_error_retained(
            self: &Self,
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> Option<Retained<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                None
            } else {
                Some(Self::new())
            }
        }

        #[unsafe(method_id(newAndShouldError:error:))]
        fn new_error(
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> Option<Retained<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                None
            } else {
                unsafe { msg_send![Self::class(), new] }
            }
        }

        #[unsafe(method(allocAndShouldError:error:))]
        fn alloc_error(should_error: bool, err: Option<&mut *mut RcTestObject>) -> *mut Self {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                ptr::null_mut()
            } else {
                unsafe { msg_send![Self::class(), alloc] }
            }
        }

        #[unsafe(method_id(initAndShouldError:error:))]
        fn init_error(
            this: Allocated<Self>,
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> Option<Retained<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Retained::autorelease_ptr(RcTestObject::new());
                }
                None
            } else {
                unsafe { msg_send![this, init] }
            }
        }

        #[unsafe(method(outParamNull:))]
        fn out_param_null(param: Option<&mut *mut RcTestObject>) {
            if let Some(param) = param {
                *param = ptr::null_mut();
            }
        }

        #[unsafe(method(willPanicWith:panicsAfter:))]
        fn will_panic(param: Option<&mut *mut RcTestObject>, panic_after: bool) {
            if !panic_after {
                panic!("intentional panic");
            }
            if let Some(param) = param {
                *param = Retained::autorelease_ptr(RcTestObject::new());
            }
            if panic_after {
                panic!("intentional panic");
            }
        }
    }

    unsafe impl NSObjectProtocol for RcTestObject {}
);

impl Drop for RcTestObject {
    fn drop(&mut self) {
        TEST_DATA.with(|data| data.borrow_mut().drop += 1);
    }
}

impl DefaultRetained for RcTestObject {
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

impl RcTestObject {
    #[doc(hidden)]
    pub(crate) fn new() -> Retained<Self> {
        // Use raw `msg_send!`, the automatic conversion are tested elsewhere.
        unsafe { Retained::from_raw(msg_send![Self::class(), new]) }.unwrap()
    }
}
