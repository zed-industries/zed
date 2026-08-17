#[allow(unused)]
macro_rules! set_insert_deser_assert_macro [

    [$set: ident, $data: ident, $($key: expr),*] => [
        $($set.insert($key));*
        ;

        let $data = borsh::to_vec(&$set).unwrap();
        #[cfg(feature = "std")]
        insta::assert_debug_snapshot!($data);
    ]
];

#[allow(unused)]
macro_rules! map_insert_deser_assert_macro [

    [$map: ident, $data: ident, $($key: expr => $value: expr),*] => [
        $($map.insert($key, $value));*
        ;

        let $data = borsh::to_vec(&$map).unwrap();
        #[cfg(feature = "std")]
        insta::assert_debug_snapshot!($data);
    ]
];

#[allow(unused)]
macro_rules! set_wrong_order_test [

    [$test_name: ident, $set_type: ty] => [

        #[test]
        fn $test_name() {
            let mut data = vec![];
            let arr_key = ["various".to_string(), "foo".to_string(), "many".to_string()];
            let len = arr_key.len() as u32;
            u32::serialize(&len, &mut data).expect("no error");

            for key in &arr_key {
                key.serialize(&mut data).expect("no error");
            }

            let result = from_slice::<$set_type>(&data);

            #[cfg(not(feature = "de_strict_order"))]
            {
                let result = result.unwrap();
                assert_eq!(result.len(), arr_key.len());
                for key in &arr_key {
                    assert!(result.contains(key));

                }

            }

            #[cfg(feature = "de_strict_order")]
            {
                assert!(result.is_err());

                assert_eq!(result.unwrap_err().to_string(), ERROR_WRONG_ORDER_OF_KEYS);
            }
        }
    ]
];

#[allow(unused)]
macro_rules! map_wrong_order_test [

    [$test_name: ident, $map_type: ty] => [

        #[test]
        fn $test_name() {
            let mut data = vec![];
            let arr_key = ["various".to_string(), "foo".to_string(), "many".to_string()];
            let arr_val = [
                "value".to_string(),
                "different".to_string(),
                "unexp".to_string(),
            ];
            let len = arr_key.len() as u32;
            u32::serialize(&len, &mut data).expect("no error");

            let entries = IntoIterator::into_iter(arr_key.clone())
                .zip(IntoIterator::into_iter(arr_val))
                .collect::<Vec<_>>();

            for (key, value) in entries.clone() {
                key.serialize(&mut data).expect("no error");
                value.serialize(&mut data).expect("no error");
            }

            let result = from_slice::<$map_type>(&data);

            #[cfg(not(feature = "de_strict_order"))]
            {
                let result = result.unwrap();
                assert_eq!(result.len(), arr_key.len());
                for (key, value) in entries {
                    assert_eq!(result.get(&key), Some(&value));
                }
            }

            #[cfg(feature = "de_strict_order")]
            {
                assert!(result.is_err());

                assert_eq!(result.unwrap_err().to_string(), ERROR_WRONG_ORDER_OF_KEYS);
            }
        }
    ]
];

#[allow(unused)]
macro_rules! schema_map(
    () => { BTreeMap::new() };
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = BTreeMap::new();
            $(
                m.insert($key.to_string(), $value);
            )+
            m
        }
     };
);

#[allow(unused)]
#[cfg(feature = "unstable__schema")]
pub mod schema_imports {
    extern crate alloc;
    pub use alloc::{
        boxed::Box,
        collections::BTreeMap,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };

    pub use borsh::schema::{
        add_definition, BorshSchemaContainer, Declaration, Definition, Fields,
        SchemaContainerValidateError, SchemaMaxSerializedSizeError,
    };
    pub use borsh::{schema_container_of, BorshSchema};
}
