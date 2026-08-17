use crate::prelude::*;

#[test]
fn by_ref() {
    #[derive(Debug, Builder)]
    struct Sut<T> {
        #[builder(start_fn)]
        _x1: u32,

        #[builder(getter(name = x2_with_custom_name))]
        _x2: &'static str,

        #[builder(getter(vis = "pub(crate)", doc {
            /// Docs on the getter
        }))]
        _x3: u32,

        #[builder(into, getter(name = x4_override, vis = "pub(crate)", doc {
            /// Docs on getter
        }))]
        _x4: &'static str,

        _no_getter: u32,

        #[builder(getter)]
        _generic_option: Option<T>,

        _x5: (),

        #[builder(getter, default)]
        _x6: u32,
    }

    let builder = Sut::builder(0u32)
        .x2("2")
        .x3(3)
        .x4("4")
        .no_getter(5)
        .x5(())
        .maybe_generic_option(None::<()>)
        .x6(7);

    let actual = (
        assert_getter::<&&'static str, _>(&builder, SutBuilder::x2_with_custom_name),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_x3),
        assert_getter::<&&'static str, _>(&builder, SutBuilder::x4_override),
        assert_getter::<Option<&()>, _>(&builder, SutBuilder::get_generic_option),
        assert_getter::<Option<&u32>, _>(&builder, SutBuilder::get_x6),
    );

    assert_debug_eq(actual, expect![[r#"("2", 3, "4", None, Some(7))"#]]);
}

#[test]
fn clone() {
    #[derive(Clone, Debug)]
    struct CloneNotCopy(#[allow(dead_code)] u32);

    #[derive(Builder)]
    struct Sut {
        #[builder(getter(clone))]
        _x1: CloneNotCopy,

        #[builder(getter(clone))]
        _x2: Option<CloneNotCopy>,

        #[builder(getter(clone), default = CloneNotCopy(0))]
        _x3: CloneNotCopy,
    }

    let sut = Sut::builder()
        .x1(CloneNotCopy(1))
        .x2(CloneNotCopy(2))
        .x3(CloneNotCopy(3));

    let actual = (
        assert_getter::<CloneNotCopy, _>(&sut, SutBuilder::get_x1),
        assert_getter::<Option<CloneNotCopy>, _>(&sut, SutBuilder::get_x2),
        assert_getter::<Option<CloneNotCopy>, _>(&sut, SutBuilder::get_x3),
    );

    assert_debug_eq(
        actual,
        expect![[r#"
        (
            CloneNotCopy(
                1,
            ),
            Some(
                CloneNotCopy(
                    2,
                ),
            ),
            Some(
                CloneNotCopy(
                    3,
                ),
            ),
        )"#]],
    );
}

#[test]
fn copy() {
    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(getter(copy))]
        _x1: u32,

        #[builder(getter(copy))]
        _x2: Option<u32>,

        #[builder(getter(copy), default)]
        _x3: u32,
    }

    let sut = Sut::builder().x1(1).x2(2).x3(3);

    let actual = (
        assert_getter::<u32, _>(&sut, SutBuilder::get_x1),
        assert_getter::<Option<u32>, _>(&sut, SutBuilder::get_x2),
        assert_getter::<Option<u32>, _>(&sut, SutBuilder::get_x3),
    );

    assert_debug_eq(actual, expect!["(1, Some(2), Some(3))"]);
}

#[test]
#[cfg(feature = "std")]
fn deref_implicit() {
    use std::borrow::Cow;
    use std::ffi::CStr;
    use std::ffi::{CString, OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[derive(Debug, Builder)]
    struct Sut<'a> {
        #[builder(getter(deref))]
        _vec: Vec<u32>,

        #[builder(getter(deref))]
        _optional_vec: Option<Vec<u32>>,

        #[builder(getter(deref), default)]
        _default_vec: Vec<u32>,

        #[builder(getter(deref))]
        _box_: Box<u32>,

        #[builder(getter(deref))]
        _rc: Rc<u32>,

        #[builder(getter(deref))]
        _arc: Arc<u32>,

        #[builder(getter(deref))]
        _string: String,

        #[builder(getter(deref))]
        _c_string: CString,

        #[builder(getter(deref))]
        _os_string: OsString,

        #[builder(getter(deref))]
        _path_buf: PathBuf,

        #[builder(getter(deref))]
        _cow: Cow<'a, str>,
    }

    let builder = Sut::builder()
        .vec(vec![1, 2, 3])
        .maybe_optional_vec(None)
        .default_vec(vec![0])
        .box_(Box::new(4))
        .rc(Rc::new(5))
        .arc(Arc::new(6))
        .string("7".to_string())
        .c_string(CString::new("8").unwrap())
        .os_string(OsString::from("9"))
        .path_buf(PathBuf::from("10"))
        .cow(Cow::Borrowed("11"));

    let actual = (
        assert_getter::<&[u32], _>(&builder, SutBuilder::get_vec),
        assert_getter::<Option<&[u32]>, _>(&builder, SutBuilder::get_optional_vec),
        assert_getter::<Option<&[u32]>, _>(&builder, SutBuilder::get_default_vec),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_box_),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_rc),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_arc),
        assert_getter::<&str, _>(&builder, SutBuilder::get_string),
        assert_getter::<&CStr, _>(&builder, SutBuilder::get_c_string),
        assert_getter::<&OsStr, _>(&builder, SutBuilder::get_os_string),
        assert_getter::<&Path, _>(&builder, SutBuilder::get_path_buf),
        assert_getter::<&str, _>(&builder, SutBuilder::get_cow),
    );

    assert_debug_eq(
        actual,
        expect![[r#"
            (
                [
                    1,
                    2,
                    3,
                ],
                None,
                Some(
                    [
                        0,
                    ],
                ),
                4,
                5,
                6,
                "7",
                "8",
                "9",
                "10",
                "11",
            )"#]],
    );
}

#[test]
#[cfg(feature = "std")]
fn deref_explicit() {
    use std::rc::Rc;

    #[derive(Debug, Builder)]
    #[allow(clippy::rc_buffer)]
    struct Sut {
        // Make sure a deref coercion happens through multiple layers
        #[builder(getter(deref(str)))]
        _x1: Rc<String>,

        #[builder(getter(deref(str)))]
        _x2: Option<Rc<String>>,

        #[builder(getter(deref(str)), default)]
        _x3: Rc<String>,
    }

    let sut = Sut::builder()
        .x1(Rc::new("hello".to_owned()))
        .x2(Rc::new("world".to_owned()))
        .x3(Rc::new("!".to_owned()));

    let actual = (
        assert_getter::<&str, _>(&sut, SutBuilder::get_x1),
        assert_getter::<Option<&str>, _>(&sut, SutBuilder::get_x2),
        assert_getter::<Option<&str>, _>(&sut, SutBuilder::get_x3),
    );

    assert_debug_eq(actual, expect![[r#"("hello", Some("world"), Some("!"))"#]]);
}

/// Helper function that is better than just `let _: ExpectedType = builder.get_foo();`
/// this notation involves an implicit deref coercion, but we want to assert the exact
/// return type of the getter without any additional implicit conversions.
fn assert_getter<'a, T, B>(builder: &'a B, method: fn(&'a B) -> T) -> T {
    method(builder)
}
