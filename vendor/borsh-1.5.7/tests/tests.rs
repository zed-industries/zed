#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod common_macro;

mod custom_reader {
    #[cfg(feature = "derive")]
    mod test_custom_reader;
}

/// this module doesn't contain runnable tests;
/// it's included into module tree to ensure derived code doesn't raise compilation
/// errors
#[rustfmt::skip]
#[cfg(feature = "derive")]
mod compile_derives {
    mod test_macro_namespace_collisions;
    #[allow(unused)]
    mod test_generic_structs;
    mod test_generic_enums;
    mod test_recursive_structs;

    #[cfg(feature = "unstable__schema")]
    mod schema {
        mod test_generic_enums;
    }
}

/// These are full roundtrip `BorshSerialize`/`BorshDeserialize` tests
#[rustfmt::skip]
mod roundtrip {
    mod test_strings;
    #[cfg(feature = "ascii")]
    mod test_ascii_strings;
    mod test_arrays;
    mod test_vecs;
    mod test_tuple;
    mod test_primitives;
    #[cfg(feature = "std")]
    mod test_ip_addr;
    mod test_nonzero_integers;
    mod test_range;
    // mod test_phantom_data; // NOTE: there's nothing corresponding to `schema::test_phantom_data`
    // mod test_option; // NOTE: there's nothing corresponding to `schema::test_option`
    // mod test_box; // NOTE: there's nothing corresponding to `schema::test_box`
    #[cfg(hash_collections)]
    mod test_hash_map;
    mod test_btree_map;
    mod test_cow;
    mod test_cells;
    #[cfg(feature = "rc")]
    mod test_rc;
    #[cfg(feature = "indexmap")]
    mod test_indexmap;

    #[cfg(feature = "derive")]
    mod requires_derive_category {
        // mod test_simple_structs; // NOTE: there's nothing corresponding to `schema::test_simple_structs`
        mod test_generic_structs;
        mod test_simple_enums;
        mod test_generic_enums;
        mod test_recursive_structs;
        mod test_recursive_enums;
        mod test_serde_with_third_party;
        mod test_enum_discriminants;
        #[cfg(feature = "bytes")]
        mod test_ultimate_many_features_combined;
        #[cfg(feature = "bson")]
        mod test_bson_object_ids;
    }
}

/// These are `BorshSchema` tests for various types
#[cfg(feature = "unstable__schema")]
#[rustfmt::skip]
mod schema {
    #[cfg(feature = "ascii")]
    mod test_ascii_strings;
    mod test_strings;
    mod test_arrays;
    mod test_vecs;
    mod test_tuple;
    mod test_primitives;
    #[cfg(feature = "std")]
    mod test_ip_addr;
    // mod test_nonzero_integers; // NOTE: there's nothing corresponding to `roundtrip::test_nonzero_integers`
    mod test_range;
    mod test_phantom_data;
    mod test_option;
    mod test_box;
    #[cfg(hash_collections)]
    mod test_hash_map;
    mod test_btree_map;
    mod test_cow;
    mod test_cells;
    #[cfg(feature = "rc")]
    mod test_rc;
    mod test_simple_structs;
    mod test_generic_structs;
    mod test_simple_enums;
    mod test_generic_enums;
    mod test_recursive_structs;
    mod test_recursive_enums;
    mod test_schema_with_third_party; // NOTE: this test corresponds to `roundtrip::test_serde_with_third_party`
    mod test_enum_discriminants;
    // mod test_ultimate_many_features_combined;  // NOTE: there's nothing corresponding to `roundtrip::test_ultimate_many_features_combined`
    // mod test_bson_object_ids; // NOTE: there's nothing corresponding to `roundtrip::test_bson_object_ids`
    mod schema_conflict {
        mod test_schema_conflict;
    }

    mod container_extension {
        mod test_schema_validate;
        mod test_max_size;
    }
}

mod deserialization_errors {
    #[cfg(feature = "ascii")]
    mod test_ascii_strings;
    mod test_cells;
    mod test_initial;
}

mod init_in_deserialize {
    #[cfg(feature = "derive")]
    mod test_init_in_deserialize;
}

mod zero_sized_types {
    #[cfg(feature = "derive")]
    mod test_zero_sized_types_forbidden;
}
