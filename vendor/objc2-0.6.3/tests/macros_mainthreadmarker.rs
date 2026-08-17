use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{define_class, extern_methods, extern_protocol};

extern_protocol!(
    #[allow(clippy::missing_safety_doc)]
    #[name = "MainThreadMarkerTestProtocol"]
    unsafe trait Proto: NSObjectProtocol {
        #[unsafe(method(myMethod:))]
        fn protocol_method(mtm: MainThreadMarker, arg: i32) -> i32;

        #[unsafe(method(myMethodRetained:))]
        fn protocol_method_retained(mtm: MainThreadMarker, arg: &Self) -> Retained<Self>;
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "MainThreadMarkerTest"]
    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Cls;

    unsafe impl NSObjectProtocol for Cls {}

    unsafe impl Proto for Cls {
        #[unsafe(method(myMethod:))]
        fn _my_mainthreadonly_method(arg: i32) -> i32 {
            arg + 1
        }

        #[unsafe(method_id(myMethodRetained:))]
        fn _my_mainthreadonly_method_retained(arg: &Self) -> Retained<Self> {
            unsafe { Retained::retain(arg as *const Self as *mut Self).unwrap() }
        }
    }
);

// The macro does a textual match; but when users actually use
// `objc2_foundation::MainThreadMarker` to ensure soundness, they will not
// do this!
#[derive(Clone, Copy)]
struct MainThreadMarker {
    _some_field: u32,
}

impl Cls {
    extern_methods!(
        #[unsafe(method(new))]
        fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(myMethod:))]
        fn method(mtm: MainThreadMarker, arg: i32, mtm2: MainThreadMarker) -> i32;

        #[unsafe(method(myMethodRetained:))]
        fn method_retained(
            mtm: MainThreadMarker,
            arg: &Self,
            mtm2: MainThreadMarker,
        ) -> Retained<Self>;
    );
}

#[test]
fn call() {
    let mtm = MainThreadMarker { _some_field: 0 };
    let obj1 = Cls::new(mtm);

    let res = Cls::method(mtm, 2, mtm);
    assert_eq!(res, 3);
    let res = Cls::protocol_method(mtm, 3);
    assert_eq!(res, 4);

    let obj2 = Cls::method_retained(mtm, &obj1, mtm);
    assert_eq!(obj1, obj2);

    let obj2 = Cls::protocol_method_retained(mtm, &obj1);
    assert_eq!(obj1, obj2);
}
