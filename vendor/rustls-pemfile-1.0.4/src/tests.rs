#[cfg(test)]
mod unit {
    fn check(data: &[u8]) -> Result<Vec<crate::Item>, std::io::Error> {
        let mut reader = std::io::BufReader::new(data);
        crate::read_all(&mut reader)
    }

    #[test]
    fn skips_leading_junk() {
        assert_eq!(
            check(
                b"junk\n\
                    -----BEGIN RSA PRIVATE KEY-----\n\
                    qw\n\
                    -----END RSA PRIVATE KEY-----\n"
            )
            .unwrap(),
            vec![crate::Item::RSAKey(vec![0xab])]
        );
    }

    #[test]
    fn skips_trailing_junk() {
        assert_eq!(
            check(
                b"-----BEGIN RSA PRIVATE KEY-----\n\
                    qw\n\
                    -----END RSA PRIVATE KEY-----\n\
                    junk"
            )
            .unwrap(),
            vec![crate::Item::RSAKey(vec![0xab])]
        );
    }

    #[test]
    fn skips_non_utf8_junk() {
        assert_eq!(
            check(
                b"\x00\x00\n\
                    -----BEGIN RSA PRIVATE KEY-----\n\
                    qw\n\
                    -----END RSA PRIVATE KEY-----\n
                    \x00\x00"
            )
            .unwrap(),
            vec![crate::Item::RSAKey(vec![0xab])]
        );
    }

    #[test]
    fn rejects_invalid_base64() {
        assert_eq!(
            format!(
                "{:?}",
                check(
                    b"-----BEGIN RSA PRIVATE KEY-----\n\
                            q=w\n\
                            -----END RSA PRIVATE KEY-----\n"
                )
            ),
            "Err(Custom { kind: InvalidData, error: InvalidByte(1, 61) })"
        );
    }

    #[test]
    fn rejects_unclosed_start_section() {
        assert_eq!(
            format!("{:?}",
                    check(b"-----BEGIN RSA PRIVATE KEY-----\n\
                            qw\n")),
            "Err(Custom { kind: InvalidData, error: \"section end \\\"-----END RSA PRIVATE KEY-----\\\" missing\" })"
        );
    }

    #[test]
    fn rejects_bad_start() {
        assert_eq!(
            format!("{:?}",
                    check(b"-----BEGIN RSA PRIVATE KEY----\n\
                            qw\n\
                            -----END RSA PRIVATE KEY-----\n")),
            "Err(Custom { kind: InvalidData, error: \"illegal section start: \\\"-----BEGIN RSA PRIVATE KEY----\\\\n\\\"\" })"
        );
    }

    #[test]
    fn skips_unrecognised_section() {
        assert_eq!(
            check(
                b"junk\n\
                    -----BEGIN BREAKFAST CLUB-----\n\
                    qw\n\
                    -----END BREAKFAST CLUB-----\n"
            )
            .unwrap(),
            vec![]
        );
    }
}
