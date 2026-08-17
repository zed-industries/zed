use crate::common_macro::schema_imports::*;
use alloc::{rc, sync};

fn common_map_i32() -> BTreeMap<String, Definition> {
    schema_map! {

        "i32" => Definition::Primitive(4)
    }
}

fn common_map_slice_i32() -> BTreeMap<String, Definition> {
    schema_map! {
        "Vec<i32>" => Definition::Sequence {
            length_width: Definition::DEFAULT_LENGTH_WIDTH,
            length_range: Definition::DEFAULT_LENGTH_RANGE,
            elements: "i32".to_string()
        },
        "i32" => Definition::Primitive(4)
    }
}

#[test]
fn test_rc() {
    assert_eq!("i32", <rc::Rc<i32> as BorshSchema>::declaration());

    let mut actual_defs = schema_map!();
    <rc::Rc<i32> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(common_map_i32(), actual_defs);
}

#[test]
fn test_slice_rc() {
    assert_eq!("Vec<i32>", <rc::Rc<[i32]> as BorshSchema>::declaration());
    let mut actual_defs = schema_map!();
    <rc::Rc<[i32]> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(common_map_slice_i32(), actual_defs);
}

#[test]
fn test_arc() {
    assert_eq!("i32", <sync::Arc<i32> as BorshSchema>::declaration());
    let mut actual_defs = schema_map!();
    <sync::Arc<i32> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(common_map_i32(), actual_defs);
}

#[test]
fn test_slice_arc() {
    assert_eq!("Vec<i32>", <sync::Arc<[i32]> as BorshSchema>::declaration());
    let mut actual_defs = schema_map!();
    <sync::Arc<[i32]> as BorshSchema>::add_definitions_recursively(&mut actual_defs);
    assert_eq!(common_map_slice_i32(), actual_defs);
}
