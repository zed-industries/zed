#[cfg(feature = "std")]
use core::hash::BuildHasher;

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::collections::{
    hash_map::{DefaultHasher, RandomState},
    HashMap, HashSet,
};

#[cfg(not(feature = "std"))]
use core::iter::IntoIterator;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use borsh::{from_slice, BorshSerialize};

macro_rules! hashset_test_template [
    [$test_name: ident, $($key: expr),* ] => [
        #[allow(unused_mut)]
        #[allow(redundant_semicolons)]
        #[test]
        fn $test_name() {
            let mut set = HashSet::new();

            set_insert_deser_assert_macro!(set, data, $($key),*);

            let actual_set = from_slice::<HashSet<String>>(&data).unwrap();
            assert_eq!(set, actual_set);

        }

    ]

];

macro_rules! hashmap_test_template [
    [$test_name: ident, $($key: expr => $value: expr),* ] => [
        #[allow(unused_mut)]
        #[allow(redundant_semicolons)]
        #[test]
        fn $test_name() {
            let mut map = HashMap::new();

            map_insert_deser_assert_macro!(map, data, $($key => $value),*);

            let actual_map = from_slice::<HashMap<String, String>>(&data).unwrap();
            assert_eq!(map, actual_map);

        }

    ]

];

#[derive(Default)]
#[cfg(feature = "std")]
struct NewHasher(RandomState);

#[cfg(feature = "std")]
impl BuildHasher for NewHasher {
    type Hasher = DefaultHasher;
    fn build_hasher(&self) -> DefaultHasher {
        self.0.build_hasher()
    }
}

#[cfg(feature = "std")]
macro_rules! generic_hashset_test_template [
    [$test_name: ident, $($key: expr),* ] => [
        #[allow(unused_mut)]
        #[allow(redundant_semicolons)]
        #[test]
        fn $test_name() {
            let mut set = HashSet::with_hasher(NewHasher::default());

            set_insert_deser_assert_macro!(set, data, $($key),*);

            let actual_set = from_slice::<HashSet<String, NewHasher>>(&data).unwrap();
            assert_eq!(set, actual_set);

        }

    ]

];

#[cfg(feature = "std")]
macro_rules! generic_hashmap_test_template [
    [$test_name: ident, $($key: expr => $value: expr),* ] => [
        #[allow(unused_mut)]
        #[allow(redundant_semicolons)]
        #[test]
        fn $test_name() {
            let mut map = HashMap::with_hasher(NewHasher::default());

            map_insert_deser_assert_macro!(map, data, $($key => $value),*);

            let actual_map = from_slice::<HashMap<String, String, NewHasher>>(&data).unwrap();
            assert_eq!(map, actual_map);

        }

    ]

];

hashset_test_template!(test_empty_hashset,);

hashset_test_template!(test_1_element_hashset, "one".to_string());

hashset_test_template!(
    test_2_element_hashset,
    "one".to_string(),
    "different".to_string()
);

hashset_test_template!(
    test_default_hashset,
    "foo".to_string(),
    "many".to_string(),
    "various".to_string(),
    "different".to_string(),
    "keys".to_string(),
    "one".to_string()
);

#[cfg(feature = "std")]
generic_hashset_test_template!(test_empty_generic_hashset,);

#[cfg(feature = "std")]
generic_hashset_test_template!(test_1_element_generic_hashset, "one".to_string());

#[cfg(feature = "std")]
generic_hashset_test_template!(
    test_2_element_generic_hashset,
    "one".to_string(),
    "different".to_string()
);

#[cfg(feature = "std")]
generic_hashset_test_template!(
    test_generic_hashset,
    "foo".to_string(),
    "many".to_string(),
    "various".to_string(),
    "different".to_string(),
    "keys".to_string(),
    "one".to_string()
);

hashmap_test_template!(test_default_hashmap,
    "foo".to_string() => "bar".to_string(),
    "one".to_string() => "two".to_string()
);

hashmap_test_template!(test_empty_hashmap,);
hashmap_test_template!(test_1_element_hashmap,
    "one".to_string() => "element".to_string()
);

hashmap_test_template!(test_8_element_hashmap,
    "one".to_string() => "element".to_string(),
    "key".to_string() => "powers".to_string(),
    "more".to_string() => "of".to_string(),
    "various".to_string() => "two".to_string(),
    "different".to_string() => "are".to_string(),
    "keys".to_string() => "always".to_string(),
    "where".to_string() => "unpredictable".to_string(),
    "nowhere".to_string() => "pile".to_string()
);

#[cfg(feature = "std")]
generic_hashmap_test_template!(test_generic_hash_hashmap,
    "foo".to_string() => "bar".to_string(),
    "one".to_string() => "two".to_string()
);

#[cfg(feature = "std")]
generic_hashmap_test_template!(test_empty_generic_hashmap,);
#[cfg(feature = "std")]
generic_hashmap_test_template!(test_1_element_generic_hashmap,
    "one".to_string() => "element".to_string()
);

#[cfg(feature = "std")]
generic_hashmap_test_template!(test_8_element_generic_hashmap,
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

set_wrong_order_test!(test_hashset_deser_err_wrong_order, HashSet<String>);
#[cfg(feature = "std")]
set_wrong_order_test!(test_generic_hashset_deser_err_wrong_order, HashSet<String, NewHasher>);

map_wrong_order_test!(test_hashmap_deser_err_wrong_order, HashMap<String, String>);

#[cfg(feature = "std")]
map_wrong_order_test!(test_generic_hashmap_deser_err_wrong_order, HashMap<String, String, NewHasher>);
