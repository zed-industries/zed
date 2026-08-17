#![cfg(feature = "serde")]
extern crate arrayvec;
extern crate serde_test;

mod array_vec {
    use arrayvec::ArrayVec;

    use serde_test::{Token, assert_tokens, assert_de_tokens_error};

    #[test]
    fn test_ser_de_empty() {
        let vec = ArrayVec::<u32, 0>::new();

        assert_tokens(&vec, &[
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
        ]);
    }


    #[test]
    fn test_ser_de() {
        let mut vec = ArrayVec::<u32, 3>::new();
        vec.push(20);
        vec.push(55);
        vec.push(123);

        assert_tokens(&vec, &[
            Token::Seq { len: Some(3) },
            Token::U32(20),
            Token::U32(55),
            Token::U32(123),
            Token::SeqEnd,
        ]);
    }

    #[test]
    fn test_de_too_large() {
        assert_de_tokens_error::<ArrayVec<u32, 2>>(&[
            Token::Seq { len: Some(3) },
            Token::U32(13),
            Token::U32(42),
            Token::U32(68),
        ], "invalid length 3, expected an array with no more than 2 items");
    }
}

mod array_string {
    use arrayvec::ArrayString;

    use serde_test::{Token, assert_tokens, assert_de_tokens_error};

    #[test]
    fn test_ser_de_empty() {
        let string = ArrayString::<0>::new();

        assert_tokens(&string, &[
            Token::Str(""),
        ]);
    }


    #[test]
    fn test_ser_de() {
        let string = ArrayString::<9>::from("1234 abcd")
            .expect("expected exact specified capacity to be enough");

        assert_tokens(&string, &[
            Token::Str("1234 abcd"),
        ]);
    }

    #[test]
    fn test_de_too_large() {
        assert_de_tokens_error::<ArrayString<2>>(&[
            Token::Str("afd")
        ], "invalid length 3, expected a string no more than 2 bytes long");
    }
}
