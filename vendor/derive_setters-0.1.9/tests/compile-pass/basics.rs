use derive_setters::Setters;

#[derive(Default, Setters, Debug, PartialEq, Eq)]
struct BasicStruct {
    #[setters(rename = "test")]
    a: u32,
    b: u32,
    c: u32,
}

fn main() {
    let s = BasicStruct::default().test(30).b(10).c(20);
    assert_eq!(s, BasicStruct { a: 30, b: 10, c: 20 });
}
