//! To remind myself that `Self` needs to work in methods in `define_class!`,
//! and hence whenever we name any of the types involved in this, we need to
//! do it in a context where `Self` works.
use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;
use objc2::{define_class, ClassType};

trait GetSameType {
    type SameType: ?Sized;
}

impl<T: ?Sized> GetSameType for T {
    type SameType = T;
}

trait GetRetained {
    type RetainedType;
}

impl<T> GetRetained for T {
    type RetainedType = Retained<T>;
}

macro_rules! get_self {
    () => {
        Self
    };
}

define_class!(
    #[unsafe(super(NSObject))]
    struct MyTestObject;

    impl MyTestObject {
        #[unsafe(method_id(initWith:))]
        fn init(
            _this: Allocated<<Self as GetSameType>::SameType>,
            _param: <*const Self as GetSameType>::SameType,
        ) -> Retained<<Self as GetSameType>::SameType> {
            unimplemented!()
        }

        #[unsafe(method(isEqual:))]
        fn is_equal(&self, _other: &Self) -> bool {
            unimplemented!()
        }

        #[unsafe(method_id(test4))]
        #[allow(unused_parens)]
        fn test4(_this: &<(Self) as GetSameType>::SameType) -> Retained<get_self!()> {
            unimplemented!()
        }

        #[unsafe(method_id(test5))]
        fn test5(&self) -> <Self as GetRetained>::RetainedType {
            unimplemented!()
        }
    }
);

#[test]
fn create_class() {
    let _ = MyTestObject::class();
}
