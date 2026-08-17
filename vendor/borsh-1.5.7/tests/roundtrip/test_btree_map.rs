use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    vec,
    vec::Vec,
};

use borsh::{from_slice, BorshSerialize};

macro_rules! btreeset_test_template [
    [$test_name: ident, $($key: expr),* ] => [
        #[allow(unused_mut)]
        #[allow(redundant_semicolons)]
        #[test]
        fn $test_name() {
            let mut set = BTreeSet::new();

            set_insert_deser_assert_macro!(set, data, $($key),*);

            let actual_set = from_slice::<BTreeSet<String>>(&data).unwrap();
            assert_eq!(set, actual_set);

        }

    ]

];

macro_rules! btreemap_test_template [
    [$test_name: ident, $($key: expr => $value: expr),* ] => [
        #[allow(unused_mut)]
        #[allow(redundant_semicolons)]
        #[test]
        fn $test_name() {
            let mut map = BTreeMap::new();

            map_insert_deser_assert_macro!(map, data, $($key => $value),*);

            let actual_map = from_slice::<BTreeMap<String, String>>(&data).unwrap();
            assert_eq!(map, actual_map);

        }

    ]

];

btreeset_test_template!(test_empty_btreeset,);

btreeset_test_template!(test_1_element_btreeset, "one".to_string());

btreeset_test_template!(
    test_2_element_btreeset,
    "one".to_string(),
    "different".to_string()
);

btreeset_test_template!(
    test_default_btreeset,
    "foo".to_string(),
    "many".to_string(),
    "various".to_string(),
    "different".to_string(),
    "keys".to_string(),
    "one".to_string()
);

btreemap_test_template!(test_default_btreemap,
    "foo".to_string() => "bar".to_string(),
    "one".to_string() => "two".to_string()
);

btreemap_test_template!(test_empty_btreemap,);
btreemap_test_template!(test_1_element_btreemap,
    "one".to_string() => "element".to_string()
);

btreemap_test_template!(test_8_element_btreemap,
    "one".to_string() => "element".to_string(),
    "key".to_string() => "powers".to_string(),
    "more".to_string() => "of".to_string(),
    "various".to_string() => "two".to_string(),
    "different".to_string() => "are".to_string(),
    "keys".to_string() => "always".to_string(),
    "where".to_string() => "unpredictable".to_string(),
    "nowhere".to_string() => "pile".to_string()
);

#[cfg(feature = "de_strict_order")]
const ERROR_WRONG_ORDER_OF_KEYS: &str = "keys were not serialized in ascending order";

set_wrong_order_test!(test_btreeset_deser_err_wrong_order, BTreeSet<String>);

map_wrong_order_test!(test_btreemap_deser_err_wrong_order, BTreeMap<String, String>);
