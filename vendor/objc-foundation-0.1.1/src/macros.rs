#[macro_export]
macro_rules! object_struct {
    ($name:ident) => (
        pub struct $name {
            _private: (),
        }

        unsafe impl ::objc::Message for $name { }

        impl $crate::INSObject for $name {
            fn class() -> &'static ::objc::runtime::Class {
                let name = stringify!($name);
                match ::objc::runtime::Class::get(name) {
                    Some(cls) => cls,
                    None => panic!("Class {} not found", name),
                }
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                use $crate::INSObject;
                self.is_equal(other)
            }
        }

        impl ::std::cmp::Eq for $name { }

        impl ::std::hash::Hash for $name {
            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
                use $crate::INSObject;
                self.hash_code().hash(state);
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use $crate::{INSObject, INSString};
                ::std::fmt::Debug::fmt(self.description().as_str(), f)
            }
        }
    );
}

macro_rules! object_impl {
    ($name:ident) => (
        object_impl!($name,);
    );
    ($name:ident<$($t:ident),+>) => (
        object_impl!($name, $($t),+);
    );
    ($name:ident, $($t:ident),*) => (
        unsafe impl<$($t),*> ::objc::Message for $name<$($t),*> { }
    );
}
