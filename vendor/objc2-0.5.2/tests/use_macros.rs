use objc2::mutability::Immutable;
use objc2::runtime::{AnyClass, NSObject};
use objc2::{class, declare_class, msg_send, sel, ClassType, DeclaredClass};

declare_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "MyObject";
    }

    impl DeclaredClass for MyObject {}
);

#[test]
fn use_class_and_msg_send() {
    unsafe {
        let cls = class!(NSObject);
        let obj: *mut NSObject = msg_send![cls, new];
        let _hash: usize = msg_send![obj, hash];
        let _: () = msg_send![obj, release];
    }
}

#[test]
fn use_sel() {
    let _sel = sel!(description);
    let _sel = sel!(setObject:forKey:);
}

pub fn test_msg_send_comma_handling(obj: &MyObject, superclass: &AnyClass) {
    unsafe {
        let _: () = msg_send![obj, a];
        let _: () = msg_send![obj, a,];
        let _: () = msg_send![obj, a: 32i32];
        let _: () = msg_send![obj, a: 32i32,];
        #[cfg_attr(feature = "unstable-msg-send-always-comma", allow(deprecated))]
        let _: () = msg_send![obj, a: 32i32 b: 32i32];
        let _: () = msg_send![obj, a: 32i32, b: 32i32];
        let _: () = msg_send![obj, a: 32i32, b: 32i32,];
    }

    unsafe {
        let _: () = msg_send![super(obj, superclass), a];
        let _: () = msg_send![super(obj, superclass), a,];
        let _: () = msg_send![super(obj, superclass), a: 32i32];
        let _: () = msg_send![super(obj, superclass), a: 32i32,];
        #[cfg_attr(feature = "unstable-msg-send-always-comma", allow(deprecated))]
        let _: () = msg_send![super(obj, superclass), a: 32i32 b: 32i32];
        let _: () = msg_send![super(obj, superclass), a: 32i32, b: 32i32];
        let _: () = msg_send![super(obj, superclass), a: 32i32, b: 32i32,];
    }

    unsafe {
        let _: () = msg_send![super(obj), a];
        let _: () = msg_send![super(obj), a,];
        let _: () = msg_send![super(obj), a: 32i32];
        let _: () = msg_send![super(obj), a: 32i32,];
        let _: () = msg_send![super(obj), a: 32i32, b: 32i32];
        let _: () = msg_send![super(obj), a: 32i32, b: 32i32,];
    }
}
