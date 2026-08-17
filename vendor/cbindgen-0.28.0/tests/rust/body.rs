
#[repr(C)]
pub struct MyFancyStruct {
    i: i32,
}

#[repr(C)]
pub enum MyFancyEnum {
    Foo,
    Bar(i32),
    Baz(i32),
}

#[repr(C)]
pub enum MyCLikeEnum {
    Foo1,
    Bar1,
    Baz1,
}

#[repr(C)]
pub union MyUnion {
    pub f: f32,
    pub u: u32,
}


#[repr(C)]
pub struct MyFancyStruct_Prepended {
    i: i32,
}

#[repr(C)]
pub enum MyFancyEnum_Prepended {
    Foo_Prepended,
    Bar_Prepended(i32),
    Baz_Prepended(i32),
}

#[repr(C)]
pub enum MyCLikeEnum_Prepended {
    Foo1_Prepended,
    Bar1_Prepended,
    Baz1_Prepended,
}

#[repr(C)]
pub union MyUnion_Prepended {
    pub f: f32,
    pub u: u32,
}


#[no_mangle]
pub extern "C" fn root(s: MyFancyStruct, e: MyFancyEnum, c: MyCLikeEnum, u: MyUnion, sp: MyFancyStruct_Prepended, ep: MyFancyEnum_Prepended, cp: MyCLikeEnum_Prepended, up: MyUnion_Prepended) {}
