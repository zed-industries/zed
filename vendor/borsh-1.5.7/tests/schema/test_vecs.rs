use crate::common_macro::schema_imports::*;
use alloc::collections::{VecDeque, LinkedList};

macro_rules! test_vec_like_collection_schema {
    [$test_name: ident, $type: ident] => [

        #[test]
        fn $test_name() {
            let actual_name = $type::<u64>::declaration();
            let mut actual_defs = schema_map!();
            $type::<u64>::add_definitions_recursively(&mut actual_defs);

            assert_eq!(format!("{}<u64>", stringify!($type)), actual_name);
            assert_eq!(
                schema_map! {
                    actual_name => Definition::Sequence {
                        length_width: Definition::DEFAULT_LENGTH_WIDTH,
                        length_range: Definition::DEFAULT_LENGTH_RANGE,
                        elements: "u64".to_string(),
                    },
                    "u64" => Definition::Primitive(8)
                },
                actual_defs
            );
        }
    ];
}

test_vec_like_collection_schema!(simple_vec, Vec);
test_vec_like_collection_schema!(vec_deque, VecDeque);
test_vec_like_collection_schema!(linked_list, LinkedList);

#[test]
fn nested_vec() {
    let actual_name = Vec::<Vec<u64>>::declaration();
    let mut actual_defs = schema_map!();
    Vec::<Vec<u64>>::add_definitions_recursively(&mut actual_defs);
    assert_eq!("Vec<Vec<u64>>", actual_name);
    assert_eq!(
        schema_map! {
            "Vec<u64>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "u64".to_string(),
            },
            "Vec<Vec<u64>>" => Definition::Sequence {
                length_width: Definition::DEFAULT_LENGTH_WIDTH,
                length_range: Definition::DEFAULT_LENGTH_RANGE,
                elements: "Vec<u64>".to_string(),
            },
            "u64" => Definition::Primitive(8)
        },
        actual_defs
    );
}

#[test]
fn slice_schema_container() {
    let schema = schema_container_of::<[i64]>();

    assert_eq!(
        schema,
        BorshSchemaContainer::new(
            "Vec<i64>".to_string(),
            schema_map! {
                "Vec<i64>" => Definition::Sequence {
                    length_width: Definition::DEFAULT_LENGTH_WIDTH,
                    length_range: Definition::DEFAULT_LENGTH_RANGE,
                    elements: "i64".to_string(),
                },
                "i64" => Definition::Primitive(8)

            }
        )
    )
}

#[test]
fn vec_schema_container() {
    let schema = schema_container_of::<Vec<i64>>();

    assert_eq!(
        schema,
        BorshSchemaContainer::new(
            "Vec<i64>".to_string(),
            schema_map! {
                "Vec<i64>" => Definition::Sequence {
                    length_width: Definition::DEFAULT_LENGTH_WIDTH,
                    length_range: Definition::DEFAULT_LENGTH_RANGE,
                    elements: "i64".to_string(),
                },
                "i64" => Definition::Primitive(8)

            }
        )
    )
}
