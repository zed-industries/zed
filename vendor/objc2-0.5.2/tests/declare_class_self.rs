//! To remind myself that `Self` needs to work in methods in `declare_class!`,
//! and hence whenever we name any of the types involved in this, we need to
//! do it in a context where `Self` works.
use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType, DeclaredClass};

trait GetSameType {
    type SameType: ?Sized;
}

impl<T: ?Sized> GetSameType for T {
    type SameType = T;
}

trait GetId {
    type IdType;
}

impl<T> GetId for T {
    type IdType = Retained<T>;
}

macro_rules! get_self {
    () => {
        Self
    };
}

declare_class!(
    struct MyTestObject;

    unsafe impl ClassType for MyTestObject {
        type Super = NSObject;
        type Mutability = mutability::Mutable;

        const NAME: &'static str = "MyTestObject";
    }

    impl DeclaredClass for MyTestObject {}

    unsafe impl MyTestObject {
        #[method_id(initWith:)]
        fn init(
            _this: Allocated<<Self as GetSameType>::SameType>,
            _param: <*const Self as GetSameType>::SameType,
        ) -> Retained<<Self as GetSameType>::SameType> {
            unimplemented!()
        }

        #[method(isEqual:)]
        fn is_equal(&self, _other: &Self) -> bool {
            unimplemented!()
        }

        #[method_id(test4)]
        #[allow(unused_parens)]
        fn test4(_this: &<(Self) as GetSameType>::SameType) -> Retained<get_self!()> {
            unimplemented!()
        }

        #[method_id(test5)]
        fn test5(&self) -> <Self as GetId>::IdType {
            unimplemented!()
        }
    }
);

#[test]
fn create_class() {
    let _ = MyTestObject::class();
}
