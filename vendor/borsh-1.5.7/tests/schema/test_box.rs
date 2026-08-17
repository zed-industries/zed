use crate::common_macro::schema_imports::*;

#[test]
fn boxed_schema() {
    let boxed_declaration = Box::<str>::declaration();
    assert_eq!("String", boxed_declaration);
    let boxed_declaration = Box::<[u8]>::declaration();
    assert_eq!("Vec<u8>", boxed_declaration);
}
