#![allow(unused_imports)]

#[cfg(feature = "experimental-derive")]
mod tests {
    use postcard::experimental::max_size::MaxSize;
    use postcard::to_slice;
    use serde::Serialize;

    #[test]
    fn test_struct_max_size() {
        #[derive(MaxSize)]
        struct Foo {
            _a: u16,
            _b: Option<u8>,
        }

        assert_eq!(Foo::POSTCARD_MAX_SIZE, 5);
    }

    #[test]
    fn test_enum_max_size() {
        #[allow(dead_code)]
        #[derive(MaxSize, Serialize)]
        enum Bar {
            A(u16),
            B(u8),
        }

        assert_eq!(Bar::POSTCARD_MAX_SIZE, 4);
        let mut buf = [0u8; 128];
        let used = to_slice(&Bar::A(0xFFFF), &mut buf).unwrap();
        assert!(
            used.len() <= Bar::POSTCARD_MAX_SIZE,
            "FAIL {} > {}",
            used.len(),
            Bar::POSTCARD_MAX_SIZE
        );

        #[derive(MaxSize)]
        enum Baz {}

        assert_eq!(Baz::POSTCARD_MAX_SIZE, 0);
    }

    #[test]
    fn test_ref() {
        #[allow(dead_code)]
        #[derive(MaxSize)]
        struct Foo {
            a: &'static u32,
        }
    }

    #[cfg(feature = "heapless")]
    #[test]
    fn test_vec_edge_cases() {
        #[track_caller]
        fn test_equals<const N: usize>(buf: &mut [u8]) {
            let mut v = heapless::Vec::<u8, N>::new();
            for _ in 0..N {
                v.push(0).unwrap();
            }

            let serialized = postcard::to_slice(&v, buf).unwrap();

            assert_eq!(heapless::Vec::<u8, N>::POSTCARD_MAX_SIZE, serialized.len());
        }

        let mut buf = [0; 16400];

        test_equals::<1>(&mut buf);
        test_equals::<2>(&mut buf);

        test_equals::<127>(&mut buf);
        test_equals::<128>(&mut buf);
        test_equals::<129>(&mut buf);

        test_equals::<16383>(&mut buf);
        test_equals::<16384>(&mut buf);
        test_equals::<16385>(&mut buf);
    }

    // #[cfg(feature = "experimental-derive")]
    // #[test]
    // fn test_union_max_size() {
    //     #[derive(postcard::MaxSize)]
    //     union Foo {
    //         a: u16,
    //         b: Option<u8>,
    //     }
    // }

    // #[cfg(feature = "experimental-derive")]
    // #[test]
    // fn test_not_implemented() {
    //     #[derive(postcard::MaxSize)]
    //     struct Foo {
    //         a: &'static str,
    //     }
    // }
}
