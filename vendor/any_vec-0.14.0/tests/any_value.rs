use std::any::TypeId;
use std::mem::size_of;
use std::ptr::NonNull;
use any_vec::any_value::{AnyValue, AnyValueMut, AnyValueRaw, AnyValueWrapper};

#[test]
fn swap_test(){
    // typed <-> typed
    {
        let mut a1 = AnyValueWrapper::new(String::from("1"));
        let mut a2 = AnyValueWrapper::new(String::from("2"));

        a1.swap(&mut a2);
        assert_eq!(a1.downcast_ref::<String>().unwrap(), &String::from("2"));
        assert_eq!(a2.downcast_ref::<String>().unwrap(), &String::from("1"));
    }

    // untyped <-> untyped
    {
        let mut s1 = String::from("1");
        let mut s2 = String::from("2");

        let mut a1 = unsafe{
            AnyValueRaw::new(
            NonNull::from(&mut s1).cast::<u8>(),
            size_of::<String>(),
            TypeId::of::<String>()
        )};
        let mut a2 = unsafe{
            AnyValueRaw::new(
            NonNull::from(&mut s2).cast::<u8>(),
            size_of::<String>(),
            TypeId::of::<String>()
        )};

        a1.swap(&mut a2);
        assert_eq!(a1.downcast_ref::<String>().unwrap(), &String::from("2"));
        assert_eq!(a2.downcast_ref::<String>().unwrap(), &String::from("1"));
    }

    // untyped <-> typed
    {
        let mut s1 = String::from("1");
        let mut a1 = unsafe{
            AnyValueRaw::new(
            NonNull::from(&mut s1).cast::<u8>(),
            size_of::<String>(),
            TypeId::of::<String>()
        )};
        let mut a2 = AnyValueWrapper::new(String::from("2"));

        a1.swap(&mut a2);
        assert_eq!(a1.downcast_ref::<String>().unwrap(), &String::from("2"));
        assert_eq!(a2.downcast_ref::<String>().unwrap(), &String::from("1"));

        a2.swap(&mut a1);
        assert_eq!(a1.downcast_ref::<String>().unwrap(), &String::from("1"));
        assert_eq!(a2.downcast_ref::<String>().unwrap(), &String::from("2"));
    }
}