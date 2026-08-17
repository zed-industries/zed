#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
struct Lorem {
    #[builder(setter(each = "foo_append"))]
    foo: String,
    #[builder(setter(each = "bar"))]
    bars: Vec<String>,
    #[builder(setter(each = "baz"))]
    bazes: HashMap<String, i32>,
    #[builder(setter(strip_option, each = "qux"))]
    quxes: Option<Vec<String>>,
    #[builder(setter(strip_option, each = "quux"))]
    quuxes: Option<HashMap<String, i32>>,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(pattern = "mutable")]
struct Ipsum {
    #[builder(setter(each(name = "foo_append")))]
    foo: String,
    #[builder(setter(each(name = "bar")))]
    bars: Vec<String>,
    #[builder(setter(each(name = "baz")))]
    bazes: HashMap<String, i32>,
    #[builder(setter(strip_option, each = "qux"))]
    quxes: Option<Vec<String>>,
    #[builder(setter(strip_option, each = "quux"))]
    quuxes: Option<HashMap<String, i32>>,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder]
struct Dolor {
    #[builder(setter(each(name = "foo_append")))]
    foo: String,
    #[builder(setter(each(name = "bar", into)))]
    bars: Vec<String>,
    #[builder(setter(each(name = "baz")))]
    bazes: HashMap<String, i32>,
}

#[test]
fn extend_field() {
    let x = LoremBuilder::default()
        .foo("foo".into())
        .bar("bar".into())
        .bar("bar bar".into())
        .bar("bar bar bar".into())
        .foo_append('-')
        .baz(("baz".into(), 1))
        .baz(("bazz".into(), 2))
        .baz(("bazzz".into(), 3))
        .foo_append("foo")
        .qux("qux".into())
        .qux("qux qux".into())
        .quux(("quux".into(), 1))
        .quux(("quuxx".into(), 2))
        .build()
        .unwrap();

    assert_eq!(
        x,
        Lorem {
            foo: "foo-foo".into(),
            bars: vec!["bar".into(), "bar bar".into(), "bar bar bar".into()],
            bazes: vec![("baz".into(), 1), ("bazz".into(), 2), ("bazzz".into(), 3)]
                .into_iter()
                .collect(),
            quxes: Some(vec!["qux".into(), "qux qux".into()]),
            quuxes: Some(
                vec![("quux".into(), 1), ("quuxx".into(), 2)]
                    .into_iter()
                    .collect()
            ),
        }
    );
}

#[test]
fn extend_field_into() {
    let x = DolorBuilder::default()
        .foo("foo".into())
        .bar("bar")
        .bar("bar bar")
        .bar("bar bar bar")
        .foo_append('-')
        .baz(("baz".into(), 1))
        .baz(("bazz".into(), 2))
        .baz(("bazzz".into(), 3))
        .foo_append("foo")
        .build()
        .unwrap();

    assert_eq!(
        x,
        Dolor {
            foo: "foo-foo".into(),
            bars: vec!["bar".into(), "bar bar".into(), "bar bar bar".into()],
            bazes: vec![("baz".into(), 1), ("bazz".into(), 2), ("bazzz".into(), 3)]
                .into_iter()
                .collect(),
        }
    );
}

#[test]
fn extend_field_mutable() {
    let x = IpsumBuilder::default()
        .foo("foo".into())
        .bar("bar".into())
        .bar("bar bar".into())
        .bar("bar bar bar".into())
        .foo_append('-')
        .baz(("baz".into(), 1))
        .baz(("bazz".into(), 2))
        .baz(("bazzz".into(), 3))
        .foo_append("foo")
        .qux("qux".into())
        .qux("qux qux".into())
        .quux(("quux".into(), 1))
        .quux(("quuxx".into(), 2))
        .build()
        .unwrap();

    assert_eq!(
        x,
        Ipsum {
            foo: "foo-foo".into(),
            bars: vec!["bar".into(), "bar bar".into(), "bar bar bar".into()],
            bazes: vec![("baz".into(), 1), ("bazz".into(), 2), ("bazzz".into(), 3)]
                .into_iter()
                .collect(),
            quxes: Some(vec!["qux".into(), "qux qux".into()]),
            quuxes: Some(
                vec![("quux".into(), 1), ("quuxx".into(), 2)]
                    .into_iter()
                    .collect()
            ),
        }
    );
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(skip))]
struct Sit {
    #[builder(setter(each(name = "foo")))]
    foos: Vec<i32>,
}

#[test]
fn extend_field_enabled() {
    let x = SitBuilder::default().foo(1).foo(2).build().unwrap();
    assert_eq!(x, Sit { foos: vec![1, 2] });
}
