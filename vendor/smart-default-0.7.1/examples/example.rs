use smart_default::SmartDefault;

#[derive(PartialEq, SmartDefault, Debug)]
#[allow(dead_code)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default(12)]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String,
    },
    Qux(i32),
}

fn main() {
    assert!(
        Foo::default()
            == Foo::Baz {
                a: 12,
                b: 0,
                c: Some(0),
                d: vec![1, 2, 3],
                e: "four".to_owned(),
            }
    );
}
