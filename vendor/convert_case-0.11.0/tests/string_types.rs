use convert_case::{Case, Casing};
use std::rc::Rc;
use std::sync::Arc;

macro_rules! test_casing_on_type {
    ($name:ident, $constructor:expr) => {
        #[test]
        fn $name() {
            let s = $constructor("rust_programming-language");
            assert_eq!(s.to_case(Case::Pascal), "RustProgrammingLanguage");
            assert_eq!(
                s.from_case(Case::Kebab).to_case(Case::Pascal),
                "Rust_programmingLanguage"
            );
        }
    };
}

test_casing_on_type!(string_type, String::from);
test_casing_on_type!(rc_str_type, Rc::<str>::from);
test_casing_on_type!(arc_str_type, Arc::<str>::from);

#[test]
fn str_type() {
    let s: &str = "rust_programming-language";
    assert_eq!(s.to_case(Case::Pascal), "RustProgrammingLanguage");
    assert_eq!(
        s.from_case(Case::Kebab).to_case(Case::Pascal),
        "Rust_programmingLanguage"
    );
}

#[test]
fn string_ref_type() {
    let s: String = String::from("rust_programming-language");
    assert_eq!((&s).to_case(Case::Pascal), "RustProgrammingLanguage");
    assert_eq!(
        s.from_case(Case::Kebab).to_case(Case::Pascal),
        "Rust_programmingLanguage"
    );
}
