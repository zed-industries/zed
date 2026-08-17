#![deny(deprecated, unreachable_code)]
use core::ptr::{self, NonNull};

use objc2::mutability::Immutable;
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{declare_class, extern_methods, sel, ClassType, DeclaredClass};

// Test that adding the `deprecated` attribute does not mean that warnings
// when using the method internally are output.
declare_class!(
    struct DeclareClassDepreactedMethod;

    unsafe impl ClassType for DeclareClassDepreactedMethod {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "DeclareClassDepreactedMethod";
    }

    impl DeclaredClass for DeclareClassDepreactedMethod {}

    #[deprecated]
    unsafe impl DeclareClassDepreactedMethod {
        #[method(deprecatedOnImpl)]
        fn deprecated_on_impl() {}
    }

    unsafe impl DeclareClassDepreactedMethod {
        #[deprecated]
        #[method(deprecatedOnMethod)]
        fn deprecated_on_method() {}
    }
);

#[test]
fn test_deprecated() {
    let _cls = DeclareClassDepreactedMethod::class();
}

// Test that `cfg` works properly.
//
// We use `debug_assertions` here because it's something that we know our CI
// already tests.
declare_class!(
    struct DeclareClassCfg;

    unsafe impl ClassType for DeclareClassCfg {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "DeclareClassCfg";
    }

    impl DeclaredClass for DeclareClassCfg {}

    unsafe impl DeclareClassCfg {
        #[cfg(debug_assertions)]
        #[method(changesOnCfg1)]
        fn _changes_on_cfg1() -> i32 {
            1
        }

        #[cfg(not(debug_assertions))]
        #[method(changesOnCfg1)]
        fn _changes_on_cfg1() -> i32 {
            2
        }

        #[cfg(debug_assertions)]
        #[method(onlyWhenEnabled1)]
        fn _only_when_enabled1(&self) {}

        #[cfg(not(debug_assertions))]
        #[method(onlyWhenDisabled1)]
        fn _only_when_disabled1(&self) {}
    }

    #[cfg(debug_assertions)]
    unsafe impl DeclareClassCfg {
        #[method(changesOnCfg2)]
        fn _changes_on_cfg2(&self) -> i32 {
            1
        }

        #[method(onlyWhenEnabled2)]
        fn _only_when_enabled2() {}
    }

    #[cfg(not(debug_assertions))]
    unsafe impl DeclareClassCfg {
        #[method(changesOnCfg2)]
        fn _changes_on_cfg2(&self) -> i32 {
            2
        }

        #[method(onlyWhenDisabled2)]
        fn _only_when_disabled2() {}
    }

    #[cfg(debug_assertions)]
    unsafe impl DeclareClassCfg {
        #[cfg(not(debug_assertions))]
        #[method(never)]
        fn _never(&self) {}

        #[cfg(not(debug_assertions))]
        #[method(never)]
        fn _never_class() {}
    }
);

extern_methods!(
    unsafe impl DeclareClassCfg {
        #[method_id(new)]
        fn new() -> Retained<Self>;
    }

    unsafe impl DeclareClassCfg {
        #[method(changesOnCfg1)]
        fn changes_on_cfg1() -> i32;

        #[method(changesOnCfg2)]
        fn changes_on_cfg2(&self) -> i32;

        #[cfg(debug_assertions)]
        #[method(onlyWhenEnabled1)]
        fn only_when_enabled1(&self);

        #[cfg(not(debug_assertions))]
        #[method(onlyWhenDisabled1)]
        fn only_when_disabled1(&self);
    }

    #[cfg(debug_assertions)]
    unsafe impl DeclareClassCfg {
        #[method(onlyWhenEnabled2)]
        fn only_when_enabled2();
    }

    #[cfg(not(debug_assertions))]
    unsafe impl DeclareClassCfg {
        #[method(onlyWhenDisabled2)]
        fn only_when_disabled2();
    }
);

#[test]
fn test_method_that_changes_based_on_cfg() {
    let expected = if cfg!(debug_assertions) { 1 } else { 2 };
    let actual = DeclareClassCfg::changes_on_cfg1();
    assert_eq!(expected, actual, "changes_on_cfg1");

    let actual = DeclareClassCfg::new().changes_on_cfg2();
    assert_eq!(expected, actual, "changes_on_cfg2");
}

#[test]
fn test_method_that_is_only_available_based_on_cfg() {
    let cls = DeclareClassCfg::class();
    let metacls = cls.metaclass();
    let obj = DeclareClassCfg::new();

    #[cfg(debug_assertions)]
    {
        assert!(!cls.responds_to(sel!(onlyWhenDisabled1)));
        assert!(!metacls.responds_to(sel!(onlyWhenDisabled2)));

        obj.only_when_enabled1();
        DeclareClassCfg::only_when_enabled2();
    }
    #[cfg(not(debug_assertions))]
    {
        assert!(!cls.responds_to(sel!(onlyWhenEnabled1)));
        assert!(!metacls.responds_to(sel!(onlyWhenEnabled2)));

        obj.only_when_disabled1();
        DeclareClassCfg::only_when_disabled2();
    }
}

#[test]
fn test_method_that_is_never_available() {
    let cls = DeclareClassCfg::class();
    let metacls = cls.metaclass();
    assert!(!cls.responds_to(sel!(never)));
    assert!(!metacls.responds_to(sel!(never)));
}

declare_class!(
    struct TestMultipleColonSelector;

    unsafe impl ClassType for TestMultipleColonSelector {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "TestMultipleColonSelector";
    }

    impl DeclaredClass for TestMultipleColonSelector {}

    unsafe impl TestMultipleColonSelector {
        #[method(test::arg3:)]
        fn _test_class(arg1: i32, arg2: i32, arg3: i32) -> i32 {
            arg1 + arg2 + arg3
        }

        #[method(test::arg3:)]
        fn _test_instance(&self, arg1: i32, arg2: i32, arg3: i32) -> i32 {
            arg1 * arg2 * arg3
        }

        #[method(test::error:)]
        fn _test_error(&self, _arg1: i32, _arg2: i32, _arg3: *mut *mut NSObject) -> bool {
            true
        }

        #[method_id(test:::withObject:)]
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

extern_methods!(
    unsafe impl TestMultipleColonSelector {
        #[method_id(new)]
        fn new() -> Retained<Self>;

        #[method(test::arg3:)]
        fn test_class(arg1: i32, arg2: i32, arg3: i32) -> i32;

        #[method(test::arg3:)]
        fn test_instance(&self, arg1: i32, arg2: i32, arg3: i32) -> i32;

        #[method(test::error:_)]
        fn test_error(&self, arg1: i32, arg2: i32) -> Result<(), Retained<NSObject>>;

        #[method_id(test:::withObject:)]
        fn test_object(
            &self,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            obj: *const Self,
        ) -> Option<Retained<Self>>;
    }
);

#[test]
fn test_multiple_colon_selector() {
    assert_eq!(TestMultipleColonSelector::test_class(2, 3, 4), 9);

    let obj = TestMultipleColonSelector::new();
    assert_eq!(obj.test_instance(1, 2, 3), 6);
    assert!(obj.test_error(1, 2).is_ok());
    assert!(obj.test_object(1, 2, 3, ptr::null()).is_none());
}

declare_class!(
    struct DeclareClassAllTheBool;

    unsafe impl ClassType for DeclareClassAllTheBool {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "DeclareClassAllTheBool";
    }

    impl DeclaredClass for DeclareClassAllTheBool {}

    unsafe impl DeclareClassAllTheBool {
        #[method(returnsBool)]
        fn returns_bool() -> bool {
            true
        }

        #[method(returnsBoolInstance)]
        fn returns_bool_instance(&self) -> bool {
            true
        }

        #[method(takesBool:andMut:andUnderscore:)]
        fn takes_bool(a: bool, mut b: bool, _: bool) -> bool {
            if b {
                b = a;
            }
            b
        }

        #[method(takesBoolInstance:andMut:andUnderscore:)]
        fn takes_bool_instance(&self, a: bool, mut b: bool, _: bool) -> bool {
            if b {
                b = a;
            }
            b
        }

        #[method(takesReturnsBool:)]
        fn takes_returns_bool(b: bool) -> bool {
            b
        }

        #[method(takesReturnsBoolInstance:)]
        fn takes_returns_bool_instance(&self, b: bool) -> bool {
            b
        }

        #[method_id(idTakesBool:)]
        fn id_takes_bool(_b: bool) -> Option<Retained<Self>> {
            None
        }

        #[method_id(idTakesBoolInstance:)]
        fn id_takes_bool_instance(&self, _b: bool) -> Option<Retained<Self>> {
            None
        }
    }
);

#[test]
fn test_all_the_bool() {
    let _ = DeclareClassAllTheBool::class();
}

declare_class!(
    struct DeclareClassUnreachable;

    unsafe impl ClassType for DeclareClassUnreachable {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "DeclareClassUnreachable";
    }

    impl DeclaredClass for DeclareClassUnreachable {}

    // Ensure none of these warn
    unsafe impl DeclareClassUnreachable {
        #[method(unreachable)]
        fn unreachable(&self) -> bool {
            unreachable!()
        }

        #[method(unreachableClass)]
        fn unreachable_class() -> bool {
            unreachable!()
        }

        #[method(unreachableVoid)]
        fn unreachable_void(&self) {
            unreachable!()
        }

        #[method(unreachableClassVoid)]
        fn unreachable_class_void() {
            unreachable!()
        }

        #[method_id(unreachableId)]
        fn unreachable_id(&self) -> Retained<Self> {
            unreachable!()
        }

        #[method_id(unreachableClassId)]
        fn unreachable_class_id() -> Retained<Self> {
            unreachable!()
        }
    }
);

#[test]
fn test_unreachable() {
    let _ = DeclareClassUnreachable::class();
}

declare_class!(
    #[derive(Debug)]
    struct OutParam;

    unsafe impl ClassType for OutParam {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "OutParam";
    }

    impl DeclaredClass for OutParam {}

    unsafe impl OutParam {
        #[method(unsupported1:)]
        fn _unsupported1(_param: &mut Retained<Self>) {}

        #[method(unsupported2:)]
        fn _unsupported2(_param: Option<&mut Retained<Self>>) {}

        #[method(unsupported3:)]
        fn _unsupported3(_param: &mut Option<Retained<Self>>) {}

        #[method(unsupported4:)]
        fn _unsupported4(_param: Option<&mut Option<Retained<Self>>>) {}
    }
);

extern_methods!(
    unsafe impl OutParam {
        #[method_id(new)]
        fn new() -> Retained<Self>;

        #[method(unsupported1:)]
        fn unsupported1(_param: &mut Retained<Self>);

        #[method(unsupported2:)]
        fn unsupported2(_param: Option<&mut Retained<Self>>);

        #[method(unsupported3:)]
        fn unsupported3(_param: &mut Option<Retained<Self>>);

        #[method(unsupported4:)]
        fn unsupported4(_param: Option<&mut Option<Retained<Self>>>);
    }
);

#[test]
#[should_panic = "`&mut Retained<_>` is not supported in `declare_class!` yet"]
#[cfg_attr(
    not(all(target_pointer_width = "64", not(feature = "catch-all"))),
    ignore = "unwinds through FFI boundary"
)]
fn out_param1() {
    let mut param = OutParam::new();
    OutParam::unsupported1(&mut param);
}

#[test]
#[should_panic = "`Option<&mut Retained<_>>` is not supported in `declare_class!` yet"]
#[cfg_attr(
    not(all(target_pointer_width = "64", not(feature = "catch-all"))),
    ignore = "unwinds through FFI boundary"
)]
fn out_param2() {
    OutParam::unsupported2(None);
}

#[test]
#[should_panic = "`&mut Option<Retained<_>>` is not supported in `declare_class!` yet"]
#[cfg_attr(
    not(all(target_pointer_width = "64", not(feature = "catch-all"))),
    ignore = "unwinds through FFI boundary"
)]
fn out_param3() {
    let mut param = Some(OutParam::new());
    OutParam::unsupported3(&mut param);
}

#[test]
#[should_panic = "`Option<&mut Option<Retained<_>>>` is not supported in `declare_class!` yet"]
#[cfg_attr(
    not(all(target_pointer_width = "64", not(feature = "catch-all"))),
    ignore = "unwinds through FFI boundary"
)]
fn out_param4() {
    OutParam::unsupported4(None);
}

#[test]
fn test_pointer_receiver_allowed() {
    declare_class!(
        #[derive(Debug)]
        struct PointerReceiver;

        unsafe impl ClassType for PointerReceiver {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "PointerReceiver";
        }

        impl DeclaredClass for PointerReceiver {}

        unsafe impl PointerReceiver {
            #[method(constPtr)]
            fn const_ptr(_this: *const Self) {}

            #[method(mutPtr)]
            fn mut_ptr(_this: *mut Self) {}

            #[method(nonnullPtr)]
            fn nonnull_ptr(_this: NonNull<Self>) {}
        }
    );

    let _ = PointerReceiver::class();
}
