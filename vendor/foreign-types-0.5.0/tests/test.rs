use foreign_types::foreign_type;

mod foo_sys {
    pub enum FOO {}

    pub unsafe extern "C" fn foo_drop(_: *mut FOO) {}
    pub unsafe extern "C" fn foo_clone(ptr: *mut FOO) -> *mut FOO { ptr }
}

foreign_type! {
    pub unsafe type Foo<'a, T>: Sync + Send {
        type CType = foo_sys::FOO;
        type PhantomData = &'a T;
        fn drop = foo_sys::foo_drop;
        fn clone = foo_sys::foo_clone;
    }

    pub unsafe type Foo2 {
        type CType = foo_sys::FOO;
        fn drop = foo_sys::foo_drop;
    }
}
