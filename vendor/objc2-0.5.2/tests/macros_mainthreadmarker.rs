use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{
    declare_class, extern_methods, extern_protocol, mutability, ClassType, DeclaredClass,
    ProtocolType,
};

extern_protocol!(
    #[allow(clippy::missing_safety_doc)]
    unsafe trait Proto: NSObjectProtocol {
        #[method(myMethod:)]
        fn protocol_method(mtm: MainThreadMarker, arg: i32) -> i32;

        #[method_id(myMethodId:)]
        fn protocol_method_id(mtm: MainThreadMarker, arg: &Self) -> Retained<Self>;
    }

    unsafe impl ProtocolType for dyn Proto {
        const NAME: &'static str = "MainThreadMarkerTestProtocol";
    }
);

declare_class!(
    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Cls;

    unsafe impl ClassType for Cls {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "MainThreadMarkerTest";
    }

    impl DeclaredClass for Cls {}

    unsafe impl Proto for Cls {
        #[method(myMethod:)]
        fn _my_mainthreadonly_method(arg: i32) -> i32 {
            arg + 1
        }

        #[method_id(myMethodId:)]
        fn _my_mainthreadonly_method_id(arg: &Self) -> Retained<Self> {
            unsafe { Retained::retain(arg as *const Self as *mut Self).unwrap() }
        }
    }
);

unsafe impl NSObjectProtocol for Cls {}

// The macro does a textual match; but when users actually use
// `objc2_foundation::MainThreadMarker` to ensure soundness, they will not
// do this!
#[derive(Clone, Copy)]
struct MainThreadMarker {
    _some_field: u32,
}

extern_methods!(
    unsafe impl Cls {
        #[method_id(new)]
        fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method(myMethod:)]
        fn method(mtm: MainThreadMarker, arg: i32, mtm2: MainThreadMarker) -> i32;

        #[method_id(myMethodId:)]
        fn method_id(mtm: MainThreadMarker, arg: &Self, mtm2: MainThreadMarker) -> Retained<Self>;
    }
);

#[test]
fn call() {
    let mtm = MainThreadMarker { _some_field: 0 };
    let obj1 = Cls::new(mtm);

    let res = Cls::method(mtm, 2, mtm);
    assert_eq!(res, 3);
    let res = Cls::protocol_method(mtm, 3);
    assert_eq!(res, 4);

    let obj2 = Cls::method_id(mtm, &obj1, mtm);
    assert_eq!(obj1, obj2);

    let obj2 = Cls::protocol_method_id(mtm, &obj1);
    assert_eq!(obj1, obj2);
}
