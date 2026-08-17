// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    collections::Slice,
    prelude::*,
    translate::{FromGlib, IntoGlib},
};

#[test]
fn derive_error_domain() {
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::ErrorDomain)]
    #[error_domain(name = "TestError")]
    enum TestError {
        Invalid,
        Bad,
        Wrong,
    }

    let err = glib::Error::new(TestError::Bad, "oh no!");
    assert!(err.is::<TestError>());
    assert!(matches!(err.kind::<TestError>(), Some(TestError::Bad)));
}

#[test]
fn derive_shared_arc() {
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct MyInnerShared {
        foo: String,
    }
    #[derive(Debug, Eq, PartialEq, Clone, glib::SharedBoxed)]
    #[shared_boxed_type(name = "MyShared")]
    struct MyShared(std::sync::Arc<MyInnerShared>);

    let t = MyShared::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyShared");

    let p = MyShared(std::sync::Arc::new(MyInnerShared {
        foo: String::from("bar"),
    }));

    assert_eq!(std::sync::Arc::strong_count(&p.0), 1);
    let v = p.to_value();
    assert_eq!(std::sync::Arc::strong_count(&p.0), 2);
    let p_clone = v.get::<MyShared>().unwrap();
    assert_eq!(std::sync::Arc::strong_count(&p.0), 3);
    drop(p_clone);
    assert_eq!(std::sync::Arc::strong_count(&p.0), 2);
    drop(v);
    assert_eq!(std::sync::Arc::strong_count(&p.0), 1);
}

#[test]
fn derive_shared_arc_nullable() {
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct MyInnerNullableShared {
        foo: String,
    }
    #[derive(Clone, Debug, PartialEq, Eq, glib::SharedBoxed)]
    #[shared_boxed_type(name = "MyNullableShared", nullable)]
    struct MyNullableShared(std::sync::Arc<MyInnerNullableShared>);

    let t = MyNullableShared::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyNullableShared");

    let p = MyNullableShared(std::sync::Arc::new(MyInnerNullableShared {
        foo: String::from("bar"),
    }));

    assert_eq!(std::sync::Arc::strong_count(&p.0), 1);
    let _v = p.to_value();
    assert_eq!(std::sync::Arc::strong_count(&p.0), 2);

    let p = Some(MyNullableShared(std::sync::Arc::new(
        MyInnerNullableShared {
            foo: String::from("foo"),
        },
    )));

    assert_eq!(std::sync::Arc::strong_count(&p.as_ref().unwrap().0), 1);
    let v = p.to_value();
    assert_eq!(std::sync::Arc::strong_count(&p.as_ref().unwrap().0), 2);
    assert_eq!(
        p.as_ref().unwrap().0.foo,
        v.get::<MyNullableShared>().unwrap().0.foo
    );

    let b: Option<&MyNullableShared> = None;
    let v = b.to_value();
    assert_eq!(None, v.get::<Option<MyNullableShared>>().unwrap());
}

#[test]
fn derive_enum() {
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "TestAnimalType")]
    enum Animal {
        Goat,
        #[enum_value(name = "The Dog")]
        Dog,
        #[enum_value(name = "The Cat", nick = "chat")]
        Cat = 5,
        Badger,
    }

    assert_eq!(Animal::Goat.into_glib(), 0);
    assert_eq!(Animal::Dog.into_glib(), 1);
    assert_eq!(Animal::Cat.into_glib(), 5);

    assert_eq!(unsafe { Animal::from_glib(0) }, Animal::Goat);
    assert_eq!(unsafe { Animal::from_glib(1) }, Animal::Dog);
    assert_eq!(unsafe { Animal::from_glib(5) }, Animal::Cat);

    assert_eq!(Animal::Goat.to_value().get::<Animal>(), Ok(Animal::Goat));
    assert_eq!(Animal::Dog.to_value().get::<Animal>(), Ok(Animal::Dog));
    assert_eq!(Animal::Cat.to_value().get::<Animal>(), Ok(Animal::Cat));

    let t = Animal::static_type();
    assert!(t.is_a(glib::Type::ENUM));
    assert_eq!(t.name(), "TestAnimalType");

    let e = glib::EnumClass::with_type(t).expect("EnumClass::new failed");
    let v = e.value(0).expect("EnumClass::get_value(0) failed");
    assert_eq!(v.name(), "Goat");
    assert_eq!(v.nick(), "goat");
    let v = e.value(1).expect("EnumClass::get_value(1) failed");
    assert_eq!(v.name(), "The Dog");
    assert_eq!(v.nick(), "dog");
    let v = e.value(5).expect("EnumClass::get_value(5) failed");
    assert_eq!(v.name(), "The Cat");
    assert_eq!(v.nick(), "chat");
    assert_eq!(e.value(2), None);
}

#[test]
fn derive_boxed() {
    #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
    #[boxed_type(name = "MyBoxed")]
    struct MyBoxed(String);

    let t = MyBoxed::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyBoxed");

    let b = MyBoxed(String::from("abc"));
    let v = b.to_value();
    assert_eq!(&b, v.get::<&MyBoxed>().unwrap());
    assert_eq!(b, v.get::<MyBoxed>().unwrap());
}

#[allow(clippy::unnecessary_literal_unwrap)]
#[test]
fn derive_boxed_nullable() {
    #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
    #[boxed_type(name = "MyNullableBoxed", nullable)]
    struct MyNullableBoxed(String);

    let t = MyNullableBoxed::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyNullableBoxed");

    let b = MyNullableBoxed(String::from("abc"));
    let v = b.to_value();
    assert_eq!(&b, v.get::<Option<&MyNullableBoxed>>().unwrap().unwrap());
    assert_eq!(b, v.get::<Option<MyNullableBoxed>>().unwrap().unwrap());

    let b = Some(MyNullableBoxed(String::from("def")));
    let v = b.to_value();
    let b = b.unwrap();
    assert_eq!(&b, v.get::<Option<&MyNullableBoxed>>().unwrap().unwrap());
    assert_eq!(b, v.get::<Option<MyNullableBoxed>>().unwrap().unwrap());

    let b = Some(MyNullableBoxed(String::from("def")));
    let v = b.to_value();
    let b = b.unwrap();
    assert_eq!(&b, v.get::<Option<&MyNullableBoxed>>().unwrap().unwrap());
    assert_eq!(b, v.get::<Option<MyNullableBoxed>>().unwrap().unwrap());

    let b: Option<MyNullableBoxed> = None;
    let v = b.to_value();
    assert_eq!(None, v.get::<Option<&MyNullableBoxed>>().unwrap());
    assert_eq!(None, v.get::<Option<MyNullableBoxed>>().unwrap());
}

#[test]
fn boxed_transparent() {
    #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
    #[boxed_type(name = "MyBoxed")]
    struct MyBoxed(String);

    let vec = vec![MyBoxed(String::from("abc")), MyBoxed(String::from("dfg"))];

    // Slice requires TransparentType trait
    let slice = Slice::from(vec);
    assert_eq!(slice.last(), Some(MyBoxed(String::from("dfg"))).as_ref());
}

#[test]
fn attr_flags() {
    #[glib::flags(name = "MyFlags")]
    enum MyFlags {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    assert_eq!(MyFlags::A.bits(), 1);
    assert_eq!(MyFlags::B.bits(), 2);
    assert_eq!(MyFlags::AB.bits(), 3);

    assert_eq!(MyFlags::empty().into_glib(), 0);
    assert_eq!(MyFlags::A.into_glib(), 1);
    assert_eq!(MyFlags::B.into_glib(), 2);
    assert_eq!(MyFlags::AB.into_glib(), 3);

    assert_eq!(unsafe { MyFlags::from_glib(0) }, MyFlags::empty());
    assert_eq!(unsafe { MyFlags::from_glib(1) }, MyFlags::A);
    assert_eq!(unsafe { MyFlags::from_glib(2) }, MyFlags::B);
    assert_eq!(unsafe { MyFlags::from_glib(3) }, MyFlags::AB);

    assert_eq!(
        MyFlags::empty().to_value().get::<MyFlags>(),
        Ok(MyFlags::empty())
    );
    assert_eq!(MyFlags::A.to_value().get::<MyFlags>(), Ok(MyFlags::A));
    assert_eq!(MyFlags::B.to_value().get::<MyFlags>(), Ok(MyFlags::B));
    assert_eq!(MyFlags::AB.to_value().get::<MyFlags>(), Ok(MyFlags::AB));

    let t = MyFlags::static_type();
    assert!(t.is_a(glib::Type::FLAGS));
    assert_eq!(t.name(), "MyFlags");

    let e = glib::FlagsClass::with_type(t).expect("FlagsClass::new failed");
    let v = e.value(1).expect("FlagsClass::get_value(1) failed");
    assert_eq!(v.name(), "Flag A");
    assert_eq!(v.nick(), "nick-a");
    let v = e.value(2).expect("FlagsClass::get_value(2) failed");
    assert_eq!(v.name(), "Flag B");
    assert_eq!(v.nick(), "b");
    let v = e.value(4).expect("FlagsClass::get_value(4) failed");
    assert_eq!(v.name(), "C");
    assert_eq!(v.nick(), "c");

    assert!(e.value_by_name("Flag A").is_some());
    assert!(e.value_by_name("Flag B").is_some());
    assert!(e.value_by_name("AB").is_none());
    assert!(e.value_by_name("C").is_some());

    assert!(e.value_by_nick("nick-a").is_some());
    assert!(e.value_by_nick("b").is_some());
    assert!(e.value_by_nick("ab").is_none());
    assert!(e.value_by_nick("c").is_some());
}

#[test]
fn attr_flags_with_default() {
    #[glib::flags(name = "MyFlags")]
    enum MyFlags {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[default]
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
    }

    assert_eq!(MyFlags::A.bits(), 1);
    assert_eq!(MyFlags::B.bits(), 2);
    assert_eq!(MyFlags::default(), MyFlags::B);
    assert_eq!(MyFlags::default().into_glib(), 2);
}

#[test]
fn subclassable() {
    mod foo {
        use glib::subclass::prelude::*;

        use super::*;

        mod imp {
            use super::*;

            #[derive(Default)]
            pub struct Foo {}

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                const NAME: &'static str = "MyFoo";
                type Type = super::Foo;
            }

            impl ObjectImpl for Foo {}

            impl Foo {
                pub(super) fn test(&self) {}
            }
        }

        pub trait FooExt: IsA<Foo> + 'static {
            fn test(&self) {
                let imp = self.as_ref().upcast_ref::<Foo>().imp();
                imp.test();
            }
        }

        impl<O: IsA<Foo>> FooExt for O {}

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<imp::Foo>);
        }
    }

    use foo::FooExt;

    let obj = glib::Object::new::<foo::Foo>();
    obj.test();
}

#[test]
fn derive_variant() {
    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant1 {
        some_string: String,
        some_int: i32,
    }

    assert_eq!(Variant1::static_variant_type().as_str(), "(si)");
    let v = Variant1 {
        some_string: String::from("bar"),
        some_int: 2,
    };
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(si)");
    assert_eq!(var.get::<Variant1>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant2 {
        some_string: Option<String>,
        some_int: i32,
    }

    assert_eq!(Variant2::static_variant_type().as_str(), "(msi)");
    let v = Variant2 {
        some_string: Some(String::from("bar")),
        some_int: 2,
    };
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(msi)");
    assert_eq!(var.get::<Variant2>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant3(u32, String);

    assert_eq!(Variant3::static_variant_type().as_str(), "(us)");
    let v = Variant3(1, String::from("foo"));
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(us)");
    assert_eq!(var.get::<Variant3>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant4;

    assert_eq!(Variant4::static_variant_type().as_str(), "()");
    let v = Variant4;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "()");
    assert_eq!(var.get::<Variant4>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant5();

    assert_eq!(Variant5::static_variant_type().as_str(), "()");
    let v = Variant5();
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "()");
    assert_eq!(var.get::<Variant5>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    enum Variant6 {
        Unit,
        Tuple(i32, Variant1),
        Struct { id: i64, data: Variant2 },
    }

    assert_eq!(Variant6::static_variant_type().as_str(), "(sv)");
    let v = Variant6::Unit;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(sv)");
    assert_eq!(var.get::<Variant6>(), Some(v));
    let v = Variant6::Tuple(
        5,
        Variant1 {
            some_string: "abc".into(),
            some_int: 77,
        },
    );
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(sv)");
    assert_eq!(var.get::<Variant6>(), Some(v));
    let v = Variant6::Struct {
        id: 299,
        data: Variant2 {
            some_string: Some("abcdef".into()),
            some_int: 300,
        },
    };
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(sv)");
    assert_eq!(var.get::<Variant6>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    #[variant_enum(repr)]
    #[repr(u32)]
    enum Variant7 {
        Unit,
        Tuple(i32, String),
        Struct { id: i64, data: Vec<u8> },
    }

    assert_eq!(Variant7::static_variant_type().as_str(), "(uv)");
    let v = Variant7::Struct {
        id: 299,
        data: vec![55, 56, 57, 58],
    };
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(uv)");
    assert_eq!(var.get::<Variant7>(), Some(v));

    #[derive(Debug, PartialEq, Eq, Clone, Copy, glib::Variant, glib::Enum)]
    #[variant_enum(enum)]
    #[repr(i32)]
    #[enum_type(name = "Variant8")]
    enum Variant8 {
        Goat,
        #[enum_value(name = "The Dog")]
        Dog,
        #[enum_value(name = "The Cat", nick = "chat")]
        Cat = 5,
        Badger,
    }

    assert_eq!(Variant8::static_variant_type().as_str(), "s");
    let v = Variant8::Cat;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "s");
    assert_eq!(var.to_string(), "'chat'");
    assert_eq!(var.get::<Variant8>(), Some(v));

    #[derive(Debug, PartialEq, Eq, Clone, Copy, glib::Variant, glib::Enum)]
    #[variant_enum(enum, repr)]
    #[repr(i32)]
    #[enum_type(name = "Variant9")]
    enum Variant9 {
        Goat,
        #[enum_value(name = "The Dog")]
        Dog,
        #[enum_value(name = "The Cat", nick = "chat")]
        Cat = 5,
        Badger,
    }

    assert_eq!(Variant9::static_variant_type().as_str(), "i");
    let v = Variant9::Badger;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "i");
    assert_eq!(var.get::<Variant9>(), Some(v));

    #[derive(glib::Variant)]
    #[variant_enum(flags)]
    #[glib::flags(name = "Variant10")]
    enum Variant10 {
        EMPTY = 0,
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    assert_eq!(Variant10::static_variant_type().as_str(), "s");
    let v = Variant10::AB;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "s");
    assert_eq!(var.to_string(), "'nick-a|b'");
    assert_eq!(var.get::<Variant10>(), Some(v));

    #[derive(glib::Variant)]
    #[variant_enum(flags, repr)]
    #[glib::flags(name = "Variant11")]
    enum Variant11 {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    assert_eq!(Variant11::static_variant_type().as_str(), "u");
    let v = Variant11::A | Variant11::C;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "u");
    assert_eq!(var.get::<Variant11>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    enum Variant12 {
        Goat,
        Dog,
        Cat = 5,
        Badger,
    }

    assert_eq!(Variant12::static_variant_type().as_str(), "s");
    let v = Variant12::Dog;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "s");
    assert_eq!(var.get::<Variant12>(), Some(v));

    #[derive(Debug, PartialEq, Eq, Copy, Clone, glib::Variant)]
    #[variant_enum(repr)]
    #[repr(u8)]
    enum Variant13 {
        Goat,
        Dog,
        Cat = 5,
        Badger,
    }

    assert_eq!(Variant13::static_variant_type().as_str(), "y");
    let v = Variant13::Badger;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "y");
    assert_eq!(var.get::<Variant13>(), Some(v));
}

#[test]
fn closure() {
    let empty = glib::closure!(|| {});
    empty.invoke::<()>(&[]);

    let no_arg = glib::closure!(|| 2i32);
    assert_eq!(no_arg.invoke::<i32>(&[]), 2);

    let add_1 = glib::closure!(|x: i32| x + 1);
    assert_eq!(add_1.invoke::<i32>(&[&3i32]), 4);

    let concat_str = glib::closure!(|s: &str| s.to_owned() + " World");
    assert_eq!(concat_str.invoke::<String>(&[&"Hello"]), "Hello World");

    let weak_test = {
        let obj = glib::Object::new::<glib::Object>();

        assert_eq!(obj.ref_count(), 1);
        let weak_test = glib::closure_local!(
            #[watch]
            obj,
            move || obj.ref_count()
        );
        assert_eq!(obj.ref_count(), 1);
        assert_eq!(weak_test.invoke::<u32>(&[]), 2);
        assert_eq!(obj.ref_count(), 1);

        weak_test
    };
    weak_test.invoke::<()>(&[]);

    {
        trait TestExt {
            fn ref_count_in_closure(&self) -> u32;
        }

        impl TestExt for glib::Object {
            fn ref_count_in_closure(&self) -> u32 {
                let closure = glib::closure_local!(
                    #[watch(rename_to = obj)]
                    self,
                    move || obj.ref_count()
                );
                closure.invoke::<u32>(&[])
            }
        }

        let obj = glib::Object::new::<glib::Object>();
        assert_eq!(obj.ref_count_in_closure(), 2);
    }

    {
        struct A {
            obj: glib::Object,
        }

        impl A {
            fn ref_count_in_closure(&self) -> u32 {
                let closure = glib::closure_local!(
                    #[watch(rename_to = obj)]
                    self.obj,
                    move || obj.ref_count()
                );
                closure.invoke::<u32>(&[])
            }
        }

        let a = A {
            obj: glib::Object::new::<glib::Object>(),
        };
        assert_eq!(a.ref_count_in_closure(), 2);
    }

    let strong_test = {
        let obj = glib::Object::new::<glib::Object>();

        let strong_test = glib::closure_local!(
            #[strong]
            obj,
            move || obj.ref_count()
        );
        assert_eq!(strong_test.invoke::<u32>(&[]), 2);

        strong_test
    };
    assert_eq!(strong_test.invoke::<u32>(&[]), 1);

    let weak_none_test = {
        let obj = glib::Object::new::<glib::Object>();

        let weak_none_test = glib::closure_local!(
            #[weak_allow_none]
            obj,
            move || { obj.map(|o| o.ref_count()).unwrap_or_default() }
        );
        assert_eq!(weak_none_test.invoke::<u32>(&[]), 2);

        weak_none_test
    };
    assert_eq!(weak_none_test.invoke::<u32>(&[]), 0);

    let weak_test_or_else = {
        let obj = glib::Object::new::<glib::Object>();

        let weak_test = glib::closure_local!(
            #[weak]
            obj,
            #[upgrade_or_else]
            || 0,
            move || obj.ref_count()
        );
        assert_eq!(weak_test.invoke::<u32>(&[]), 2);

        weak_test
    };
    assert_eq!(weak_test_or_else.invoke::<u32>(&[]), 0);

    let weak_test_or = {
        let obj = glib::Object::new::<glib::Object>();

        let weak_test = glib::closure_local!(
            #[weak]
            obj,
            #[upgrade_or]
            0,
            move || obj.ref_count()
        );
        assert_eq!(weak_test.invoke::<u32>(&[]), 2);

        weak_test
    };
    assert_eq!(weak_test_or.invoke::<u32>(&[]), 0);

    let weak_test_or_default = {
        let obj = glib::Object::new::<glib::Object>();

        let weak_test = glib::closure_local!(
            #[weak]
            obj,
            #[upgrade_or_default]
            move || obj.ref_count()
        );
        assert_eq!(weak_test.invoke::<u32>(&[]), 2);

        weak_test
    };
    assert_eq!(weak_test_or_default.invoke::<u32>(&[]), 0);

    {
        let ret = std::rc::Rc::new(std::cell::Cell::new(0));
        let weak_test_or_unit = {
            let obj = glib::Object::new::<glib::Object>();

            let weak_test = glib::closure_local!(
                #[weak]
                obj,
                #[strong]
                ret,
                move || {
                    ret.set(obj.ref_count());
                }
            );
            weak_test.invoke::<()>(&[]);
            assert_eq!(ret.get(), 2);
            ret.set(0);

            weak_test
        };
        weak_test_or_unit.invoke::<()>(&[]);

        assert_eq!(ret.get(), 0);
    }

    {
        let obj1 = glib::Object::new::<glib::Object>();
        let obj2 = glib::Object::new::<glib::Object>();

        let obj_arg_test =
            glib::closure!(|a: glib::Object, b: glib::Object| { a.ref_count() + b.ref_count() });
        let rc = obj_arg_test.invoke::<u32>(&[&obj1, &obj2]);
        assert_eq!(rc, 6);

        let alias_test = glib::closure_local!(
            #[strong(rename_to = a)]
            obj1,
            #[strong]
            obj2,
            move || { a.ref_count() + obj2.ref_count() }
        );
        assert_eq!(alias_test.invoke::<u32>(&[]), 4);
    }

    {
        struct A {
            a: glib::Object,
        }

        let a = glib::Object::new::<glib::Object>();
        let a_struct = A { a };
        let struct_test = glib::closure_local!(
            #[strong(rename_to = a)]
            a_struct.a,
            move || { a.ref_count() }
        );
        assert_eq!(struct_test.invoke::<u32>(&[]), 2);
    }

    {
        use glib::{prelude::*, subclass::prelude::*};

        #[derive(Default)]
        pub struct FooPrivate {}

        #[glib::object_subclass]
        impl ObjectSubclass for FooPrivate {
            const NAME: &'static str = "MyFoo2";
            type Type = Foo;
        }

        impl ObjectImpl for FooPrivate {}

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<FooPrivate>);
        }

        impl Foo {
            fn my_ref_count(&self) -> u32 {
                self.ref_count()
            }
        }

        let cast_test = {
            let f = glib::Object::new::<Foo>();

            assert_eq!(f.my_ref_count(), 1);
            let cast_test = glib::closure_local!(
                #[watch]
                f,
                move || f.my_ref_count()
            );
            assert_eq!(f.my_ref_count(), 1);
            assert_eq!(cast_test.invoke::<u32>(&[]), 2);
            assert_eq!(f.my_ref_count(), 1);

            let f_ref = &f;
            let _ = glib::closure_local!(
                #[watch]
                f_ref,
                move || f_ref.my_ref_count()
            );

            cast_test
        };
        cast_test.invoke::<()>(&[]);
    }

    {
        use glib::subclass::prelude::*;

        #[derive(Default)]
        pub struct SendObjectPrivate {
            value: std::sync::Mutex<i32>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for SendObjectPrivate {
            const NAME: &'static str = "SendObject";
            type Type = SendObject;
        }

        impl ObjectImpl for SendObjectPrivate {}

        glib::wrapper! {
            pub struct SendObject(ObjectSubclass<SendObjectPrivate>);
        }
        impl SendObject {
            fn value(&self) -> i32 {
                *self.imp().value.lock().unwrap()
            }
            fn set_value(&self, v: i32) {
                *self.imp().value.lock().unwrap() = v;
            }
        }

        let inc_by = {
            let obj = glib::Object::new::<SendObject>();
            let obj = obj.imp().obj();
            let inc_by = glib::closure!(
                #[watch]
                obj,
                move |x: i32| {
                    let old = obj.value();
                    obj.set_value(x + old);
                    old
                }
            );
            obj.set_value(42);
            assert_eq!(obj.value(), 42);
            assert_eq!(inc_by.invoke::<i32>(&[&24i32]), 42);
            assert_eq!(obj.value(), 66);
            inc_by
        };
        inc_by.invoke::<()>(&[]);
    }
}
