use smart_default::SmartDefault;

#[test]
fn test_unit() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo;

    assert!(Foo::default() == Foo);
}

#[test]
fn test_tuple() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo(
        #[default = 10] i32,
        #[default = 20] i32,
        // No default
        i32,
    );

    assert!(Foo::default() == Foo(10, 20, 0));
}

#[test]
fn test_struct() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo {
        #[default = 10]
        x: i32,
        #[default = 20]
        y: i32,
        // No default
        z: i32,
    }

    assert!(Foo::default() == Foo { x: 10, y: 20, z: 0 });
}

#[test]
fn test_enum_of_units() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar,
        #[default]
        Baz,
        #[allow(dead_code)]
        Qux,
    }

    assert!(Foo::default() == Foo::Baz);
}

#[test]
fn test_enum_of_tuples() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar(i32),
        #[default]
        Baz(#[default = 10] i32, i32),
        #[allow(dead_code)]
        Qux(i32),
    }

    assert!(Foo::default() == Foo::Baz(10, 0));
}

#[test]
fn test_enum_of_structs() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar { x: i32 },
        #[default]
        Baz {
            #[default = 10]
            y: i32,
            z: i32,
        },
        #[allow(dead_code)]
        Qux { w: i32 },
    }

    assert!(Foo::default() == Foo::Baz { y: 10, z: 0 });
}

#[test]
fn test_enum_mixed() {
    #[derive(PartialEq, SmartDefault)]
    enum Foo {
        #[allow(dead_code)]
        Bar,
        #[default]
        Baz(#[default = 10] i32),
        #[allow(dead_code)]
        Qux { w: i32 },
    }

    assert!(Foo::default() == Foo::Baz(10));
}

#[test]
fn test_generics_type_parameters() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo<T>
    where
        T: Default,
    {
        #[default(Some(Default::default()))]
        x: Option<T>,
    }

    assert!(Foo::default() == Foo { x: Some(0) });
}

#[test]
fn test_generics_lifetime_parameters() {
    // NOTE: A default value makes no sense with lifetime parameters, since ::default() receives no
    // paramters and therefore can receive no lifetimes. But it does make sense if you make a variant
    // without ref fields the default.

    #[derive(PartialEq, SmartDefault)]
    enum Foo<'a> {
        #[default]
        Bar(i32),
        #[allow(dead_code)]
        Baz(&'a str),
    }

    assert!(Foo::default() == Foo::Bar(0));
}

#[test]
fn test_code_hack() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo {
        #[default(_code = "vec![1, 2, 3]")]
        v: Vec<u32>,
    }

    assert!(Foo::default().v == [1, 2, 3]);
}

#[test]
fn test_string_conversion() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo(#[default = "one"] &'static str, #[default("two")] String);

    assert!(Foo::default() == Foo("one", "two".to_owned()));
}

#[test]
fn test_non_code_hack_valid_meta() {
    #[derive(Debug, PartialEq, SmartDefault)]
    struct Foo {
        #[default(true)]
        bar: bool,
        #[default(Option::None)]
        baz: Option<()>,
    }

    assert_eq!(
        Foo::default(),
        Foo {
            bar: true,
            baz: None
        }
    );
}
