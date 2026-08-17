use crate::prelude::*;

struct IntoBool(bool);

impl From<IntoBool> for bool {
    fn from(value: IntoBool) -> Self {
        value.0
    }
}

struct IntoUnit;

impl From<IntoUnit> for () {
    fn from(IntoUnit: IntoUnit) {}
}

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(value: IntoStrRef<'a>) -> Self {
        value.0
    }
}

struct Generic<T>(T);
struct IntoGeneric<T>(T);

impl<T> From<IntoGeneric<T>> for Generic<T> {
    fn from(value: IntoGeneric<T>) -> Self {
        Self(value.0)
    }
}

#[test]
fn match_any() {
    #[builder(on(_, into))]
    fn sut<T>(_arg1: bool, _arg2: Option<()>, _arg3: T) {}

    sut::<&str>()
        .arg1(IntoBool(true))
        .arg2(IntoUnit)
        .arg3(IntoStrRef("foo"))
        .call();
}

#[test]
fn match_str_ref() {
    #[builder(on(&str, into))]
    fn sut(_arg1: bool, _arg2: Option<()>, _arg3: &str) {}

    sut().arg1(true).arg2(()).arg3(IntoStrRef("foo")).call();
}

#[test]
fn match_path() {
    #[builder(on(bool, into))]
    fn sut<T>(_arg1: bool, _arg2: Option<()>, _arg3: T) {}

    sut::<&str>()
        .arg1(IntoBool(true))
        .arg2(())
        .arg3("foo")
        .call();
}

#[test]
fn match_generic() {
    #[builder(on(Generic<_>, into))]
    fn sut<T>(_arg1: bool, _arg2: Option<()>, _arg3: Generic<T>) {}

    sut().arg1(true).arg2(()).arg3(IntoGeneric("foo")).call();
}
