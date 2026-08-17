use derive_setters::*;

use std::borrow::Cow;

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(generate_delegates(ty = "BasicDelegateField", field = "x"))]
#[setters(generate_delegates(ty = "BasicDelegateMethod", method = "get_x"))]
#[setters(generate_delegates(ty = "PrefixBasicDelegateField", field = "x", prefix = "with_"))]
struct BasicStruct {
    #[setters(rename = "test")]
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct BasicDelegateField {
    x: BasicStruct,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct BasicDelegateMethod {
    x: Option<BasicStruct>,
}
impl BasicDelegateMethod {
    fn get_x(&mut self) -> &mut BasicStruct {
        if self.x.is_none() {
            self.x = Some(BasicStruct::default());
        }
        self.x.as_mut().unwrap()
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct PrefixBasicDelegateField {
    x: BasicStruct,
}

#[test]
fn basic_struct() {
    assert_eq!(BasicStruct::default().test(30).b(10).c(20), BasicStruct { a: 30, b: 10, c: 20 },);
    assert_eq!(BasicStruct::default().b(15).test(10), BasicStruct {
        a: 10,
        b: 15,
        ..Default::default()
    },);
}

#[test]
fn delegated_structs() {
    assert_eq!(BasicDelegateField::default().b(15).test(10), BasicDelegateField {
        x: BasicStruct { a: 10, b: 15, ..Default::default() }
    },);
    assert_eq!(BasicDelegateMethod::default().b(15).test(10), BasicDelegateMethod {
        x: Some(BasicStruct { a: 10, b: 15, ..Default::default() })
    },);
    assert_eq!(
        PrefixBasicDelegateField::default().with_b(15).with_test(10),
        PrefixBasicDelegateField { x: BasicStruct { a: 10, b: 15, ..Default::default() } },
    )
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
struct GenericStruct<'a, T> {
    a: Option<&'a T>,
    b: Option<&'a T>,
    c: T,
}

#[test]
fn generic_struct() {
    assert_eq!(GenericStruct::default().a(Some(&30)).b(Some(&10)).c(20), GenericStruct {
        a: Some(&30),
        b: Some(&10),
        c: 20
    },);
    assert_eq!(GenericStruct::default().b(Some(&15)).a(Some(&10)), GenericStruct {
        a: Some(&10),
        b: Some(&15),
        ..Default::default()
    },);
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(strip_option)]
struct StripOptionStruct {
    a: Option<u32>,
    b: core::option::Option<u32>,
    c: std::option::Option<u32>,
    d: u32,
}

#[test]
fn strip_option_struct() {
    assert_eq!(StripOptionStruct::default().a(3).b(42).c(43).d(7), StripOptionStruct {
        a: Some(3),
        b: Some(42),
        c: Some(43),
        d: 7
    },);
    assert_eq!(StripOptionStruct::default().b(6), StripOptionStruct {
        a: None,
        b: Some(6),
        c: None,
        d: 0
    },);
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(into, strip_option)]
struct IntoStruct<'a> {
    a: Cow<'a, str>,
    b: Option<Cow<'a, str>>,
}

#[test]
fn into_struct() {
    let testb = "testb".to_string();
    assert_eq!(IntoStruct::default().a("testa").b(&testb), IntoStruct {
        a: "testa".into(),
        b: Some("testb".into())
    },);
    assert_eq!(IntoStruct::default().a("testa").b(testb), IntoStruct {
        a: "testa".into(),
        b: Some("testb".into())
    },);
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(prefix = "with_")]
struct WithStruct {
    #[setters(rename = "test")]
    a: u32,
    b: u32,
    c: u32,
}

#[test]
fn with_struct() {
    assert_eq!(WithStruct::default().test(30).with_b(10).with_c(20), WithStruct {
        a: 30,
        b: 10,
        c: 20
    },);
    assert_eq!(WithStruct::default().with_b(15).test(10), WithStruct {
        a: 10,
        b: 15,
        ..Default::default()
    },);
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(bool)]
struct BoolStruct {
    a: bool,
    b: bool,
    c: bool,
}

#[test]
fn bool_struct() {
    assert_eq!(BoolStruct::default().a().c(), BoolStruct { a: true, b: false, c: true },);
    assert_eq!(BoolStruct::default().b().a(), BoolStruct { a: true, b: true, c: false },);
}
