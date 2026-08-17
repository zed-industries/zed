#![deny(deprecated, unreachable_code)]
use core::ptr::{self, NonNull};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{define_class, extern_methods, sel, ClassType, MainThreadOnly};
use static_assertions::{assert_impl_all, assert_not_impl_any};

// Test that adding the `deprecated` attribute does not mean that warnings
// when using the method internally are output.
#[test]
fn allow_deprecated() {
    #![deny(deprecated)]

    // Test allow propagates to impls
    define_class!(
        #[unsafe(super(NSObject))]
        #[deprecated]
        #[allow(deprecated)]
        struct AllowDeprecated;

        #[expect(deprecated)]
        impl AllowDeprecated {
            #[unsafe(method(someMethod))]
            fn some_method() {}
        }
    );

    #[expect(deprecated)]
    let _ = AllowDeprecated::class();
}

define_class!(
    #[unsafe(super(NSObject))]
    struct DefineClassDeprecatedMethod;

    #[deprecated]
    impl DefineClassDeprecatedMethod {
        #[unsafe(method(deprecatedOnImpl))]
        fn deprecated_on_impl() {}
    }

    impl DefineClassDeprecatedMethod {
        #[deprecated]
        #[unsafe(method(deprecatedOnMethod))]
        fn deprecated_on_method() {}
    }
);

#[test]
fn test_deprecated() {
    let _cls = DefineClassDeprecatedMethod::class();
}

#[test]
fn cfg() {
    // Test `cfg`. We use `debug_assertions` here because it's something that we
    // know our CI already tests.

    define_class!(
        #[unsafe(super(NSObject))]
        #[cfg(debug_assertions)]
        struct OnlyOnDebugAssertions;
    );

    #[cfg(debug_assertions)]
    let _ = OnlyOnDebugAssertions::class();

    define_class!(
        #[unsafe(super(NSObject))]
        #[cfg(not(debug_assertions))]
        struct NeverOnDebugAssertions;
    );

    #[cfg(not(debug_assertions))]
    let _ = NeverOnDebugAssertions::class();
}

// Test that `cfg` in methods.
define_class!(
    #[unsafe(super(NSObject))]
    struct DefineClassCfg;

    impl DefineClassCfg {
        #[cfg(debug_assertions)]
        #[unsafe(method(changesOnCfg1))]
        fn _changes_on_cfg1() -> i32 {
            1
        }

        #[cfg(not(debug_assertions))]
        #[unsafe(method(changesOnCfg1))]
        fn _changes_on_cfg1() -> i32 {
            2
        }

        #[cfg(debug_assertions)]
        #[unsafe(method(onlyWhenEnabled1))]
        fn _only_when_enabled1(&self) {}

        #[cfg(not(debug_assertions))]
        #[unsafe(method(onlyWhenDisabled1))]
        fn _only_when_disabled1(&self) {}
    }

    #[cfg(debug_assertions)]
    impl DefineClassCfg {
        #[unsafe(method(changesOnCfg2))]
        fn _changes_on_cfg2(&self) -> i32 {
            1
        }

        #[unsafe(method(onlyWhenEnabled2))]
        fn _only_when_enabled2() {}
    }

    #[cfg(not(debug_assertions))]
    impl DefineClassCfg {
        #[unsafe(method(changesOnCfg2))]
        fn _changes_on_cfg2(&self) -> i32 {
            2
        }

        #[unsafe(method(onlyWhenDisabled2))]
        fn _only_when_disabled2() {}
    }

    #[cfg(debug_assertions)]
    impl DefineClassCfg {
        #[cfg(not(debug_assertions))]
        #[unsafe(method(never))]
        fn _never(&self) {}

        #[cfg(not(debug_assertions))]
        #[unsafe(method(never))]
        fn _never_class() {}
    }
);

impl DefineClassCfg {
    extern_methods!(
        #[unsafe(method(new))]
        fn new() -> Retained<Self>;
    );

    extern_methods!(
        #[unsafe(method(changesOnCfg1))]
        fn changes_on_cfg1() -> i32;

        #[unsafe(method(changesOnCfg2))]
        fn changes_on_cfg2(&self) -> i32;

        #[cfg(debug_assertions)]
        #[unsafe(method(onlyWhenEnabled1))]
        fn only_when_enabled1(&self);

        #[cfg(not(debug_assertions))]
        #[unsafe(method(onlyWhenDisabled1))]
        fn only_when_disabled1(&self);
    );
}

#[cfg(debug_assertions)]
impl DefineClassCfg {
    extern_methods!(
        #[unsafe(method(onlyWhenEnabled2))]
        fn only_when_enabled2();
    );
}

#[cfg(not(debug_assertions))]
impl DefineClassCfg {
    extern_methods!(
        #[unsafe(method(onlyWhenDisabled2))]
        fn only_when_disabled2();
    );
}

#[test]
fn test_method_that_changes_based_on_cfg() {
    let expected = if cfg!(debug_assertions) { 1 } else { 2 };
    let actual = DefineClassCfg::changes_on_cfg1();
    assert_eq!(expected, actual, "changes_on_cfg1");

    let actual = DefineClassCfg::new().changes_on_cfg2();
    assert_eq!(expected, actual, "changes_on_cfg2");
}

#[test]
fn test_method_that_is_only_available_based_on_cfg() {
    let cls = DefineClassCfg::class();
    let metacls = cls.metaclass();
    let obj = DefineClassCfg::new();

    #[cfg(debug_assertions)]
    {
        assert!(!cls.responds_to(sel!(onlyWhenDisabled1)));
        assert!(!metacls.responds_to(sel!(onlyWhenDisabled2)));

        obj.only_when_enabled1();
        DefineClassCfg::only_when_enabled2();
    }
    #[cfg(not(debug_assertions))]
    {
        assert!(!cls.responds_to(sel!(onlyWhenEnabled1)));
        assert!(!metacls.responds_to(sel!(onlyWhenEnabled2)));

        obj.only_when_disabled1();
        DefineClassCfg::only_when_disabled2();
    }
}

#[test]
fn test_method_that_is_never_available() {
    let cls = DefineClassCfg::class();
    let metacls = cls.metaclass();
    assert!(!cls.responds_to(sel!(never)));
    assert!(!metacls.responds_to(sel!(never)));
}

define_class!(
    #[unsafe(super(NSObject))]
    struct TestMultipleColonSelector;

    impl TestMultipleColonSelector {
        #[unsafe(method(test::arg3:))]
        fn _test_class(arg1: i32, arg2: i32, arg3: i32) -> i32 {
            arg1 + arg2 + arg3
        }

        #[unsafe(method(test::arg3:))]
        fn _test_instance(&self, arg1: i32, arg2: i32, arg3: i32) -> i32 {
            arg1 * arg2 * arg3
        }

        #[unsafe(method(test::error:))]
        fn _test_error(&self, _arg1: i32, _arg2: i32, _arg3: *mut *mut NSObject) -> bool {
            true
        }

        #[unsafe(method_id(test:::withObject:))]
        fn _test_object(
            &self,
            _arg1: i32,
            _arg2: i32,
            _arg3: i32,
            _obj: *const Self,
        ) -> Option<Retained<Self>> {
            None
        }
    }
);

impl TestMultipleColonSelector {
    extern_methods!(
        #[unsafe(method(new))]
        fn new() -> Retained<Self>;

        #[unsafe(method(test::arg3:))]
        fn test_class(arg1: i32, arg2: i32, arg3: i32) -> i32;

        #[unsafe(method(test::arg3:))]
        fn test_instance(&self, arg1: i32, arg2: i32, arg3: i32) -> i32;

        #[unsafe(method(test::error:_))]
        fn test_error(&self, arg1: i32, arg2: i32) -> Result<(), Retained<NSObject>>;

        #[unsafe(method(test:::withObject:))]
        fn test_object(
            &self,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            obj: *const Self,
        ) -> Option<Retained<Self>>;
    );
}

#[test]
fn test_multiple_colon_selector() {
    assert_eq!(TestMultipleColonSelector::test_class(2, 3, 4), 9);

    let obj = TestMultipleColonSelector::new();
    assert_eq!(obj.test_instance(1, 2, 3), 6);
    assert!(obj.test_error(1, 2).is_ok());
    assert!(obj.test_object(1, 2, 3, ptr::null()).is_none());
}

define_class!(
    #[unsafe(super(NSObject))]
    struct DefineClassAllTheBool;

    impl DefineClassAllTheBool {
        #[unsafe(method(returnsBool))]
        fn returns_bool() -> bool {
            true
        }

        #[unsafe(method(returnsBoolInstance))]
        fn returns_bool_instance(&self) -> bool {
            true
        }

        #[unsafe(method(takesBool:andMut:andUnderscore:))]
        fn takes_bool(a: bool, mut b: bool, _: bool) -> bool {
            if b {
                b = a;
            }
            b
        }

        #[unsafe(method(takesBoolInstance:andMut:andUnderscore:))]
        fn takes_bool_instance(&self, a: bool, mut b: bool, _: bool) -> bool {
            if b {
                b = a;
            }
            b
        }

        #[unsafe(method(takesReturnsBool:))]
        fn takes_returns_bool(b: bool) -> bool {
            b
        }

        #[unsafe(method(takesReturnsBoolInstance:))]
        fn takes_returns_bool_instance(&self, b: bool) -> bool {
            b
        }

        #[unsafe(method_id(idTakesBool:))]
        fn id_takes_bool(_b: bool) -> Option<Retained<Self>> {
            None
        }

        #[unsafe(method_id(idTakesBoolInstance:))]
        fn id_takes_bool_instance(&self, _b: bool) -> Option<Retained<Self>> {
            None
        }
    }
);

#[test]
fn test_all_the_bool() {
    let _ = DefineClassAllTheBool::class();
}

define_class!(
    #[unsafe(super(NSObject))]
    struct DefineClassUnreachable;

    // Ensure none of these warn
    impl DefineClassUnreachable {
        #[unsafe(method(unreachable))]
        fn unreachable(&self) -> bool {
            unreachable!()
        }

        #[unsafe(method(unreachableClass))]
        fn unreachable_class() -> bool {
            unreachable!()
        }

        #[unsafe(method(unreachableVoid))]
        fn unreachable_void(&self) {
            unreachable!()
        }

        #[unsafe(method(unreachableClassVoid))]
        fn unreachable_class_void() {
            unreachable!()
        }

        #[unsafe(method_id(unreachableRetained))]
        fn unreachable_retained(&self) -> Retained<Self> {
            unreachable!()
        }

        #[unsafe(method_id(unreachableClassRetained))]
        fn unreachable_class_retained() -> Retained<Self> {
            unreachable!()
        }
    }
);

#[test]
fn test_unreachable() {
    let _ = DefineClassUnreachable::class();
}

define_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug)]
    struct OutParam;

    impl OutParam {
        #[unsafe(method(unsupported1:))]
        fn _unsupported1(_param: &mut Retained<Self>) {}

        #[unsafe(method(unsupported2:))]
        fn _unsupported2(_param: Option<&mut Retained<Self>>) {}

        #[unsafe(method(unsupported3:))]
        fn _unsupported3(_param: &mut Option<Retained<Self>>) {}

        #[unsafe(method(unsupported4:))]
        fn _unsupported4(_param: Option<&mut Option<Retained<Self>>>) {}
    }
);

impl OutParam {
    extern_methods!(
        #[unsafe(method(new))]
        fn new() -> Retained<Self>;

        #[unsafe(method(unsupported1:))]
        fn unsupported1(_param: &mut Retained<Self>);

        #[unsafe(method(unsupported2:))]
        fn unsupported2(_param: Option<&mut Retained<Self>>);

        #[unsafe(method(unsupported3:))]
        fn unsupported3(_param: &mut Option<Retained<Self>>);

        #[unsafe(method(unsupported4:))]
        fn unsupported4(_param: Option<&mut Option<Retained<Self>>>);
    );
}

#[test]
#[should_panic = "`&mut Retained<_>` is not supported in `define_class!` yet"]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "unwinding seems to not work properly here"
)]
fn out_param1() {
    let mut param = OutParam::new();
    OutParam::unsupported1(&mut param);
}

#[test]
#[should_panic = "`Option<&mut Retained<_>>` is not supported in `define_class!` yet"]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "unwinding seems to not work properly here"
)]
fn out_param2() {
    OutParam::unsupported2(None);
}

#[test]
#[should_panic = "`&mut Option<Retained<_>>` is not supported in `define_class!` yet"]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "unwinding seems to not work properly here"
)]
fn out_param3() {
    let mut param = Some(OutParam::new());
    OutParam::unsupported3(&mut param);
}

#[test]
#[should_panic = "`Option<&mut Option<Retained<_>>>` is not supported in `define_class!` yet"]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "unwinding seems to not work properly here"
)]
fn out_param4() {
    OutParam::unsupported4(None);
}

#[test]
fn test_pointer_receiver_allowed() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[derive(Debug)]
        struct PointerReceiver;

        impl PointerReceiver {
            #[unsafe(method(constPtr))]
            fn const_ptr(_this: *const Self) {}

            #[unsafe(method(mutPtr))]
            fn mut_ptr(_this: *mut Self) {}

            #[unsafe(method(nonnullPtr))]
            fn nonnull_ptr(_this: NonNull<Self>) {}
        }
    );

    let _ = PointerReceiver::class();
}

#[test]
fn test_auto_traits() {
    struct NotSend(PhantomData<*mut usize>);
    unsafe impl Sync for NotSend {}
    assert_impl_all!(NotSend: Sync, UnwindSafe, RefUnwindSafe, Unpin);
    assert_not_impl_any!(NotSend: Send);

    struct NotSync(PhantomData<*mut usize>);
    unsafe impl Send for NotSync {}
    assert_impl_all!(NotSync: Send, UnwindSafe, RefUnwindSafe, Unpin);
    assert_not_impl_any!(NotSync: Sync);

    struct NotUnwindSafe(PhantomData<*mut UnsafeCell<usize>>);
    unsafe impl Send for NotUnwindSafe {}
    unsafe impl Sync for NotUnwindSafe {}
    assert_impl_all!(NotUnwindSafe: Send, Sync, Unpin);
    assert_not_impl_any!(NotUnwindSafe: UnwindSafe, RefUnwindSafe);

    // Superclass propagates.

    define_class!(
        #[unsafe(super(NSObject))]
        #[ivars = (NotSend, NotSync, NotUnwindSafe)]
        struct NonThreadSafeHelper;
    );
    define_class!(
        #[unsafe(super(NonThreadSafeHelper))]
        #[ivars = ()]
        struct InheritsCustomWithNonSendIvar;
    );
    let _ = InheritsCustomWithNonSendIvar::class();
    assert_not_impl_any!(InheritsCustomWithNonSendIvar: Unpin, Send, Sync, UnwindSafe, RefUnwindSafe);

    // Main thread only. Not Send + Sync.

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[ivars = ()]
        struct InheritsNSObjectMainThreadOnly;
    );
    let _ = InheritsNSObjectMainThreadOnly::class();
    assert_impl_all!(InheritsNSObjectMainThreadOnly: UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(InheritsNSObjectMainThreadOnly: Unpin, Send, Sync);

    // NSObject is special.

    define_class!(
        #[unsafe(super(NSObject))]
        #[ivars = ()]
        struct InheritsNSObject;
    );
    let _ = InheritsNSObject::class();
    assert_impl_all!(InheritsNSObject: Send, Sync, UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(InheritsNSObject: Unpin);

    define_class!(
        #[unsafe(super(NSObject))]
        #[ivars = NotSend]
        struct InheritsNSObjectWithNonSendIvar;
    );
    let _ = InheritsNSObjectWithNonSendIvar::class();
    assert_impl_all!(InheritsNSObjectWithNonSendIvar: Sync, UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(InheritsNSObjectWithNonSendIvar: Unpin, Send);

    define_class!(
        #[unsafe(super(NSObject))]
        #[ivars = NotSync]
        struct InheritsNSObjectWithNonSyncIvar;
    );
    let _ = InheritsNSObjectWithNonSyncIvar::class();
    assert_impl_all!(InheritsNSObjectWithNonSyncIvar: Send, UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(InheritsNSObjectWithNonSyncIvar: Unpin, Sync);

    define_class!(
        #[unsafe(super(NSObject))]
        #[ivars = NotUnwindSafe]
        struct InheritsNSObjectWithNonUnwindSafeIvar;
    );
    let _ = InheritsNSObjectWithNonUnwindSafeIvar::class();
    assert_impl_all!(InheritsNSObjectWithNonUnwindSafeIvar: Send, Sync);
    assert_not_impl_any!(InheritsNSObjectWithNonUnwindSafeIvar: Unpin, UnwindSafe, RefUnwindSafe);
}

#[test]
fn auto_name() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[ivars = ()]
        struct AutoName;
    );

    let expected = format!("define_class::AutoName{}", env!("CARGO_PKG_VERSION"));

    assert_eq!(AutoName::class().name().to_str().unwrap(), expected);
    assert_eq!(AutoName::NAME, expected);
}

#[test]
fn set_name() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "SetName"]
        struct SetName;
    );

    assert_eq!(SetName::class().name().to_str().unwrap(), "SetName");
    assert_eq!(SetName::NAME, "SetName");
}

#[test]
fn name_can_be_expr() {
    const NAME: &str = "  NameCanBeExpr  ";

    define_class!(
        #[unsafe(super(NSObject))]
        #[name = NAME.trim_ascii()]
        struct NameCanBeExpr;
    );

    let expected = "NameCanBeExpr";
    assert_eq!(NameCanBeExpr::class().name().to_str().unwrap(), expected);
    assert_eq!(NameCanBeExpr::NAME, expected);
}
