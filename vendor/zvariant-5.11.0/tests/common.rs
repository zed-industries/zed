// Test through both generic and specific API (wrt byte order)
#[macro_export]
macro_rules! basic_type_test {
    ($endian:expr, $format:ident, $test_value:expr, $expected_len:expr, $expected_ty:ty, $align:literal) => {{
        // Lie that we're starting at byte 1 in the overall message to test padding
        let ctxt =
            zvariant::serialized::Context::new(zvariant::serialized::Format::$format, $endian, 1);
        let encoded = zvariant::to_bytes(ctxt, &$test_value).unwrap();
        let padding = zvariant::padding_for_n_bytes(1, $align);

        assert_eq!(
            encoded.len(),
            $expected_len + padding,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): ($expected_ty, _) = encoded.deserialize().unwrap();
        assert!(decoded == $test_value, "invalid decoding");
        assert!(parsed == encoded.len(), "invalid parsing");

        // Now encode w/o padding
        let ctxt =
            zvariant::serialized::Context::new(zvariant::serialized::Format::$format, $endian, 0);
        let encoded = zvariant::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );

        encoded
    }};
    ($endian:expr, $format:ident, $test_value:expr, $expected_len:expr, $expected_ty:ty, $align:literal, $kind:ident, $expected_value_len:expr) => {{
        let encoded = basic_type_test!(
            $endian,
            $format,
            $test_value,
            $expected_len,
            $expected_ty,
            $align
        );

        // As Value
        let v: zvariant::Value<'_> = $test_value.into();
        assert_eq!(
            v.value_signature(),
            <$expected_ty as zvariant::Basic>::SIGNATURE_STR
        );
        assert_eq!(v, zvariant::Value::$kind($test_value));
        value_test!($endian, $format, v, $expected_value_len);

        let v: $expected_ty = v.try_into().unwrap();
        assert_eq!(v, $test_value);

        encoded
    }};
}

#[macro_export]
macro_rules! value_test {
    ($endian:expr, $format:ident, $test_value:expr, $expected_len:expr) => {{
        let ctxt =
            zvariant::serialized::Context::new(zvariant::serialized::Format::$format, $endian, 0);
        let encoded = zvariant::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): (zvariant::Value<'_>, _) = encoded.deserialize().unwrap();
        assert!(decoded == $test_value, "invalid decoding");
        assert!(parsed == encoded.len(), "invalid parsing");

        encoded
    }};
}

#[cfg(unix)]
#[macro_export]
macro_rules! fd_value_test {
    ($endian:expr, $format:ident, $test_value:expr, $expected_len:expr, $align:literal, $expected_value_len:expr) => {{
        use std::os::fd::AsFd;

        // Lie that we're starting at byte 1 in the overall message to test padding
        let ctxt =
            zvariant::serialized::Context::new(zvariant::serialized::Format::$format, $endian, 1);
        let encoded = zvariant::to_bytes(ctxt, &$test_value).unwrap();
        let padding = zvariant::padding_for_n_bytes(1, $align);
        assert_eq!(
            encoded.len(),
            $expected_len + padding,
            "invalid encoding using `to_bytes`"
        );
        #[cfg(unix)]
        let (_, parsed): (zvariant::Fd<'_>, _) = encoded.deserialize().unwrap();
        assert!(
            parsed == encoded.len(),
            "invalid parsing using `from_slice`"
        );

        // Now encode w/o padding
        let ctxt =
            zvariant::serialized::Context::new(zvariant::serialized::Format::$format, $endian, 0);
        let encoded = zvariant::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );

        // As Value
        let v: zvariant::Value<'_> = $test_value.into();
        assert_eq!(v.value_signature(), zvariant::Fd::SIGNATURE_STR);
        assert_eq!(v, zvariant::Value::Fd($test_value));
        let encoded = zvariant::to_bytes(ctxt, &v).unwrap();
        assert_eq!(encoded.fds().len(), 1, "invalid encoding using `to_bytes`");
        assert_eq!(
            encoded.len(),
            $expected_value_len,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): (zvariant::Value<'_>, _) = encoded.deserialize().unwrap();
        assert_eq!(
            decoded,
            zvariant::Fd::from(encoded.fds()[0].as_fd()).into(),
            "invalid decoding using `from_slice`"
        );
        assert_eq!(parsed, encoded.len(), "invalid parsing using `from_slice`");

        let v: zvariant::Fd<'_> = v.try_into().unwrap();
        assert_eq!(v, $test_value);
    }};
}
