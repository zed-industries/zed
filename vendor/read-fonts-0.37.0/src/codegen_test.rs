//! A module used to test codegen.
//!
//! This imports a single codegen output; while modifying the codegen crate,
//! this file can be regenerated to check that changes compile, without needing
//! to rebuild everything.
//!
//! To rebuild this input and test it, run:
//!
//! $ cargo run --bin=codegen resources/test_plan.toml && cargo test

pub mod records {
    include!("../generated/generated_test_records.rs");
}

pub mod formats {
    include!("../generated/generated_test_formats.rs");
}

pub mod offsets_arrays {

    include!("../generated/generated_test_offsets_arrays.rs");

    #[cfg(test)]
    use font_test_data::bebuffer::BeBuffer;

    pub struct VarSizeDummy<'a> {
        #[allow(dead_code)]
        count: u16,
        pub bytes: &'a [u8],
    }

    impl VarSize for VarSizeDummy<'_> {
        type Size = u16;
    }

    impl<'a> FontRead<'a> for VarSizeDummy<'a> {
        fn read(data: FontData<'a>) -> Result<Self, ReadError> {
            let count: u16 = data.read_at(0)?;
            let bytes = data
                .as_bytes()
                .get(2..2 + (count as usize))
                .ok_or(ReadError::OutOfBounds)?;
            Ok(Self { count, bytes })
        }
    }

    #[test]
    fn array_offsets() {
        let builder = BeBuffer::new()
            .push(MajorMinor::VERSION_1_0)
            .push(12_u16) // offset to 0xdead
            .push(0u16) // nullable
            .push(2u16) // array len
            .push(12u16) // array offset
            .extend([0xdead_u16, 0xbeef]);

        let table = KindsOfOffsets::read(builder.data().into()).unwrap();
        assert_eq!(table.nonnullable().unwrap().value(), 0xdead);

        let array = table.array().unwrap();
        assert_eq!(array, &[0xdead, 0xbeef]);
    }

    #[test]
    fn var_len_array_empty() {
        let builder = BeBuffer::new().push(0u16).push(0xdeadbeef_u32);

        let table = VarLenHaver::read(builder.data().into()).unwrap();
        assert_eq!(table.other_field(), 0xdeadbeef);
    }

    #[test]
    fn var_len_array_some() {
        let builder = BeBuffer::new()
            .push(3u16)
            .push(0u16) // first item in array is empty
            .push(2u16)
            .extend([1u8, 1])
            .push(5u16)
            .extend([7u8, 7, 7, 7, 7])
            .push(0xdeadbeef_u32);

        let table = VarLenHaver::read(builder.data().into()).unwrap();
        let kids = table
            .var_len()
            .iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        assert!(kids[0].bytes.is_empty());
        assert_eq!(kids[1].bytes, &[1, 1]);
        assert_eq!(kids[2].bytes, &[7, 7, 7, 7, 7]);
        assert_eq!(table.other_field(), 0xdeadbeef)
    }

    #[test]
    #[cfg(feature = "experimental_traverse")]
    fn array_offsets_traverse() {
        let mut builder = BeBuffer::new()
            .push(MajorMinor::VERSION_1_1)
            .push(22_u16) // offset to [0xf00, 0xba4]
            .push(0u16) // nullable
            .push(2u16) // array len
            .push(26u16) // offset to [69, 70]
            .push(30u16) // record_array_offset
            .push(0u16) // versioned_nullable_record_array_offset
            .push(42u16) // versioned nonnullable offset
            .push(0u32); // versioned nullable offset
                         //
        let data_start = builder.len();
        assert_eq!(data_start, 22);
        builder = builder
            .extend([0xf00u16, 0xba4])
            .extend([69u16, 70])
            .push(3u16) // shmecord[0]
            .push(9u32)
            .push(5u16) // shmecord[1]
            .push(0xdead_beefu32)
            .extend([0xb01du16, 0xface]); // versioned nonnullable offset;

        let table = KindsOfOffsets::read(builder.data().into()).unwrap();
        // traversal should not crash
        let _ = format!("{table:?}");
        assert_eq!(
            table.versioned_nonnullable().unwrap().unwrap().value(),
            0xb01d
        );
    }
}

pub mod flags {
    include!("../generated/generated_test_flags.rs");

    #[test]
    fn basics() {
        let all = ValueFormat::all();
        let none = ValueFormat::empty();
        assert!(all.contains(ValueFormat::X_PLACEMENT));
        assert!(all.contains(ValueFormat::Y_PLACEMENT));
        assert!(!none.contains(ValueFormat::X_PLACEMENT));
        assert!(!none.contains(ValueFormat::Y_PLACEMENT));
        assert_eq!(none, ValueFormat::default());
    }

    #[test]
    fn formatting() {
        let all = ValueFormat::all();
        assert_eq!(format!("{all:?}"), "X_PLACEMENT | Y_PLACEMENT");
        let none = ValueFormat::empty();
        assert_eq!(format!("{none:?}"), "(empty)");
        let xplace = ValueFormat::X_PLACEMENT;
        assert_eq!(format!("{xplace:?}"), "X_PLACEMENT");
    }

    // not exactly a test, but this will fail to compile if these are missing
    #[test]
    fn impl_traits() {
        fn impl_check<T: Copy + std::hash::Hash + Eq + Ord>() {}
        impl_check::<ValueFormat>();
    }
}

pub mod enums {
    include!("../generated/generated_test_enum.rs");
}

pub mod count_all {
    use crate::FontData;

    include!("../generated/generated_test_count_all.rs");

    /// Test for count(..) with element sizes > 1
    #[test]
    fn element_size_greater_than_one_with_padding() {
        // Size of 13 ensures we have an extra padding byte
        let bytes = [0u8; 13];
        // Generated table has a 2 byte field above the array
        let remainder_len = bytes.len() - 2;
        let data = FontData::new(&bytes);
        // Trailing array with 16-bit elements
        assert!(remainder_len % 2 != 0);
        let count16 = CountAll16::read(data).unwrap();
        assert_eq!(count16.remainder().len(), remainder_len / 2);
        // Trailing array with 32-bit elements
        assert!(remainder_len % 4 != 0);
        let count32 = CountAll32::read(data).unwrap();
        assert_eq!(count32.remainder().len(), remainder_len / 4);
    }
}

pub mod conditions {
    #[cfg(test)]
    use font_test_data::bebuffer::BeBuffer;
    use font_types::MajorMinor;

    include!("../generated/generated_test_conditions.rs");

    #[test]
    fn majorminor_1() {
        let bytes = BeBuffer::new().push(MajorMinor::VERSION_1_0).push(0u16);
        let table = MajorMinorVersion::read(bytes.data().into()).unwrap();
        assert_eq!(table.always_present(), 0);
    }

    #[test]
    fn majorminor_1_1() {
        let bytes = BeBuffer::new().push(MajorMinor::VERSION_1_1).push(0u16);
        // shouldn't parse, we're missing a field
        assert!(MajorMinorVersion::read(bytes.data().into()).is_err());

        let bytes = BeBuffer::new()
            .push(MajorMinor::VERSION_1_1)
            .push(0u16)
            .push(1u16);
        let table = MajorMinorVersion::read(bytes.data().into()).unwrap();
        assert_eq!(table.if_11(), Some(1));
    }

    #[test]
    fn major_minor_2() {
        let bytes = BeBuffer::new().push(MajorMinor::VERSION_2_0).push(0u16);
        // shouldn't parse, we're missing a field
        assert!(MajorMinorVersion::read(bytes.data().into()).is_err());

        let bytes = BeBuffer::new()
            .push(MajorMinor::VERSION_2_0)
            .push(0u16)
            .push(2u32);
        let table = MajorMinorVersion::read(bytes.data().into()).unwrap();
        assert_eq!(table.if_11(), None);
        assert_eq!(table.if_20(), Some(2));
    }

    #[cfg(test)]
    fn make_flag_data(flags: GotFlags) -> BeBuffer {
        let mut buf = BeBuffer::new().push(42u16).push(flags);
        if flags.contains(GotFlags::FOO) {
            buf = buf.push(0xf00_u16);
        }
        if flags.contains(GotFlags::BAR) {
            buf = buf.push(0xba4_u16);
        }
        if flags.contains(GotFlags::FOO) || flags.contains(GotFlags::BAZ) {
            buf = buf.push(0xba2_u16);
        }
        buf
    }

    #[test]
    fn flags_none() {
        let data = make_flag_data(GotFlags::empty());
        let table = FlagDay::read(data.data().into()).unwrap();
        assert!(table.foo().is_none());
        assert!(table.bar().is_none());
    }

    #[test]
    fn flags_foo() {
        let data = make_flag_data(GotFlags::FOO);
        let table = FlagDay::read(data.data().into()).unwrap();
        assert_eq!(table.foo(), Some(0xf00));
        assert!(table.bar().is_none());
        assert_eq!(table.baz(), Some(0xba2));
    }

    #[test]
    fn flags_bar() {
        let data = make_flag_data(GotFlags::BAR);
        let table = FlagDay::read(data.data().into()).unwrap();
        assert!(table.foo().is_none());
        assert_eq!(table.bar(), Some(0xba4));
        assert!(table.baz().is_none());
    }

    #[test]
    fn flags_baz() {
        let data = make_flag_data(GotFlags::BAZ);
        let table = FlagDay::read(data.data().into()).unwrap();
        assert!(table.foo().is_none());
        assert!(table.bar().is_none());
        assert_eq!(table.baz(), Some(0xba2));
    }

    #[test]
    fn flags_foobar() {
        let data = make_flag_data(GotFlags::BAR | GotFlags::FOO);
        let table = FlagDay::read(data.data().into()).unwrap();
        assert_eq!(table.foo(), Some(0xf00));
        assert_eq!(table.bar(), Some(0xba4));
        assert_eq!(table.baz(), Some(0xba2));
    }

    #[test]
    fn fields_after_conditions_all_none() {
        let data = BeBuffer::new().push(GotFlags::empty()).extend([1u16, 2, 3]);

        let table = FieldsAfterConditionals::read(data.data().into()).unwrap();
        assert_eq!(table.always_here(), 1);
        assert_eq!(table.also_always_here(), 2);
        assert_eq!(table.and_me_too(), 3);
    }

    #[test]
    #[should_panic(expected = "OutOfBounds")]
    fn fields_after_conditions_wrong_len() {
        let data = BeBuffer::new().push(GotFlags::FOO).extend([1u16, 2, 3]);

        let _table = FieldsAfterConditionals::read(data.data().into()).unwrap();
    }

    #[test]
    fn fields_after_conditionals_one_present() {
        let data = BeBuffer::new()
            .push(GotFlags::BAR)
            .extend([1u16, 0xba4, 2, 3]);

        let table = FieldsAfterConditionals::read(data.data().into()).unwrap();
        assert_eq!(table.always_here(), 1);
        assert_eq!(table.bar(), Some(0xba4));
        assert_eq!(table.also_always_here(), 2);
        assert!(table.foo().is_none() && table.baz().is_none());
        assert_eq!(table.and_me_too(), 3);
    }

    #[test]
    fn fields_after_conditions_all_present() {
        let data = BeBuffer::new()
            .push(GotFlags::FOO | GotFlags::BAR | GotFlags::BAZ)
            .extend([0xf00u16, 1, 0xba4, 0xba2, 2, 3]);

        let table = FieldsAfterConditionals::read(data.data().into()).unwrap();
        assert_eq!(table.foo(), Some(0xf00));
        assert_eq!(table.always_here(), 1);
        assert_eq!(table.bar(), Some(0xba4));
        assert_eq!(table.baz(), Some(0xba2));
        assert_eq!(table.also_always_here(), 2);
        assert_eq!(table.and_me_too(), 3);
    }
}
